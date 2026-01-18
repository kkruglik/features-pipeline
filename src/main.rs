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
    let mut df = CsvReader::new(csv_file).finish()?;

    println!("Data before transform: {:?}", df.shape());

    df = features_pipeline.apply(&df)?;
    df = labels_pipeline.apply(&df)?;

    println!("Data after transform: {:?}", df.shape());

    let output_filename = File::create_new(run_dir.join("output.csv"))?;

    println!(
        "Saving processed features to {}",
        run_dir.join("output.csv").display()
    );

    CsvWriter::new(&output_filename)
        .include_header(true)
        .with_separator(b';')
        .finish(&mut df)?;

    let features_array = df.to_ndarray::<Float64Type>(IndexOrder::Fortran)?;

    let (features, targets) = (
        features_array.slice(s![.., 0..9]).to_owned(),
        features_array.column(10).to_owned(),
    );

    Ok(())
}
