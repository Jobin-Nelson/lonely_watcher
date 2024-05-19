use derive_more::From;
use std::path::PathBuf;

// pub type Error = Box<dyn std::error::Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    #[from]
    BackupFailed{
        source_file: PathBuf,
        destination_file: Option<PathBuf>,
    },

    #[from]
    Io(std::io::Error),
}

// begin region: -- Custom
impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}


impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}
// end region: -- Custom


// begin region: -- Error boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
