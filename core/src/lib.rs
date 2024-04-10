// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

#![allow(dead_code, unused_variables)]

use std::{collections::HashMap, fmt, marker::PhantomData};
pub use url::Url;

pub mod builders;

#[derive(Clone, Debug)]
pub struct DiagnosticsOptions {
    pub application_id: Option<String>,
    pub logging: bool,
    pub logging_content: bool,
    pub telemetry: bool,
}

impl Default for DiagnosticsOptions {
    fn default() -> Self {
        Self {
            application_id: None,
            logging: true,
            logging_content: false,
            telemetry: true,
        }
    }
}

#[derive(Debug)]
pub struct Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("error")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        todo!()
    }
}

pub type Headers = HashMap<String, String>;

pub struct Pipeline {}

impl Pipeline {
    pub fn new(endpoint: impl AsRef<str>) -> Self {
        todo!()
    }
}

pub struct RequestContent<T> {
    _phantom: PhantomData<T>,
}

impl<T> From<T> for RequestContent<T> {
    fn from(value: T) -> Self {
        todo!()
    }
}

impl<T> std::str::FromStr for RequestContent<T> {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}

pub struct Response<T> {
    status: StatusCode,
    headers: Headers,
    _phantom: PhantomData<T>,
}

impl<T> Response<T> {
    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub async fn try_into(self) -> Result<T> {
        todo!()
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Default)]
enum RetryMode {
    #[default]
    Exponential,
    Fixed,
    Custom,
    None,
}

impl fmt::Debug for RetryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exponential => f.write_str("exponential"),
            Self::Fixed => f.write_str("fixed"),
            Self::Custom => f.write_str("custom"),
            Self::None => f.write_str("none"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RetryOptions {
    mode: RetryMode,
}

impl RetryOptions {
    pub fn exponential() -> Self {
        Self {
            mode: RetryMode::Exponential,
        }
    }

    pub fn fixed() -> Self {
        Self {
            mode: RetryMode::Fixed,
        }
    }

    pub fn custom() -> Self {
        Self {
            mode: RetryMode::Custom,
        }
    }

    pub fn none() -> Self {
        Self {
            mode: RetryMode::None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StatusCode(u16);

impl From<u16> for StatusCode {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

pub trait TokenCredential: fmt::Debug {}
