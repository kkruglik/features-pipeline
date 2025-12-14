# Phase 3: Configuration with Serde

## What is Serde?

**Serde** = **Ser**ialization + **De**serialization

It's Rust's ecosystem-standard framework for converting Rust data structures to/from formats like JSON, YAML, TOML, CSV, etc.

**Why it matters for ML pipelines:**

- Store feature engineering configs (which features to create, thresholds, parameters)
- Save model hyperparameters
- Manage environment-specific settings (dev/prod)
- Export/import data in standard formats

## Core Concepts

### 1. The Two Main Traits

```rust
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer;
}

pub trait Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>;
}
```

**You never implement these manually** - use `#[derive(Serialize, Deserialize)]`

### 2. Data Formats (separate crates)

- `serde_json` - JSON format
- `serde_yaml` - YAML format
- `serde` built-in - CSV (via csv crate)
- `toml` - TOML format

Each format crate provides:

- `to_string()` / `from_str()` - String conversion
- `to_writer()` / `from_reader()` - File I/O
- Format-specific options

## Basic Usage Patterns

### Pattern 1: Simple Struct Serialization

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    name: String,
    threshold: f64,
    enabled: bool,
}

// Rust struct → JSON string
let config = Config {
    name: "feature_v1".to_string(),
    threshold: 0.95,
    enabled: true,
};
let json = serde_json::to_string(&config)?;

// JSON string → Rust struct
let loaded: Config = serde_json::from_str(&json)?;
```

### Pattern 2: Nested Structures

```rust
#[derive(Serialize, Deserialize, Debug)]
struct FeatureConfig {
    name: String,
    params: FeatureParams,
    transforms: Vec<Transform>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FeatureParams {
    window_size: usize,
    aggregation: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transform {
    operation: String,
    column: String,
}
```

### Pattern 3: Working with Files

```rust
use std::fs::File;
use std::io::{BufReader, BufWriter};

// Save to file
let file = File::create("config.json")?;
let writer = BufWriter::new(file);
serde_json::to_writer_pretty(writer, &config)?;

// Load from file
let file = File::open("config.json")?;
let reader = BufReader::new(file);
let config: Config = serde_json::from_reader(reader)?;
```

## Essential Attributes

### `#[serde(rename = "...")]`

Change field name in serialized format:

```rust
#[derive(Serialize, Deserialize)]
struct ApiResponse {
    #[serde(rename = "userId")]
    user_id: i32,  // Rust: user_id, JSON: userId
}
```

### `#[serde(default)]`

Use default value if field missing:

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,

    #[serde(default)]  // Uses Default::default() if missing
    enabled: bool,  // Will be false if not in config

    #[serde(default = "default_threshold")]
    threshold: f64,  // Custom default function
}

fn default_threshold() -> f64 { 0.5 }
```

### `#[serde(skip)]` / `#[serde(skip_serializing)]`

Exclude fields from serialization:

```rust
#[derive(Serialize, Deserialize)]
struct User {
    username: String,

    #[serde(skip)]  // Never serialize/deserialize
    runtime_cache: HashMap<String, String>,

    #[serde(skip_serializing)]  // Can deserialize, but won't serialize
    password: String,
}
```

### `#[serde(flatten)]`

Merge fields into parent:

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    name: String,

    #[serde(flatten)]
    settings: Settings,  // Settings fields appear at same level
}

#[derive(Serialize, Deserialize)]
struct Settings {
    debug: bool,
    timeout: u64,
}

// JSON: {"name": "app", "debug": true, "timeout": 30}
// Not: {"name": "app", "settings": {"debug": true, "timeout": 30}}
```

## ML Pipeline Use Cases

### Use Case 1: Feature Engineering Config

```yaml
# config/features.yaml
features:
  - name: avg_units_by_country
    operation: mean
    column: Units Sold
    group_by:
      - Country

  - name: total_profit_ratio
    operation: custom
    formula: "Total Profit / Total Revenue"

  - name: high_value_flag
    operation: threshold
    column: Total Profit
    threshold: 100000.0
```

```rust
#[derive(Serialize, Deserialize, Debug)]
struct FeaturesConfig {
    features: Vec<FeatureDefinition>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FeatureDefinition {
    name: String,
    operation: String,
    column: Option<String>,

    #[serde(default)]
    group_by: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    threshold: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    formula: Option<String>,
}
```

### Use Case 2: Pipeline Configuration

```yaml
# config/pipeline.yaml
pipeline:
  input:
    path: data/raw/
    format: csv

