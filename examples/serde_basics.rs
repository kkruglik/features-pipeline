use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};

// ============================================================================
// SECTION 1: Basic Struct Serialization
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct SimpleConfig {
    name: String,
    threshold: f64,
    enabled: bool,
    tags: Vec<String>,
}

fn basic_json_example() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 1: Basic JSON Serialization ===\n");

    // Create a config
    let config = SimpleConfig {
        name: "feature_v1".to_string(),
        threshold: 0.95,
        enabled: true,
        tags: vec!["production".to_string(), "ml".to_string()],
    };

    // Serialize to JSON string
    let json = serde_json::to_string_pretty(&config)?;
    println!("Serialized JSON:\n{}", json);

    // Deserialize back
    let loaded: SimpleConfig = serde_json::from_str(&json)?;
    println!("\nDeserialized config: {:?}", loaded);

    Ok(())
}

// ============================================================================
// SECTION 2: Working with Files
// ============================================================================

fn json_file_io() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 2: JSON File I/O ===\n");

    let config = SimpleConfig {
        name: "test_feature".to_string(),
        threshold: 0.75,
        enabled: false,
        tags: vec!["experimental".to_string()],
    };

    // Save to file
    let file = File::create("data/example_config.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &config)?;
    println!("Saved config to data/example_config.json");

    // Load from file
    let file = File::open("data/example_config.json")?;
    let reader = BufReader::new(file);
    let loaded: SimpleConfig = serde_json::from_reader(reader)?;
    println!("Loaded config: {:?}", loaded);

    Ok(())
}

// ============================================================================
// SECTION 3: YAML Serialization
// ============================================================================

fn yaml_example() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 3: YAML Serialization ===\n");

    let config = SimpleConfig {
        name: "yaml_feature".to_string(),
        threshold: 0.88,
        enabled: true,
        tags: vec!["v2".to_string(), "optimized".to_string()],
    };

    // Serialize to YAML
    let yaml = serde_yaml::to_string(&config)?;
    println!("Serialized YAML:\n{}", yaml);

    // Deserialize from YAML
    let loaded: SimpleConfig = serde_yaml::from_str(&yaml)?;
    println!("Deserialized: {:?}", loaded);

    // Save to YAML file
    let file = File::create("data/example_config.yaml")?;
    let writer = BufWriter::new(file);
    serde_yaml::to_writer(writer, &config)?;
    println!("\nSaved to data/example_config.yaml");

    Ok(())
}

// ============================================================================
// SECTION 4: Serde Attributes - rename, default, skip
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct ApiConfig {
    // Rename in JSON: userId instead of user_id
    #[serde(rename = "userId")]
    user_id: i32,

    // Default value if missing (false for bool)
    #[serde(default)]
    debug_mode: bool,

    // Custom default function
    #[serde(default = "default_timeout")]
    timeout_seconds: u64,

    // Skip serialization (won't appear in JSON)
    #[serde(skip)]
    runtime_data: String,
}

fn default_timeout() -> u64 {
    30
}

fn attributes_example() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 4: Serde Attributes ===\n");

    let config = ApiConfig {
        user_id: 42,
        debug_mode: true,
        timeout_seconds: 60,
        runtime_data: "This won't be serialized".to_string(),
    };

    let json = serde_json::to_string_pretty(&config)?;
    println!("With all fields:\n{}", json);

    // Deserialize with missing fields (will use defaults)
    let json_missing = r#"{"userId": 100}"#;
    let loaded: ApiConfig = serde_json::from_str(json_missing)?;
    println!("\nWith missing fields (uses defaults):");
    println!("{:?}", loaded);
    println!(
        "  debug_mode: {} (default)",
        loaded.debug_mode
    );
    println!(
        "  timeout_seconds: {} (custom default)",
        loaded.timeout_seconds
    );

    Ok(())
}

