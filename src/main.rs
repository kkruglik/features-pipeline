use chrono::Local;
use features_pipeline::pipeline::labels::LabelsPipeline;
use linfa::DatasetBase;
use linfa::metrics::ToConfusionMatrix;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use polars::prelude::*;
use serde_yaml::to_string;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber;

use features_pipeline::config::entry::EntrypointConfig;
use features_pipeline::pipeline::features::FeaturePipeline;

fn create_run_folder() -> Result<PathBuf, std::io::Error> {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let run_dir = PathBuf::from("data/output").join(timestamp);

    fs::create_dir_all(&run_dir)?;

    Ok(run_dir)
}

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    // tracing_subscriber::fmt().json().init();

    let entrypoint_config = EntrypointConfig::from_yaml("config/entrypoint_adult.yaml")?;

    let features_pipeline = FeaturePipeline::from_yaml(&entrypoint_config.features)?;

    let labels_pipeline = LabelsPipeline::from_yaml(&entrypoint_config.labels)?;

    let run_dir = create_run_folder()?;

    info!("Created run folder: {}", run_dir.display());

    info!(
        "Loaded {} labels steps from config/features.yaml\n",
        labels_pipeline.steps.len()
    );

    for (i, feature) in features_pipeline.steps.iter().enumerate() {
        info!("Feature {}: {:?}", i + 1, feature);
    }

    let csv_file = std::fs::File::open(&entrypoint_config.data)?;
    let df = CsvReader::new(csv_file).finish()?;

    info!("Raw data shape: {:?}", df.shape());
    info!("Columns: {:?}", df.get_column_names());

    let mut features = features_pipeline.apply(&df)?;

    info!("Features before fill_null: {:?}", features.shape());

    features = features.fill_null(FillNullStrategy::Zero)?;

    info!("Features after fill_null: {:?}", features.shape());
    info!("Feature columns: {:?}", features.get_column_names());

    let mut labels = labels_pipeline.apply(&df)?;

    info!("Labels shape: {:?}", labels.shape());

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

    info!("Saved to: {}", run_dir.display());

    let features_array = features.to_ndarray::<Float64Type>(IndexOrder::C)?;
    let targets_array = labels
        .to_ndarray::<Int32Type>(IndexOrder::C)?
        .column(0)
        .to_owned();

    info!("Features array shape: {:?}", features_array.shape());
    info!("Targets array shape: {:?}", targets_array.shape());

    let training_dataset = DatasetBase::new(features_array, targets_array);
    let (train, test) = training_dataset.split_with_ratio(0.8);

    info!(
        "Train size: {}, Test size: {}",
        train.nsamples(),
        test.nsamples()
    );

    let model = LogisticRegression::default()
        .max_iterations(300)
        .gradient_tolerance(0.0001)
        .fit(&train)?;
    info!("Model trained successfully");

    info!("\n=== Making Predictions ===");
    let predictions = model.predict(&test);
    info!("Predictions shape: {:?}", predictions.shape());

    let pred_bool = predictions.mapv(|x| x == 1);
    let target_bool = test.targets().mapv(|x| x == 1);
    let confusion = pred_bool.confusion_matrix(&target_bool)?;

    info!("\n=== Model Evaluation ===");
    info!("{:?}", confusion);
    info!("Accuracy:  {:.4}", confusion.accuracy());
    info!("Precision: {:.4}", confusion.precision());
    info!("Recall:    {:.4}", confusion.recall());
    info!("F1 Score:  {:.4}", confusion.f1_score());

    Ok(())
}
