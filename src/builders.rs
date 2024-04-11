// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

//! Client builders.

use crate::BlobClient;
use azure_core::{builders::ClientBuilder, Pipeline, Result, TokenCredential, Url};
use std::{marker::PhantomData, sync::Arc};

#[doc(hidden)]
#[derive(Debug)]
pub enum Set {}

#[doc(hidden)]
#[derive(Debug)]
pub enum Unset {}

/// Helps builds a [`crate::BlobClient`].
#[derive(Clone, Debug)]
pub struct BlobClientBuilder<C, A> {
    endpoint: Option<Url>,
    credential: Option<Arc<dyn TokenCredential>>,
    connection_string: Option<String>,
    sas_token: Option<String>,
    api_version: Option<String>,
    options: ClientBuilder,
    _phantom: PhantomData<(C, A)>,
}

impl<C, A> Default for BlobClientBuilder<C, A> {
    fn default() -> Self {
        Self {
            endpoint: None,
            credential: None,
            connection_string: None,
            sas_token: None,
            api_version: None,
            options: ClientBuilder::default(),
            _phantom: PhantomData,
        }
    }
}

impl BlobClientBuilder<Unset, Unset> {
    /// Creates a new `BlobClientBuilder`.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Sets the endpoint to call.
    pub fn with_endpoint(self, endpoint: impl AsRef<str>) -> Result<BlobClientBuilder<Set, Unset>> {
        let url = Url::parse(endpoint.as_ref())?;
        Ok(BlobClientBuilder {
            endpoint: Some(url),
            credential: self.credential,
            connection_string: self.connection_string,
            sas_token: self.sas_token,
            api_version: self.api_version,
            options: self.options,
            _phantom: PhantomData,
        })
    }

    /// Sets the connection string containing the endpoint and credentials.
    pub fn with_connection_string(
        self,
        connect_string: impl Into<String>,
    ) -> BlobClientBuilder<Set, Set> {
        BlobClientBuilder {
            endpoint: self.endpoint,
            credential: self.credential,
            connection_string: Some(connect_string.into()),
            sas_token: self.sas_token,
            api_version: self.api_version,
            options: self.options,
            _phantom: PhantomData,
        }
    }
}

impl BlobClientBuilder<Set, Unset> {
    /// Sets the credential to use for the endpoint.
    pub fn with_credential(
        self,
        credential: Arc<dyn TokenCredential>,
    ) -> BlobClientBuilder<Set, Set> {
        BlobClientBuilder {
            endpoint: self.endpoint,
            credential: Some(credential.clone()),
            connection_string: self.connection_string,
            sas_token: self.sas_token,
            api_version: self.api_version,
            options: self.options,
            _phantom: PhantomData,
        }
    }

    /// Sets the SAS token to use for the endpoint.
    pub fn with_sas_token(self, sas_token: impl Into<String>) -> BlobClientBuilder<Set, Set> {
        BlobClientBuilder {
            endpoint: self.endpoint,
            credential: self.credential,
            connection_string: self.connection_string,
            sas_token: Some(sas_token.into()),
            api_version: self.api_version,
            options: self.options,
            _phantom: PhantomData,
        }
    }
}

impl BlobClientBuilder<Set, Set> {
    /// Overrides the `api-version` query parameter to the endpoint.
    pub fn with_api_version(self, api_version: impl Into<String>) -> Self {
        BlobClientBuilder {
            endpoint: self.endpoint,
            credential: self.credential,
            connection_string: self.connection_string,
            sas_token: self.sas_token,
            api_version: Some(api_version.into()),
            options: self.options,
            _phantom: PhantomData,
        }
    }

    // Add all core client options to this builder.
    azure_core::client_options!(options);

    /// Builds the [`BlobClient`] ready to use.
    pub fn build(self) -> BlobClient {
        let endpoint = self
            .endpoint
            // unwrap() okay since we'll have either an endpoint or connection_string.
            .map_or_else(|| self.connection_string.unwrap(), |v| v.to_string());

        BlobClient {
            // TODO: Obviously this would need to do something with the options.
            pipeline: Pipeline::new(endpoint),
        }
    }
}
