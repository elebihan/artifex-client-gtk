//
// This file is part of artifex-client-gtk
//
// SPDX-FileCopyrightText: Copyright © 2025 Eric Le Bihan
//
// SPDX-License-Identifier: MIT
//

//! Secrets management.

use secret_service::EncryptionType;
use std::collections::HashMap;
use thiserror::Error;

/// Errors reported when managing secrets.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Secret service error: {0}")]
    SecretService(#[from] secret_service::Error),
}

/// Store password for encrypted key specified by URI.
pub fn store_password(uri: &str, password: &str) -> Result<(), Error> {
    use secret_service::blocking::SecretService;
    let service = SecretService::connect(EncryptionType::Dh)?;
    let collection = service.get_default_collection()?;
    let props = HashMap::from([("auth-client-key-uri", uri)]);
    collection.create_item(
        "Artifex client authentication",
        props,
        password.as_bytes(),
        false,
        "text/plain",
    )?;
    Ok(())
}

/// Retrieve password for encrypted key specified by URI.
pub async fn retrieve_password(uri: &str) -> Result<String, Error> {
    use secret_service::SecretService;
    let service = SecretService::connect(EncryptionType::Dh).await?;
    let props = HashMap::from([("auth-client-key-uri", uri)]);
    let items = service.search_items(props).await?;
    if let Some(item) = items.unlocked.first() {
        let value = item.get_secret().await?;
        Ok(String::from_utf8(value).expect("Password should be a valid UTF-8 string"))
    } else {
        Err(Error::NotFound(uri.to_string()))
    }
}
