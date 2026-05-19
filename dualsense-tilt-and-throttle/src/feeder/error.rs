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
