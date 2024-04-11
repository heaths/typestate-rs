// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

//! Models with owned fields.

use serde::{Deserialize, Serialize};

/// Example model to pass to or receive from [`crate::BlobClient::invoke`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Model {
    pub name: String,
    pub value: String,
}