// ============================================================================
// SECTION 5: Nested Structures
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct PipelineConfig {
    name: String,
    input: InputConfig,
    preprocessing: PreprocessingConfig,
    output: OutputConfig,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputConfig {
    path: String,
    format: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PreprocessingConfig {
    handle_nulls: String,
    normalize: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OutputConfig {
    path: String,
    format: String,
}

fn nested_structures() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 5: Nested Structures ===\n");

    let pipeline = PipelineConfig {
        name: "sales_pipeline".to_string(),
        input: InputConfig {
            path: "data/raw/".to_string(),
            format: "csv".to_string(),
        },
        preprocessing: PreprocessingConfig {
            handle_nulls: "drop".to_string(),
            normalize: true,
        },
        output: OutputConfig {
            path: "data/processed/".to_string(),
            format: "parquet".to_string(),
        },
    };

    // YAML is more readable for nested configs
    let yaml = serde_yaml::to_string(&pipeline)?;
    println!("Pipeline config (YAML):\n{}", yaml);

    // Save to file
    let file = File::create("data/pipeline_config.yaml")?;
    serde_yaml::to_writer(file, &pipeline)?;
    println!("Saved to data/pipeline_config.yaml");

    Ok(())
}

// ============================================================================
// SECTION 6: Feature Engineering Config
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct FeaturesConfig {
    features: Vec<FeatureDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FeatureDefinition {
    name: String,
    operation: AggregationType,
    column: String,

    #[serde(default)]
    group_by: Vec<String>,

    // Only serialize if Some (cleaner JSON/YAML)
    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    window_size: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum AggregationType {
    Mean,
    Sum,
    Count,
    Max,
    Min,
    Threshold,
}

fn feature_config_example() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 6: Feature Engineering Config ===\n");

    let features = FeaturesConfig {
        features: vec![
            FeatureDefinition {
                name: "avg_units_by_country".to_string(),
                operation: AggregationType::Mean,
                column: "Units Sold".to_string(),
                group_by: vec!["Country".to_string()],
                threshold: None,
                window_size: None,
            },
            FeatureDefinition {
                name: "total_profit_by_item".to_string(),
                operation: AggregationType::Sum,
                column: "Total Profit".to_string(),
                group_by: vec!["Country".to_string(), "Item Type".to_string()],
                threshold: None,
                window_size: None,
            },
            FeatureDefinition {
                name: "high_value_flag".to_string(),
                operation: AggregationType::Threshold,
                column: "Total Profit".to_string(),
                group_by: vec![],
                threshold: Some(100000.0),
                window_size: None,
            },
        ],
    };

    let yaml = serde_yaml::to_string(&features)?;
    println!("Features config:\n{}", yaml);

    // Save to file
    let file = File::create("data/features_config.yaml")?;
    serde_yaml::to_writer(file, &features)?;
    println!("Saved to data/features_config.yaml");

    Ok(())
}

// ============================================================================
// SECTION 7: Using Enums for Configuration
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct ModelConfig {
    model_type: ModelType,
    hyperparameters: HyperParams,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ModelType {
    LinearRegression,
    LogisticRegression { penalty: String },
    RandomForest { n_trees: usize, max_depth: usize },
}

#[derive(Serialize, Deserialize, Debug)]
struct HyperParams {
    learning_rate: f64,
    max_iterations: usize,
    random_seed: u64,
}

fn enum_config_example() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 7: Enum-based Configuration ===\n");

    let configs = vec![
        ModelConfig {
            model_type: ModelType::LinearRegression,
            hyperparameters: HyperParams {
                learning_rate: 0.01,
                max_iterations: 1000,
                random_seed: 42,
            },
        },
        ModelConfig {
            model_type: ModelType::RandomForest {
                n_trees: 100,
                max_depth: 10,
            },
            hyperparameters: HyperParams {
                learning_rate: 0.1,
                max_iterations: 500,
                random_seed: 42,
            },
        },
    ];

    for (i, config) in configs.iter().enumerate() {
        let json = serde_json::to_string_pretty(&config)?;
        println!("Model {} config:\n{}\n", i + 1, json);
    }

    Ok(())
}

// ============================================================================
// SECTION 8: Working with HashMap (dynamic configs)
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct DynamicConfig {
    name: String,
    settings: HashMap<String, ConfigValue>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ConfigValue {
    String(String),
    Number(f64),
    Bool(bool),
    List(Vec<String>),
}

fn hashmap_example() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 8: Dynamic Config with HashMap ===\n");

    let mut settings = HashMap::new();
    settings.insert("database_url".to_string(), ConfigValue::String("localhost:5432".to_string()));
    settings.insert("max_connections".to_string(), ConfigValue::Number(100.0));
    settings.insert("enable_cache".to_string(), ConfigValue::Bool(true));
    settings.insert("allowed_origins".to_string(), ConfigValue::List(vec!["localhost".to_string(), "api.example.com".to_string()]));

    let config = DynamicConfig {
        name: "app_config".to_string(),
        settings,
    };

    let json = serde_json::to_string_pretty(&config)?;
    println!("Dynamic config:\n{}", json);

    Ok(())
}

// ============================================================================
// SECTION 9: Error Handling & Validation
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct ValidatedConfig {
    threshold: f64,
    batch_size: usize,
    model_path: String,
}

#[derive(Debug)]
enum ConfigError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    Validation(String),
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Parse(err)
    }
}

fn load_and_validate_config(json: &str) -> Result<ValidatedConfig, ConfigError> {
    let config: ValidatedConfig = serde_json::from_str(json)?;

    // Custom validation
    if !(0.0..=1.0).contains(&config.threshold) {
        return Err(ConfigError::Validation(
            "threshold must be between 0 and 1".to_string()
        ));
    }

    if config.batch_size == 0 {
        return Err(ConfigError::Validation(
            "batch_size must be > 0".to_string()
        ));
    }

    Ok(config)
}

fn validation_example() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 9: Validation ===\n");

    let valid_json = r#"{
        "threshold": 0.8,
        "batch_size": 32,
        "model_path": "models/trained_model.bin"
    }"#;

    match load_and_validate_config(valid_json) {
        Ok(config) => println!("Valid config: {:?}", config),
        Err(e) => println!("Error: {:?}", e),
    }

    let invalid_json = r#"{
        "threshold": 1.5,
        "batch_size": 32,
        "model_path": "models/trained_model.bin"
    }"#;

    println!("\nTrying invalid config (threshold > 1.0):");
    match load_and_validate_config(invalid_json) {
        Ok(config) => println!("Valid config: {:?}", config),
        Err(e) => println!("Validation error: {:?}", e),
    }

    Ok(())
}

