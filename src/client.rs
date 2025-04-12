//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use std::sync::OnceLock;
use thiserror::Error;
use tokio::runtime::Runtime;
use tokio::time::{sleep, Duration};
use tracing::debug;

pub(crate) fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| Runtime::new().expect("Setting up tokio runtime needs to succeed."))
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Already connected")]
    AlreadyConnected,
}

#[derive(Debug, Default)]
pub struct Client {}

impl Client {
    /// Create a connection to server at `url`.
    pub async fn connect(_url: &str) -> Result<Self, Error> {
        debug!("Client::connect");
        let duration = Duration::from_secs(1);
        sleep(duration).await;
        Ok(Self {})
    }
    /// Inspect
    pub async fn inspect(&self) -> Result<InspectReply, Error> {
        debug!("Client::inspect");
        let duration = Duration::from_secs(1);
        sleep(duration).await;
        Ok(InspectReply {
            kernel_version: "6.12.15-200.fc41.x86_64".to_string(),
            system_uptime: "1month 10h 7m 42s".to_string(),
        })
    }
}

#[derive(Debug, Default)]
pub struct InspectReply {
    pub kernel_version: String,
    pub system_uptime: String,
}
