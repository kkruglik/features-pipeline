use std::{
    error::Error,
    fmt::{self},
};

use polars::error::PolarsError;

#[derive(Debug)]
pub enum FeatureError {
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

impl fmt::Display for FeatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FeatureError::ColumnNotFound { found, available } => {
                write!(
                    f,
                    "Column '{}' not found. Available [{:?}]",
                    found, available
                )
            }
            FeatureError::EmptyGroupby { feature_name } => {
                write!(
                    f,
                    "Feature '{}' dont have any groupby columns",
                    feature_name
                )
            }
            FeatureError::DataframeError(err) => write!(f, "Polars error: {}", err),
            FeatureError::IoError(e) => write!(f, "IO error: {}", e),
            FeatureError::SerdeError(e) => write!(f, "Serde error: {}", e),
        }
    }
}

impl Error for FeatureError {}

impl From<PolarsError> for FeatureError {
    fn from(value: PolarsError) -> Self {
        FeatureError::DataframeError(value)
    }
}

impl From<std::io::Error> for FeatureError {
    fn from(value: std::io::Error) -> Self {
        FeatureError::IoError(value)
    }
}

impl From<serde_yaml::Error> for FeatureError {
    fn from(value: serde_yaml::Error) -> Self {
        FeatureError::SerdeError(value)
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