// ============================================================================
// SECTION 10: Optional Fields and Defaults
// ============================================================================

#[derive(Serialize, Deserialize, Debug)]
struct FlexibleConfig {
    // Required field
    name: String,

    // Optional with Option<T>
    description: Option<String>,

    // Default if missing
    #[serde(default = "default_workers")]
    workers: usize,

    // Use type's default (0)
    #[serde(default)]
    retry_count: usize,

    // Complex default
    #[serde(default = "default_tags")]
    tags: Vec<String>,
}

fn default_workers() -> usize { 4 }
fn default_tags() -> Vec<String> { vec!["default".to_string()] }

fn optional_fields_example() -> Result<(), Box<dyn Error>> {
    println!("\n=== SECTION 10: Optional Fields ===\n");

    // Full config
    let full = FlexibleConfig {
        name: "full_pipeline".to_string(),
        description: Some("Complete configuration".to_string()),
        workers: 8,
        retry_count: 3,
        tags: vec!["prod".to_string(), "v2".to_string()],
    };

    println!("Full config:");
    println!("{}", serde_json::to_string_pretty(&full)?);

    // Minimal config (only required field)
    let minimal_json = r#"{"name": "minimal_pipeline"}"#;
    let minimal: FlexibleConfig = serde_json::from_str(minimal_json)?;

    println!("\nMinimal config (with defaults):");
    println!("{:?}", minimal);
    println!("  description: {:?}", minimal.description); // None
    println!("  workers: {}", minimal.workers); // 4 (custom default)
    println!("  retry_count: {}", minimal.retry_count); // 0 (type default)
    println!("  tags: {:?}", minimal.tags); // ["default"]

    Ok(())
}

// ============================================================================
// MAIN - Run all examples
// ============================================================================

fn main() -> Result<(), Box<dyn Error>> {
    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║       Serde Basics - Serialization Examples          ║");
    println!("╚═══════════════════════════════════════════════════════╝");

    basic_json_example()?;
    json_file_io()?;
    yaml_example()?;
    attributes_example()?;
    nested_structures()?;
    feature_config_example()?;
    enum_config_example()?;
    hashmap_example()?;
    validation_example()?;
    optional_fields_example()?;

    println!("\n╔═══════════════════════════════════════════════════════╗");
    println!("║  All examples completed successfully!                ║");
    println!("║  Check data/ folder for generated config files       ║");
    println!("╚═══════════════════════════════════════════════════════╝\n");

    Ok(())
}
