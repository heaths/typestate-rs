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
    endpoint: Option<String>,
    credential: Option<Arc<dyn TokenCredential>>,
    connection_string: Option<String>,
    sas_token: Option<String>,
    api_version: Option<String>,
    options: ClientBuilder,
    _phantom: PhantomData<(C, A)>,
}

impl<C, A> BlobClientBuilder<C, A> {
    /// Stable workaround for [RFC 2528: Type-changing struct update syntax](https://github.com/rust-lang/rfcs/blob/master/text/2528-type-changing-struct-update-syntax.md).
    ///
    /// Tracking issue for stabilization: <https://github.com/rust-lang/rust/issues/86555>.
    #[inline(always)]
    fn into<C2, A2>(self) -> BlobClientBuilder<C2, A2> {
        BlobClientBuilder {
            endpoint: self.endpoint,
            credential: self.credential,
            connection_string: self.connection_string,
            sas_token: self.sas_token,
            api_version: self.api_version,
            options: self.options,
            _phantom: PhantomData,
        }
    }
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

/// The initial `BlobClientBuilder`. You must set an endpoint or connection string.
impl BlobClientBuilder<Unset, Unset> {
    /// Creates a new `BlobClientBuilder`.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Sets the endpoint to call.
    pub fn with_endpoint(self, endpoint: impl Into<String>) -> BlobClientBuilder<Set, Unset> {
        BlobClientBuilder {
            endpoint: Some(endpoint.into()),
            ..self.into()
        }
    }

    /// Sets the connection string containing the endpoint and credentials.
    pub fn with_connection_string(
        self,
        connect_string: impl Into<String>,
    ) -> BlobClientBuilder<Set, Set> {
        BlobClientBuilder {
            connection_string: Some(connect_string.into()),
            ..self.into()
        }
    }
}

/// A `BlobClientBuilder` with an endpoint set, but requiring either a credential or SAS token.
impl BlobClientBuilder<Set, Unset> {
    /// Sets the credential to use for the endpoint.
    pub fn with_credential(
        self,
        credential: Arc<dyn TokenCredential>,
    ) -> BlobClientBuilder<Set, Set> {
        BlobClientBuilder {
            credential: Some(credential.clone()),
            ..self.into()
        }
    }

    /// Sets the SAS token to use for the endpoint.
    pub fn with_sas_token(self, sas_token: impl Into<String>) -> BlobClientBuilder<Set, Set> {
        BlobClientBuilder {
            sas_token: Some(sas_token.into()),
            ..self.into()
        }
    }
}

/// A `BlobClientBuilder` with all required state set and ready for additional options,
/// or to call [`BlobClientBuilder::build()`] to construct the final [`BlobClient`].
impl BlobClientBuilder<Set, Set> {
    /// Overrides the `api-version` query parameter to the endpoint.
    pub fn with_api_version(self, api_version: impl Into<String>) -> Self {
        BlobClientBuilder {
            api_version: Some(api_version.into()),
            ..self.into()
        }
    }

    // Add all core client options to this builder.
    azure_core::client_options!(options);

    /// Builds the [`BlobClient`] ready to use.
    pub fn build(self) -> Result<BlobClient> {
        let endpoint = self.endpoint.map_or_else(
            || {
                ConnectionString::parse(self.connection_string.as_ref().unwrap())
                    .map(|v| v.endpoint.to_string())
            },
            Ok,
        )?;
        let endpoint = Url::parse(endpoint.as_str())?;

        Ok(BlobClient {
            // TODO: Obviously this would need to do something with the options.
            pipeline: Pipeline::new(endpoint),
        })
    }
}

#[derive(Clone, Debug)]
struct ConnectionString<'a> {
    endpoint: &'a str,
    _account_key: &'a str,
}

impl<'a> ConnectionString<'a> {
    fn parse(connection_string: &'a str) -> Result<Self> {
        // TODO: Obviously this would need to parse the connection string.
        Ok(Self {
            endpoint: connection_string,
            _account_key: "",
        })
    }
}
