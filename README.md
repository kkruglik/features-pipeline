# Features Pipeline

A config-driven ML feature engineering and training pipeline built in Rust.

## Overview

This project implements an end-to-end machine learning pipeline:

1. **Feature Engineering** - Config-driven transformations using Polars
2. **Label Encoding** - Automatic encoding of categorical targets
3. **Model Training** - Logistic regression with linfa
4. **Evaluation** - Confusion matrix and classification metrics

## Features

### Feature Transformations

All features defined in YAML, not hardcoded:

```yaml
steps:
  - function: mean
    column: age
    group_by: [occupation]
    name: avg_age_by_occupation

  - function: ratio
    numerator: capital-gain
    denominator: capital-loss
    name: capital_net

  - function: threshold
    column: hours-per-week
    threshold: 40.0
    comparator: gt
    name: high_hours

  - function: ohe
    columns: [gender, workclass]
    drop_first: true
    drop_nulls: true
```

**Supported feature types:**
- `mean`, `sum`, `max`, `min` - Aggregations with group_by
- `count`, `count_distinct` - Counting with group_by
- `ratio` - Division of two columns
- `threshold` - Boolean comparison (gt/lt)
- `ohe` - One-hot encoding

### Parallel Processing

Three execution modes:

```rust
// Sequential
let features = pipeline.apply(&df)?;

// Parallel with Rayon
let features = pipeline.apply_parallel(&df)?;

// Parallel with std::thread
let features = pipeline.apply_parallel_threads(&df)?;
```

### Model Training

```rust
let model = LogisticRegression::default()
    .max_iterations(300)
    .fit(&train)?;

let predictions = model.predict(&test);
let confusion = predictions.confusion_matrix(&test)?;

println!("Accuracy: {:.4}", confusion.accuracy());
println!("F1 Score: {:.4}", confusion.f1_score());
```

## Project Structure

```
src/
├── main.rs                 # Entry point, orchestrates pipeline
├── lib.rs                  # Library exports
├── errors.rs               # Custom error types
├── config/
│   └── entry.rs            # Entrypoint config (data paths)
└── pipeline/
    ├── features.rs         # Feature transformations
    └── labels.rs           # Label encoding

config/
├── entrypoint_adult.yaml   # Points to data and feature configs
├── features/
│   └── adult.yaml          # Feature definitions
└── labels/
    └── adult.yaml          # Target encoding config

data/
├── input/                  # Raw datasets
└── output/{timestamp}/     # Run outputs (features.csv, labels.csv)

docs/tutorials/
├── 01-getting-started.md
├── 05-tracing.md           # Structured logging
├── 06-criterion-benchmarking.md
└── 07-rayon-parallelism.md
```

## Usage

### Build and Run

```bash
# Run the pipeline
cargo run

# Release mode
cargo build --release
./target/release/features-pipeline

# With logging
RUST_LOG=info cargo run
```

### Configuration

1. Define features in `config/features/adult.yaml`
2. Define target encoding in `config/labels/adult.yaml`
3. Point to data in `config/entrypoint_adult.yaml`
4. Run the pipeline

### Output

Each run creates a timestamped folder:

```
data/output/20240122_161429/
├── features.csv    # Engineered features
└── labels.csv      # Encoded target
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| polars | DataFrame operations |
| serde + serde_yaml | Config parsing |
| linfa | ML training |
| linfa-logistic | Logistic regression |
| ndarray | Numeric arrays |
| rayon | Parallel processing |
| tracing | Structured logging |
| chrono | Timestamps |

## Example Output

```
INFO Raw data shape: (48842, 15)
INFO Features after fill_null: (48842, 19)
INFO Train size: 39073, Test size: 9769
INFO Model trained successfully

=== Model Evaluation ===
Accuracy:  0.7911
Precision: 0.2983
Recall:    0.6695
F1 Score:  0.4127
```

## Learning Project

This project was built to learn Rust concepts:

- Ownership and borrowing
- Enums with serde for config-driven design
- Custom error types with `From` trait
- Traits and pattern matching
- Parallel processing (rayon, std::thread)
- Working with Polars DataFrames

## License

MIT
