// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use crate::BlobClient;
use azure_core::{Pipeline, Result, RetryOptions, TokenCredential, Url};
use std::{marker::PhantomData, sync::Arc};

pub enum Set {}
pub enum Unset {}

/// Helps builds a [`BlobClient`].
pub struct BlobClientBuilder<C, A> {
    endpoint: Option<Url>,
    credential: Option<Arc<dyn TokenCredential>>,
    connection_string: Option<String>,
    sas_token: Option<String>,
    api_version: Option<String>,
    retry: Option<RetryOptions>,
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
            retry: None,
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
            retry: self.retry,
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
            retry: self.retry,
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
            retry: self.retry,
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
            retry: self.retry,
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
            retry: self.retry,
            _phantom: PhantomData,
        }
    }

    /// Sets [`RetryOptions`] for connections made by the [`BlobClient`].
    pub fn with_retry(self, options: RetryOptions) -> Self {
        BlobClientBuilder {
            endpoint: self.endpoint,
            credential: self.credential,
            connection_string: self.connection_string,
            sas_token: self.sas_token,
            api_version: self.api_version,
            retry: Some(options),
            _phantom: PhantomData,
        }
    }

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
