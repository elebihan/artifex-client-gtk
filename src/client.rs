//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

pub use artifex_rpc::artifex_client::ArtifexClient;
use std::sync::OnceLock;
use thiserror::Error;
use tokio::runtime::Runtime;
use tonic::transport::Channel;

/// Errors reported when operating a `ArtifexClient`.
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tonic transport error: {0}")]
    Tonic(#[from] tonic::transport::Error),
    #[error("URI error: {0}")]
    Uri(#[from] http::uri::InvalidUri),
}

pub(crate) fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

/// Create a client for server at `url`.
pub(crate) async fn connect(url: &str) -> Result<ArtifexClient<Channel>, Error> {
    let channel = Channel::from_shared(url.to_string())?;
    let conn = channel.connect().await?;
    Ok(ArtifexClient::new(conn))
}
