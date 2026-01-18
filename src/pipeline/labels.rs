use crate::errors::PipelineStepError;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "function")]
pub enum LabelsConfig {
    #[serde(rename = "existing_target")]
    ExistingTarget {
        column: String,
        name: String,
        encode: bool,
        drop_original: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelsPipeline {
    pub steps: Vec<LabelsConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl LabelsPipeline {
    pub fn from_yaml(filepath: &str) -> Result<Self, PipelineStepError> {
        let config_yaml = File::open(filepath)?;
        let reader = BufReader::new(config_yaml);
        let config: LabelsPipeline = from_reader(reader)?;
        Ok(config)
    }

    pub fn apply(&self, data: &DataFrame) -> Result<DataFrame, PipelineStepError> {
        let mut result = data.clone();
        for step in &self.steps {
            result = step.apply(&result)?;
        }
        Ok(result)
    }
}

impl LabelsConfig {
    pub fn apply(&self, data: &DataFrame) -> Result<DataFrame, PipelineStepError> {
        match self {
            Self::ExistingTarget {
                column,
                name,
                encode,
                drop_original,
            } => {
                if !self.is_column_exists(data, column) {
                    return Err(PipelineStepError::ColumnNotFound {
                        found: column.clone(),
                        available: data
                            .get_column_names()
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    });
                }

                if !encode {
                    let mut result = data
                        .clone()
                        .lazy()
                        .with_columns([col(column).alias(name)])
                        .collect()?;
                    if *drop_original {
                        result = result.drop(column)?;
                    }
                    return Ok(result);
                }

                let unique: Vec<String> = data
                    .column(column)?
                    .unique()?
                    .sort(Default::default())?
                    .str()?
                    .into_iter()
                    .flatten()
                    .map(|s| s.to_string())
                    .collect();

                let mut expr = lit(-1i32);
                for (i, val) in unique.iter().enumerate() {
                    expr = when(col(column).eq(lit(val.as_str())))
                        .then(lit(i as i32))
                        .otherwise(expr);
                }

                let mut result = data
                    .clone()
                    .lazy()
                    .with_columns([expr.alias(name)])
                    .collect()?;

                if *drop_original {
                    result = result.drop(column)?;
                }

                Ok(result)
            }
        }
    }

    fn is_column_exists(&self, data: &DataFrame, col_name: &str) -> bool {
        data.get_column_names().iter().any(|col| *col == col_name)
    }
}
