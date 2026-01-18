use std::{
    error::Error,
    fmt::{self},
};

use polars::error::PolarsError;

#[derive(Debug)]
pub enum PipelineStepError {
    ColumnNotFound {
        found: String,
        available: Vec<String>,
    },
    EmptyGroupby {
        feature_name: String,
    },
    DataframeError(PolarsError),
    IoError(std::io::Error),
    SerdeError(serde_yaml::Error),
}

impl fmt::Display for PipelineStepError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineStepError::ColumnNotFound { found, available } => {
                write!(
                    f,
                    "Column '{}' not found. Available [{:?}]",
                    found, available
                )
            }
            PipelineStepError::EmptyGroupby { feature_name } => {
                write!(
                    f,
                    "Feature '{}' dont have any groupby columns",
                    feature_name
                )
            }
            PipelineStepError::DataframeError(err) => write!(f, "Polars error: {}", err),
            PipelineStepError::IoError(e) => write!(f, "IO error: {}", e),
            PipelineStepError::SerdeError(e) => write!(f, "Serde error: {}", e),
        }
    }
}

impl Error for PipelineStepError {}

impl From<PolarsError> for PipelineStepError {
    fn from(value: PolarsError) -> Self {
        PipelineStepError::DataframeError(value)
    }
}

impl From<std::io::Error> for PipelineStepError {
    fn from(value: std::io::Error) -> Self {
        PipelineStepError::IoError(value)
    }
}

impl From<serde_yaml::Error> for PipelineStepError {
    fn from(value: serde_yaml::Error) -> Self {
        PipelineStepError::SerdeError(value)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound { path: String, kind: String },
    IoError(std::io::Error),
    ParseError { path: String, error: String },
    SerdeError(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound { path, kind } => {
                write!(f, "{} file not found: {}", kind, path)
            }
            ConfigError::ParseError { path, error } => {
                write!(f, "Failed to parse {}: {}", path, error)
            }
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
            ConfigError::SerdeError(e) => write!(f, "Serde error: {}", e),
        }
    }
}

impl Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::IoError(value)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(value: serde_yaml::Error) -> Self {
        ConfigError::SerdeError(value)
    }
}