  preprocessing:
    handle_nulls: drop
    normalize: true

  feature_engineering:
    config_path: config/features.yaml

  output:
    path: data/processed/
    format: parquet
```

### Use Case 3: Model Hyperparameters

```json
{
  "model": "linear_regression",
  "hyperparameters": {
    "learning_rate": 0.01,
    "max_iterations": 1000,
    "regularization": "l2",
    "lambda": 0.1
  },
  "training": {
    "train_split": 0.8,
    "validation_split": 0.1,
    "random_seed": 42
  }
}
```

## Common Patterns for ML Pipelines

### Pattern: Environment-Specific Configs

```rust
#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    environment: String,

    #[serde(flatten)]
    settings: EnvironmentSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "environment")]
enum EnvironmentSettings {
    #[serde(rename = "development")]
    Dev { debug: bool, data_path: String },

    #[serde(rename = "production")]
    Prod { workers: usize, cache_size: usize },
}
```

### Pattern: Optional Nested Configs

```rust
#[derive(Serialize, Deserialize, Debug)]
struct PipelineConfig {
    name: String,

    #[serde(default)]
    preprocessing: Option<PreprocessingConfig>,

    feature_config: FeatureConfig,

    #[serde(default)]
    model_config: Option<ModelConfig>,
}
```

## Error Handling

```rust
use serde_json::Error as SerdeError;
use std::io::Error as IoError;

#[derive(Debug)]
enum ConfigError {
    Io(IoError),
    Parse(SerdeError),
    Validation(String),
}

impl From<IoError> for ConfigError {
    fn from(err: IoError) -> Self {
        ConfigError::Io(err)
    }
}

impl From<SerdeError> for ConfigError {
    fn from(err: SerdeError) -> Self {
        ConfigError::Parse(err)
    }
}

fn load_config(path: &str) -> Result<Config, ConfigError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;

    // Custom validation
    if config.threshold < 0.0 || config.threshold > 1.0 {
        return Err(ConfigError::Validation(
            "threshold must be between 0 and 1".to_string()
        ));
    }

    Ok(config)
}
```

## Best Practices

1. **Always use `#[derive(Debug)]`** alongside Serialize/Deserialize for debugging
2. **Use `to_string_pretty()` / `to_writer_pretty()`** for human-readable configs
3. **Add validation after deserializing** - serde checks types, not business logic
4. **Use enums for fixed choices** (e.g., aggregation types: Mean, Sum, Max)
5. **Use `Option<T>` for optional fields** - more Rustic than default values
6. **Create separate config modules** - don't mix config structs with business logic

## Learning Path

### Step 1: Basic Serialization (30 min)

- Create simple structs
- Serialize to JSON/YAML
- Deserialize back
- Use `Debug` to inspect

### Step 2: Attributes (1 hour)

- Practice `#[serde(rename)]`
- Use `#[serde(default)]` for optional fields
- Try `#[serde(flatten)]` for composition

### Step 3: Real Config (1-2 hours)

- Create feature engineering config struct
- Write YAML config file
- Load and use in your pipeline
- Add validation logic

### Step 4: Multiple Formats (30 min)

- Same struct → JSON, YAML, TOML
- Understand format differences
- Choose format for your use case

## Next Steps

After mastering serde:

1. Create `config/` directory for your pipeline configs
2. Define config structs for feature engineering
3. Implement config loading in your pipeline
4. Add CLI args (clap) to override config values
5. Move to Phase 4: ndarray + linfa for ML models

## Quick Reference

```rust
// JSON
let json = serde_json::to_string(&data)?;
let data: Config = serde_json::from_str(&json)?;

// YAML
let yaml = serde_yaml::to_string(&data)?;
let data: Config = serde_yaml::from_str(&yaml)?;

// File I/O
serde_json::to_writer_pretty(File::create("config.json")?, &data)?;
let data: Config = serde_json::from_reader(File::open("config.json")?)?;

// Common attributes
#[serde(rename = "newName")]
#[serde(default)]
#[serde(skip)]
#[serde(flatten)]
#[serde(skip_serializing_if = "Option::is_none")]
```
