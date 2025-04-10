use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    UnableToCloneClient,
    NoRecordsToProcess,
    InvalidEnvironmentTarget(String),
    #[from]
    Reqwest(reqwest::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnableToCloneClient => write!(fmt, "Unable to process URL"),
            Self::NoRecordsToProcess => write!(fmt, "No records to process"),
            Self::InvalidEnvironmentTarget(target) => {
                write!(fmt, "Invalid target `{}` specified", target)
            }
            Self::Reqwest(e) => write!(fmt, "Unable to process request. {}", e),
        }
    }
}

impl core::error::Error for Error {}
