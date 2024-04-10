// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

use azure_core::TokenCredential;

#[derive(Clone, Debug, Default)]
pub struct DefaultAzureCredential {}

impl TokenCredential for DefaultAzureCredential {}
