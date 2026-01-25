use std::{fs::File, io::BufReader};

use polars::prelude::*;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use tracing::instrument;

use crate::errors::PipelineStepError;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "function")]
pub enum FeatureConfig {
    #[serde(rename = "mean")]
    Mean {
        column: String,
        group_by: Vec<String>,
        name: String,
    },

    #[serde(rename = "sum")]
    Sum {
        column: String,
        group_by: Vec<String>,
        name: String,
    },

    #[serde(rename = "max")]
    Max {
        column: String,
        group_by: Vec<String>,
        name: String,
    },

    #[serde(rename = "min")]
    Min {
        column: String,
        group_by: Vec<String>,
        name: String,
    },

    #[serde(rename = "threshold")]
    Threshold {
        column: String,
        threshold: f64,
        comparator: String,
        name: String,
    },

    #[serde(rename = "ratio")]
    Ratio {
        numerator: String,
        denominator: String,
        name: String,
    },

    #[serde(rename = "count_distinct")]
    CountDistinct {
        column: String,
        group_by: Vec<String>,
        name: String,
    },

    #[serde(rename = "count")]
    Count {
        column: String,
        group_by: Vec<String>,
        name: String,
    },

    #[serde(rename = "ohe")]
    Ohe {
        columns: Vec<String>,
        drop_first: bool,
        drop_nulls: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FeaturePipeline {
    pub steps: Vec<FeatureConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl FeaturePipeline {
    pub fn from_yaml(filepath: &str) -> Result<Self, PipelineStepError> {
        let config_yaml = File::open(filepath)?;
        let reader = BufReader::new(config_yaml);
        let config: FeaturePipeline = from_reader(reader)?;
        Ok(config)
    }

    pub fn apply(&self, data: &DataFrame) -> Result<DataFrame, PipelineStepError> {
        let mut result = data.clone();
        let mut output_columns: Vec<String> = vec![];
        for step in &self.steps {
            result = step.apply_feature(&result)?;
            match step {
                FeatureConfig::Ohe { .. } => {
                    let ohe_cols: Vec<String> = result
                        .get_column_names()
                        .iter()
                        .filter(|col| col.contains("__ohe__"))
                        .map(|col| col.to_string())
                        .collect();
                    output_columns.extend(ohe_cols);
                }
                _ => {
                    if let Some(name) = step.name() {
                        output_columns.push(format!("feature_{}", name));
                    }
                }
            }
        }

        output_columns.sort();
        output_columns.dedup();

        result = result.select(output_columns)?;

        Ok(result)
    }
}

impl FeatureConfig {
    pub fn apply_feature(&self, data: &DataFrame) -> Result<DataFrame, PipelineStepError> {
        match self {
            Self::Mean {
                column,
                group_by,
                name,
            } if !group_by.is_empty() => {
                let feature_col_name = format!("feature_{name}");
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

                for i in group_by.iter() {
                    if !self.is_column_exists(data, i) {
                        return Err(PipelineStepError::ColumnNotFound {
                            found: i.clone(),
                            available: data
                                .get_column_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        });
                    }
                }

                let groupby_cols: Vec<Expr> = group_by.iter().map(col).collect();

                Ok(data
                    .clone()
                    .lazy()
                    .with_columns([col(column)
                        .mean()
                        .over(groupby_cols)
                        .alias(feature_col_name)])
                    .collect()?)
            }
            Self::Max {
                column,
                group_by,
                name,
            } if !group_by.is_empty() => {
                let feature_col_name = format!("feature_{name}");
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

                for i in group_by.iter() {
                    if !self.is_column_exists(data, i) {
                        return Err(PipelineStepError::ColumnNotFound {
                            found: i.clone(),
                            available: data
                                .get_column_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        });
                    }
                }

                let groupby_cols: Vec<Expr> = group_by.iter().map(col).collect();

                Ok(data
                    .clone()
                    .lazy()
                    .with_columns([col(column).max().over(groupby_cols).alias(feature_col_name)])
                    .collect()?)
            }
            Self::Sum {
                column,
                group_by,
                name,
            } if !group_by.is_empty() => {
                let feature_col_name = format!("feature_{name}");
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

                for i in group_by.iter() {
                    if !self.is_column_exists(data, i) {
                        return Err(PipelineStepError::ColumnNotFound {
                            found: i.clone(),
                            available: data
                                .get_column_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        });
                    }
                }

                let groupby_cols: Vec<Expr> = group_by.iter().map(col).collect();

                Ok(data
                    .clone()
                    .lazy()
                    .with_columns([col(column).sum().over(groupby_cols).alias(feature_col_name)])
                    .collect()?)
            }
            Self::Min {
                column,
                group_by,
                name,
            } if !group_by.is_empty() => {
                let feature_col_name = format!("feature_{name}");
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

                for i in group_by.iter() {
                    if !self.is_column_exists(data, i) {
                        return Err(PipelineStepError::ColumnNotFound {
                            found: i.clone(),
                            available: data
                                .get_column_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        });
                    }
                }

                let groupby_cols: Vec<Expr> = group_by.iter().map(col).collect();

                Ok(data
                    .clone()
                    .lazy()
                    .with_columns([col(column).min().over(groupby_cols).alias(feature_col_name)])
                    .collect()?)
            }
            Self::Count {
                column,
                group_by,
                name,
            } if !group_by.is_empty() => {
                let feature_col_name = format!("feature_{name}");
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

                for i in group_by.iter() {
                    if !self.is_column_exists(data, i) {
                        return Err(PipelineStepError::ColumnNotFound {
                            found: i.clone(),
                            available: data
                                .get_column_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        });
                    }
                }

                let groupby_cols: Vec<Expr> = group_by.iter().map(col).collect();

                Ok(data
                    .clone()
                    .lazy()
                    .with_columns([col(column)
                        .count()
                        .over(groupby_cols)
                        .alias(feature_col_name)])
                    .collect()?)
            }

            Self::Ratio {
                numerator,
                denominator,
                name,
            } => {
                let feature_col_name = format!("feature_{name}");
                if !self.is_column_exists(data, numerator) {
                    return Err(PipelineStepError::ColumnNotFound {
                        found: numerator.clone(),
                        available: data
                            .get_column_names()
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    });
                }

                if !self.is_column_exists(data, denominator) {
                    return Err(PipelineStepError::ColumnNotFound {
                        found: denominator.clone(),
                        available: data
                            .get_column_names()
                            .iter()
                            .map(|s| s.to_string())
                            .collect(),
                    });
                }

                Ok(data
                    .clone()
                    .lazy()
                    .with_columns([(col(numerator) / col(denominator)).alias(feature_col_name)])
                    .collect()?)
            }

            Self::CountDistinct {
                column,
                group_by,
                name,
            } if !group_by.is_empty() => {
                let feature_col_name = format!("feature_{name}");
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

                for i in group_by.iter() {
                    if !self.is_column_exists(data, i) {
                        return Err(PipelineStepError::ColumnNotFound {
                            found: i.clone(),
                            available: data
                                .get_column_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        });
                    }
                }

                let groupby_cols: Vec<Expr> = group_by.iter().map(col).collect();

                Ok(data
                    .clone()
                    .lazy()
                    .with_columns([col(column)
                        .n_unique()
                        .over(groupby_cols)
                        .alias(feature_col_name)])
                    .collect()?)
            }

            Self::Threshold {
                column,
                threshold,
                comparator,
                name,
            } => {
                let feature_col_name = format!("feature_{name}");
                match comparator.as_ref() {
                    "gt" => Ok(data
                        .clone()
                        .lazy()
                        .with_columns([col(column).gt(*threshold).alias(feature_col_name)])
                        .collect()?),
                    "lt" => Ok(data
                        .clone()
                        .lazy()
                        .with_columns([col(column).lt(*threshold).alias(feature_col_name)])
                        .collect()?),
                    _ => Ok(data.clone()),
                }
            }

            Self::Ohe {
                columns,
                drop_first,
                drop_nulls,
            } => {
                for col in columns.iter() {
                    if !self.is_column_exists(data, col) {
                        return Err(PipelineStepError::ColumnNotFound {
                            found: col.clone(),
                            available: data
                                .get_column_names()
                                .iter()
                                .map(|s| s.to_string())
                                .collect(),
                        });
                    }
                }
                let col_strs: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();
                Ok(data.clone().columns_to_dummies(
                    col_strs,
                    Some("__ohe__"),
                    *drop_first,
                    *drop_nulls,
                )?)
            }
            _ => Ok(data.clone()),
        }
    }

    fn is_column_exists(&self, data: &DataFrame, col_name: &str) -> bool {
        data.get_column_names().iter().any(|col| *col == col_name)
    }

    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Mean { name, .. }
            | Self::Sum { name, .. }
            | Self::Max { name, .. }
            | Self::Min { name, .. }
            | Self::Count { name, .. }
            | Self::CountDistinct { name, .. }
            | Self::Ratio { name, .. }
            | Self::Threshold { name, .. } => Some(name),
            Self::Ohe { .. } => None,
        }
    }
}
