//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use crate::{config::APP_ID, secrets};
pub use artifex_client_cli::{client, tls};
pub use artifex_rpc::artifex_client::ArtifexClient;
use gtk::gio::{self, prelude::*};
use std::sync::OnceLock;
use thiserror::Error;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

/// Errors reported when operating a `ArtifexClient`.
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Client error: {0}")]
    Client(#[from] client::Error),
    #[error("Invalid URI error: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TLS configuration error: {0}")]
    Tls(#[from] tls::Error),
    #[error("Tonic transport error: {0}")]
    Tonic(#[from] tonic::transport::Error),
}

pub(crate) fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

struct Settings {
    root_cert_uri: String,
    client_cert_uri: String,
    client_key_uri: String,
    server_alt_name: String,
}

fn get_settings() -> Settings {
    let settings = gio::Settings::new(&format!("{APP_ID}.connection"));
    let root_cert_uri = settings.string("auth-root-cert-uri").to_string();
    let client_cert_uri = settings.string("auth-client-cert-uri").to_string();
    let client_key_uri = settings.string("auth-client-key-uri").to_string();
    let server_alt_name = settings.string("auth-server-alt-name").to_string();
    Settings {
        root_cert_uri,
        client_cert_uri,
        client_key_uri,
        server_alt_name,
    }
}

pub(crate) async fn create_tls_config() -> Result<tls::Config, Error> {
    let settings = get_settings();
    let server_alt_name = if settings.server_alt_name.is_empty() {
        None
    } else {
        Some(settings.server_alt_name.to_string())
    };
    let client_password = secrets::retrieve_password(&settings.client_key_uri)
        .await
        .ok();
    let config = tls::Config {
        root_cert: settings.root_cert_uri,
        client_cert: settings.client_cert_uri,
        client_key: settings.client_key_uri,
        client_password,
        server_alt_name,
    };
    Ok(config)
}

/// Create a client for server at `url`.
pub(crate) async fn connect(url: &str) -> Result<ArtifexClient<Channel>, Error> {
    let config = create_tls_config().await?;
    let client = client::ClientBuilder::with_tls_config(config)
        .connect(url)
        .await?;
    Ok(client)
}
