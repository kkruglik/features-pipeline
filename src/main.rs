use chrono::Local;
use features_pipeline::pipeline::labels::LabelsPipeline;
use linfa::Dataset;
use ndarray::s;
use polars::prelude::*;
use serde_yaml::to_string;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use features_pipeline::config::entry::EntrypointConfig;
use features_pipeline::pipeline::features::FeaturePipeline;

fn create_run_folder() -> Result<PathBuf, std::io::Error> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let run_dir = PathBuf::from("data/output").join(timestamp);

    fs::create_dir_all(&run_dir)?;

    Ok(run_dir)
}

fn main() -> Result<(), Box<dyn Error>> {
    let entrypoint_config = EntrypointConfig::from_yaml("config/entrypoint_adult.yaml")?;

    let features_pipeline = FeaturePipeline::from_yaml(&entrypoint_config.features)?;

    let labels_pipeline = LabelsPipeline::from_yaml(&entrypoint_config.labels)?;

    let run_dir = create_run_folder()?;

    println!(
        "Loaded {} features steps from config/features.yaml\n",
        features_pipeline.steps.len()
    );

    println!(
        "Loaded {} labels steps from config/features.yaml\n",
        labels_pipeline.steps.len()
    );

    for (i, feature) in features_pipeline.steps.iter().enumerate() {
        println!("Feature {}: {:?}", i + 1, feature);
    }

    let yaml_output = to_string(&features_pipeline)?;
    println!("{}", yaml_output);

    let csv_file = std::fs::File::open(&entrypoint_config.data)?;
    let df = CsvReader::new(csv_file).finish()?;

    println!("Data before transform: {:?}", df.shape());

    let mut features = features_pipeline.apply(&df)?;
    let mut labels = labels_pipeline.apply(&df)?;

    println!("Number of features: {:?}", features.shape());

    let features_filename = File::create_new(run_dir.join("features.csv"))?;
    let labels_filename = File::create_new(run_dir.join("labels.csv"))?;

    CsvWriter::new(&features_filename)
        .include_header(true)
        .with_separator(b';')
        .finish(&mut features)?;

    CsvWriter::new(&labels_filename)
        .include_header(true)
        .with_separator(b';')
        .finish(&mut labels)?;

    let features_array = features.to_ndarray::<Float64Type>(IndexOrder::Fortran)?;
    let labels_array = labels.to_ndarray::<Float64Type>(IndexOrder::Fortran)?;

    Ok(())
}
