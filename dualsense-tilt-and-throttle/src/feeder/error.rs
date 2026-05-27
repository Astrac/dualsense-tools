// use std::fmt::Display;
//
// #[derive(Debug)]
// pub enum Error {
//     #[cfg(target_os = "windows")]
//     VJoyError(vjoy::Error),
// }
//
// #[cfg(target_os = "windows")]
// impl From<vjoy::Error> for Error {
//     fn from(value: vjoy::Error) -> Self {
//         Error::VJoyError(value)
//     }
// }
//
// impl Display for Error {
//     fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         #[cfg(target_os = "windows")]
//         {
//             let Self::VJoyError(error) = self;
//             _f.write_str(format!("Error in the feeder: {error}").as_str())?;
//         }
//
//         Ok(())
//     }
// }
//
// impl std::error::Error for Error {}
