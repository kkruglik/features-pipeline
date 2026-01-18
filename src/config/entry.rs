use std::{fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;

use crate::errors::ConfigError;

#[derive(Serialize, Deserialize, Debug)]
pub struct EntrypointConfig {
    pub data: String,
    pub features: String,
    pub labels: String,
}

impl EntrypointConfig {
    pub fn from_yaml(filepath: &str) -> Result<Self, ConfigError> {
        let config_yaml = File::open(filepath)?;
        let reader = BufReader::new(config_yaml);
        let config: EntrypointConfig = from_reader(reader)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if !Path::new(&self.data).exists() {
            return Err(ConfigError::FileNotFound {
                path: self.data.clone(),
                kind: "data".to_string(),
            });
        }

        if !Path::new(&self.features).exists() {
            return Err(ConfigError::FileNotFound {
                path: self.features.clone(),
                kind: "features".to_string(),
            });
        }

        if !Path::new(&self.labels).exists() {
            return Err(ConfigError::FileNotFound {
                path: self.labels.clone(),
                kind: "labels".to_string(),
            });
        }

        Ok(())
    }
}
