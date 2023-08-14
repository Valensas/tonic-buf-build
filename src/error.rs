use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct TonicBufBuildError {
    pub message: String,
    pub cause: Option<Box<dyn Error>>,
}

impl TonicBufBuildError {
    pub(crate) fn new(message: &str, cause: Box<dyn Error>) -> Self {
        Self {
            message: message.into(),
            cause: Some(cause),
        }
    }

    pub(crate) fn new_without_cause(message: &str) -> Self {
        Self {
            message: message.into(),
            cause: None,
        }
    }
}

impl Display for TonicBufBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.cause {
            Some(cause) => f.write_str(&format!("{}: {}", self.message, cause)),
            None => f.write_str(&self.message),
        }
    }
}

impl Error for TonicBufBuildError {}
