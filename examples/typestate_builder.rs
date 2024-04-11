// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use azure_core::{DiagnosticsOptions, RetryOptions};
use azure_identity::DefaultAzureCredential;
use azure_typestate_example::{models::Model, BlobClient};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BlobClient::builder()
        .with_endpoint("https://api.contoso.com/endpoint")
        .with_credential(Arc::new(DefaultAzureCredential::default()))
        .with_diagnostics(DiagnosticsOptions {
            logging_content: true,
            ..Default::default()
        })
        .with_retry(RetryOptions::none())
        .build()?;

    let model = Model {
        name: "foo".to_string(),
        value: "bar".to_string(),
    };

    let _ = client.invoke(model.into()).await?;

    Ok(())
}
