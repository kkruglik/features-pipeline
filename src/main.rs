use chrono::Local;
use polars::prelude::*;
use serde_yaml::to_string;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;

use features_pipeline::config::{EntrypointConfig, PipelineSteps};

fn create_run_folder() -> Result<PathBuf, std::io::Error> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let run_dir = PathBuf::from("data/output").join(timestamp);

    fs::create_dir_all(&run_dir)?;

    Ok(run_dir)
}

fn main() -> Result<(), Box<dyn Error>> {
    let entrypoint_config = EntrypointConfig::load_from_yaml("config/entrypoint.yaml")?;

    let features_pipeline = PipelineSteps::load_from_yaml(&entrypoint_config.features)?;

    let run_dir = create_run_folder()?;

    println!(
        "Loaded {} features from config/features.yaml\n",
        features_pipeline.steps.len()
    );

    for (i, feature) in features_pipeline.steps.iter().enumerate() {
        println!("Feature {}: {:?}", i + 1, feature);
    }

    let yaml_output = to_string(&features_pipeline)?;
    println!("{}", yaml_output);

    let csv_file = std::fs::File::open(&entrypoint_config.data)?;
    let mut df = CsvReader::new(csv_file).finish()?;

    println!("Data before transform: {:?}", df.shape());

    df = features_pipeline.apply(&df)?;

    println!("Data after transform: {:?}", df.shape());

    let output_filename = File::create_new(run_dir.join("output.csv"))?;

    CsvWriter::new(&output_filename)
        .include_header(true)
        .with_separator(b';')
        .finish(&mut df)?;

    let features_array = df.to_ndarray::<Float64Type>(IndexOrder::Fortran)?;

    Ok(())
}
