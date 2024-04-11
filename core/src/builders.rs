// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use crate::DiagnosticsOptions;

use super::RetryOptions;

/// Adds setters for all core client options.
/// Your struct must declare a field of type [`ClientBuilder`].
/// Pass the name of that field into this macro.
#[macro_export]
macro_rules! client_options {
    // NOTE: This is perhaps a naive design meant for demonstration only.
    ($field:ident) => {
        /// Sets the [`azure_core::DiagnosticsOptions`] for this client builder.
        pub fn with_diagnostics(mut self, options: $crate::DiagnosticsOptions) -> Self {
            self.$field.diagnostics = options;
            self
        }

        /// Sets the [`azure_core::RetryOptions`] for this client builder.
        pub fn with_retry(mut self, options: $crate::RetryOptions) -> Self {
            self.$field.retry = options;
            self
        }
    };
}

#[derive(Clone, Debug, Default)]
pub struct ClientBuilder {
    pub diagnostics: DiagnosticsOptions,
    pub retry: RetryOptions,
}
