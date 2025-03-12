use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    UnableToCloneClient,
    NoRecordsToProcess,
    #[from]
    Reqwest(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(fmt, "{}", &self)
    }
}

impl core::error::Error for Error {}
