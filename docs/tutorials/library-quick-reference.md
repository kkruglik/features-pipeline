# Library Quick Reference

Quick lookup for common operations in each library.

## Polars

### Creating DataFrames

```rust
use polars::prelude::*;

// From CSV
let df = CsvReader::from_path("data.csv")?.finish()?;

// From Parquet
let df = ParquetReader::from_path("data.parquet")?.finish()?;

// From scratch
let df = df! {
    "name" => &["Alice", "Bob"],
    "age" => &[25, 30],
}?;
```

### Basic Operations

```rust
// Shape
let (rows, cols) = df.shape();

// Column names
let names = df.get_column_names();

// First/last n rows
df.head(Some(5));
df.tail(Some(10));

// Select columns
df.select(["name", "age"])?;

// Filter rows
df.filter(&df.column("age")?.gt(25)?)?;
```

### Transformations

```rust
// Add new column
df.with_column(
    col("age").alias("age_doubled") * 2
)?;

// Multiple transformations
df.lazy()
    .select([
        col("age"),
        col("income").log().alias("log_income"),
        col("name").str().to_uppercase(),
    ])
    .filter(col("age").gt(lit(20)))
    .collect()?;
```

### Aggregations

```rust
// Group by
df.groupby(["country"])?
    .agg(&[
        col("age").mean().alias("avg_age"),
        col("income").sum().alias("total_income"),
    ])?;

// Statistics
df.column("age")?.mean();
df.column("age")?.std(0);
df.column("age")?.min();
```

### Missing Data

```rust
// Fill nulls
df.column("age")?.fill_null(FillNullStrategy::Mean)?;
df.column("age")?.fill_null(FillNullStrategy::Forward)?;

// Drop nulls
df.drop_nulls(None)?;
```

---

## ndarray

### Creating Arrays

```rust
use ndarray::prelude::*;

// From vec
let arr = Array1::from_vec(vec![1.0, 2.0, 3.0]);

// 2D array
let arr = Array2::from_shape_vec((3, 2), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])?;

// Zeros, ones
let zeros = Array2::<f64>::zeros((3, 3));
let ones = Array2::<f64>::ones((2, 4));

// Range
let arr = Array::range(0.0, 10.0, 1.0);
```

### Indexing

```rust
// Single element
let val = arr[[2, 1]];

// Slicing
let slice = arr.slice(s![0..2, ..]);  // First 2 rows, all columns

// Row/column
let row = arr.row(0);
let col = arr.column(1);
```

### Operations

```rust
// Element-wise
let result = &arr1 + &arr2;
let result = &arr * 2.0;

// Matrix multiplication
let result = arr1.dot(&arr2);

// Transpose
let transposed = arr.t();

// Sum, mean
arr.sum();
arr.mean().unwrap();
```

---

## linfa

### Creating Dataset

```rust
use linfa::prelude::*;
use ndarray::Array2;

// Features and targets
let features = Array2::from_shape_vec((100, 5), data)?;
let targets = Array1::from_vec(labels);

// Create dataset
let dataset = Dataset::new(features, targets);

// Train/test split
let (train, test) = dataset.split_with_ratio(0.8);
```

### Linear Regression

```rust
use linfa_linear::LinearRegression;

// Train
let model = LinearRegression::default()
    .fit(&train)?;

// Predict
let predictions = model.predict(&test);

// Evaluate
let r2 = predictions.r2(&test)?;
```

### Logistic Regression

```rust
use linfa_linear::LogisticRegression;

// Train
let model = LogisticRegression::default()
    .max_iterations(100)
    .fit(&train)?;

// Predict probabilities
let probabilities = model.predict(&test);

// Predict classes
let classes = model.predict(&test);
```

---

## serde + YAML

### Defining Structs

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    name: String,
    value: i32,
    #[serde(default)]  // Use default if missing
    optional: bool,
}
```

### Loading YAML

```rust
use std::fs;

// Read file
let content = fs::read_to_string("config.yaml")?;

// Parse
let config: Config = serde_yaml::from_str(&content)?;
```

### Saving YAML

```rust
// Serialize
let yaml = serde_yaml::to_string(&config)?;

// Write to file
fs::write("output.yaml", yaml)?;
```

---

## clap

### Defining CLI

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "my-app")]
#[command(about = "Does awesome things")]
struct Cli {
    /// Verbosity level
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Train a model
    Train {
        /// Path to data
        #[arg(short, long)]
        data: PathBuf,
    },
    /// Make predictions
    Predict {
        #[arg(short, long)]
        model: PathBuf,
    },
}
```

### Using CLI

```rust
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Train { data } => {
            // Train logic
        }
        Commands::Predict { model } => {
            // Predict logic
        }
    }
}
```

---

## tracing

### Setup

```rust
use tracing::{info, warn, error, debug};
use tracing_subscriber;

fn main() {
    // Initialize
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Application started");
}
```

### Logging

```rust
// Simple messages
info!("Processing file: {}", filename);
warn!("Missing value, using default");
error!("Failed to load file: {}", err);

// With fields
info!(
    rows = df.height(),
    cols = df.width(),
    "Loaded DataFrame"
);

// Spans (for timing)
use tracing::instrument;

#[instrument]
fn process_data(df: &DataFrame) -> Result<DataFrame> {
    info!("Starting processing");
    // ... work ...
    Ok(processed)
}
```

---

## rayon

### Parallel Iterators

```rust
use rayon::prelude::*;

// Parallel map
let results: Vec<_> = data
    .par_iter()
    .map(|x| expensive_operation(x))
    .collect();

// Parallel filter
let filtered: Vec<_> = data
    .par_iter()
    .filter(|x| x > &threshold)
    .collect();

// Parallel for_each
data.par_iter().for_each(|x| {
    process(x);
});
```

### When to Use

- Processing large collections (>10k items)
- CPU-bound operations
- Independent computations (no shared state)

---

## anyhow

### Error Handling

```rust
use anyhow::{Result, Context, bail};

fn load_data(path: &Path) -> Result<DataFrame> {
    // Automatic error conversion
    let df = CsvReader::from_path(path)?
        .finish()
        .context("Failed to parse CSV")?;  // Add context

    // Manual error
    if df.height() == 0 {
        bail!("DataFrame is empty");
    }

    Ok(df)
}

fn main() -> Result<()> {
    let df = load_data(Path::new("data.csv"))?;
    Ok(())
}
```

---

## Common Patterns

### Polars → ndarray

```rust
// Get columns as ndarray
let features: Array2<f64> = df
    .select(["age", "income"])?
    .to_ndarray::<Float64Type>()?;

let target: Array1<f64> = df
    .column("target")?
    .f64()?
    .to_ndarray()?;
```

### Error handling with context

```rust
load_csv(path)
    .with_context(|| format!("Failed to load file: {:?}", path))?;
```

### Lazy evaluation for performance

```rust
let result = df.lazy()
    .filter(col("age").gt(lit(18)))
    .select([
        col("name"),
        col("age"),
    ])
    .groupby([col("country")])
    .agg([col("age").mean()])
    .collect()?;
```

This reference grows as you learn. Add your own snippets!
