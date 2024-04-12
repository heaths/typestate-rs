// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![doc = include_str!("../README.md")]

use azure_core::{Pipeline, RequestContent, Response, Result};

pub mod builders;
use builders::{BlobClientBuilder, Unset};

pub mod models;
use models::Model;

/// Storage client to access blobs.
///
/// See [`BlobClient::builder`] to construct a `BlobClient`.
pub struct BlobClient {
    #[allow(dead_code)]
    pub(crate) pipeline: Pipeline,
}

impl BlobClient {
    /// Gets a [`BlobClientBuilder`] to construct a `BlobClient`.
    ///
    /// # Examples
    ///
    /// Construct a client using an endpoint.
    ///
    /// ``` no_run
    /// use azure_identity::DefaultAzureCredential;
    /// use azure_typestate_example::BlobClient;
    /// use std::sync::Arc;
    ///
    /// # fn main() -> azure_core::Result<()> {
    /// let client = BlobClient::builder()
    ///     .with_endpoint("https://mycontainer.blobs.azure.net")
    ///     .with_credential(Arc::new(DefaultAzureCredential::default()))
    ///     .build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> BlobClientBuilder<Unset, Unset> {
        BlobClientBuilder::new()
    }

    /// Invokes the client method.
    ///
    /// # Examples
    ///
    /// Call the client method.
    ///
    /// ``` no_run
    /// use azure_typestate_example::{BlobClient, models::Model};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> azure_core::Result<()> {
    /// # let client: BlobClient = BlobClient::builder()
    /// #   .with_connection_string("")
    /// #   .build()?;
    /// let model = Model {
    ///     name: "name".to_string(),
    ///     value: "value".to_string(),
    /// };
    /// let model = client.invoke(model.into()).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(unused_variables)]
    pub async fn invoke(&self, body: RequestContent<Model>) -> Result<Response<Model>> {
        todo!()
    }
}
