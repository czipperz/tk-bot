#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    IO(std::io::Error),
    EnvironmentError(std::env::VarError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::IO(e)
    }
}

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Error {
        Error::EnvironmentError(e)
    }
}
