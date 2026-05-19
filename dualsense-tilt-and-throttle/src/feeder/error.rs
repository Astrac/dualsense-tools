use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    #[cfg(target_os = "windows")]
    VJoyError(vjoy::Error),
}

#[cfg(target_os = "windows")]
impl From<vjoy::Error> for Error {
    fn from(value: vjoy::Error) -> Self {
        Error::VJoyError(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: improve error
        f.write_str("Error in the feeder")
    }
}

impl std::error::Error for Error {}
