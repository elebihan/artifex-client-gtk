//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use crate::config::APP_ID;
pub use artifex_rpc::artifex_client::ArtifexClient;
use gtk::{
    gio::{self, prelude::*},
    glib,
};
use std::sync::OnceLock;
use thiserror::Error;
use tokio::runtime::Runtime;
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity, Uri};

/// Errors reported when operating a `ArtifexClient`.
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Invalid URI error: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tonic transport error: {0}")]
    Tonic(#[from] tonic::transport::Error),
    #[error("Unsupported scheme: {0}")]
    UnsupportedScheme(String),
    #[error("URI error: {0}")]
    Uri(String),
}

fn read_pem_from_file(uri: &str) -> Result<String, Error> {
    let uri = glib::Uri::parse(uri, glib::UriFlags::NONE).map_err(|e| Error::Uri(e.to_string()))?;
    match uri.scheme().as_str() {
        "file" => {
            let pem = std::fs::read_to_string(uri.path())?;
            Ok(pem)
        }
        s => Err(Error::UnsupportedScheme(s.to_string())),
    }
}

fn create_tls_config() -> Result<ClientTlsConfig, Error> {
    let settings = gio::Settings::new(&format!("{APP_ID}.connection"));
    let root_cert = read_pem_from_file(&settings.string("auth-root-cert-uri"))?;
    let client_cert = read_pem_from_file(&settings.string("auth-client-cert-uri"))?;
    let client_key = read_pem_from_file(&settings.string("auth-client-key-uri"))?;
    let identity = Identity::from_pem(client_cert, client_key);
    let config = ClientTlsConfig::new()
        .domain_name("artifex-server")
        .ca_certificate(Certificate::from_pem(root_cert))
        .identity(identity);
    Ok(config)
}

pub(crate) fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

/// Create a client for server at `url`.
pub(crate) async fn connect(url: &str) -> Result<ArtifexClient<Channel>, Error> {
    let uri = Uri::from_maybe_shared(url.to_string())?;
    let secured = match uri.scheme_str() {
        Some("http") => false,
        Some("https") => true,
        Some(s) => return Err(Error::UnsupportedScheme(s.to_string())),
        None => return Err(Error::UnsupportedScheme("none given".to_string())),
    };
    let channel = Channel::builder(uri);
    let channel = if secured {
        let tls = create_tls_config()?;
        channel.tls_config(tls)?
    } else {
        channel
    };
    let conn = channel.connect().await?;
    Ok(ArtifexClient::new(conn))
}
