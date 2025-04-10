//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

use std::sync::atomic::{AtomicBool, Ordering};
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
pub struct Client {
    connected: AtomicBool,
}

impl Client {
    /// Open a connection to server at `url`.
    pub async fn connect(&self, _url: &str) -> Result<(), Error> {
        debug!("Client::connect");
        let duration = Duration::from_secs(1);
        sleep(duration).await;
        self.connected.store(true, Ordering::Relaxed);
        Ok(())
    }
    /// Disconnect from server, destroying the client.
    pub async fn disconnect(&self) -> Result<(), Error> {
        debug!("Client::disconnect");
        self.connected.store(false, Ordering::Relaxed);
        Ok(())
    }
    /// Tell whether client is connected or not.
    pub fn connected(&self) -> bool {
        let connected = self.connected.load(Ordering::Relaxed);
        debug!("Client::connected: {}", connected);
        connected
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
