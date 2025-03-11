use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Custom(String),
    #[from]
    Reqwest(reqwest::Error),
}

impl Error {
    pub fn custom(val: impl core::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(fmt, "{}", &self)
    }
}

impl core::error::Error for Error {}
