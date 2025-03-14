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
        match self {
            Self::UnableToCloneClient => write!(fmt, "Unable to clone web client"),
            Self::NoRecordsToProcess => write!(fmt, "No records to process"),
            Self::Reqwest(e) => write!(fmt, "Unable to process request. {}", e.to_string()),
        }
    }
}

impl core::error::Error for Error {}
