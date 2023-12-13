//! The `error` module defines a library-specific `Error` alias [`enum@Error`], and an alias for Result
//! [`SheetResult`] using the `Error` alias.
use thiserror::Error;

/// The `SheetResult` type is an alias for `Result` using the library-defined [`enum@Error`].
pub type SheetResult<T> = Result<T, Error>;

/// The `Error` enum is a library-specific error conversion.
#[derive(Error, Debug)]
pub enum Error {
    /// A `ParseError` indicates an error occurred during parsing.
    #[error("Parse error.")]
    ParseError,
    /// The `DeserializeError` converts errors from the `serde` crate.
    #[error("Deserialize error.")]
    DeserializeError(#[from] serde::de::value::Error),
    /// The `UserBuildError` indicates an error occurred using a builder pattern.
    #[error("Value not provided for {value:?}.")]
    UserBuildError {
        /// The `value` field returns messages on missing parameters in the builder function.
        value: Vec<String>,
    },
    /// The `Io` variant represents error conversions from [`std::io::Error`].
    #[error("Input/output error from std.")]
    Io(#[from] std::io::Error),
    /// The `EnvError` variant represents error conversions from [`std::env::VarError`].
    #[error("Could not read environmental variables from .env.")]
    EnvError(#[from] std::env::VarError),
    /// The `AuthError` variant indicates an error occurred during the authorization process.
    #[error("Authorization failed.")]
    AuthError,
    /// The `FileNameError` variant indicates a malformed file name, from [`std::ffi::OsString`].
    #[error("Bad file name {0:?}.")]
    FileNameError(std::ffi::OsString),
    /// The `IntError` variant represents error conversions from [`std::num::ParseIntError`],
    /// indicating a failure to parse an integer from a string.
    #[error("Could not parse integer from string.")]
    IntError(#[from] std::num::ParseIntError),
    /// The `UnknownError` is a catch-all error variant for library operations.
    #[error("Unexpected error.")]
    UnknownError,
    /// The `UnknownError` is a catch-all error variant for library operations.
    #[error("Error parsing address.")]
    AddressError(#[from] address::prelude::AddressError),
}
