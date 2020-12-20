use thiserror::Error;

use std::io;
use std::path::PathBuf;

#[derive(Debug)]
enum ErrorOld {
    ReadFile(PathBuf, io::Error),
    Command(PathBuf, io::Error),
    Reqwest(reqwest::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ReadFile(path, _) => write!(fmt, "Failed to read {}", path.display()),
            Self::Command(path, _) => write!(fmt, "Failed to execute {}", path.display()),
            Self::Reqwest(err) => write!(fmt, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadFile(_, source) | Self::Command(_, source) => Some(source),
            Self::Reqwest(err) => err.source(),
        }
    }
}

// ↓これに書き換えられる。

#[derive(Debug, Error)]
enum Error {
    #[error("Failed to read {}", .0.display())]
    ReadFile(PathBuf, #[source] io::Error),

    #[error("Failed to execute {}", .0.display())]
    Command(PathBuf, #[source] io::Error),

    #[error("{}", .0)]
    Reqwest(#[from] reqwest::Error),
}
