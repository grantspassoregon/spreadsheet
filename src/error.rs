use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parse error.")]
    ParseError,
    #[error("Deserialize error.")]
    DeserializeError(#[from] serde::de::value::Error),
    #[error("Value not provided for {value:?}.")]
    UserBuildError { value: Vec<String> },
    #[error("Input/output error from std.")]
    Io(#[from] std::io::Error),
    #[error("Could not read environmental variables from .env.")]
    EnvError(#[from] std::env::VarError),
    #[error("Authorization failed.")]
    AuthError,
    #[error("Bad file name {0:?}.")]
    FileNameError(std::ffi::OsString),
    #[error("Could not parse integer from string.")]
    IntError(#[from] std::num::ParseIntError),
}
