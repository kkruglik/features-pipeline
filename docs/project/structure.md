# Project Structure

This document shows the final project structure you'll build throughout the learning phases.

## Directory Layout

```
features-pipeline/
├── Cargo.toml                 # Dependencies and project config
├── Cargo.lock                 # Locked dependency versions
├── README.md                  # Project overview
│
├── src/
│   ├── main.rs               # CLI entry point
│   ├── lib.rs                # Library root (if needed)
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   └── pipeline.rs       # Pipeline config structs
│   │
│   ├── data/
│   │   ├── mod.rs
│   │   └── loader.rs         # Load CSV/Parquet files
│   │
│   ├── features/
│   │   ├── mod.rs
│   │   ├── transformations.rs # Feature engineering logic
│   │   ├── numerical.rs       # Numerical transformations
│   │   ├── categorical.rs     # Categorical encoding
│   │   └── datetime.rs        # DateTime feature extraction
│   │
│   ├── ml/
│   │   ├── mod.rs
│   │   ├── dataset.rs         # Polars → ndarray conversion
│   │   ├── models.rs          # Model training/prediction
│   │   └── metrics.rs         # Evaluation metrics
│   │
│   └── utils/
│       ├── mod.rs
│       └── logging.rs         # Tracing setup
│
├── config/
│   ├── pipeline.yaml          # Example pipeline config
│   └── model.yaml             # Model config
│
├── data/
│   ├── sample.csv             # Sample dataset
│   ├── train.csv              # Training data
│   └── test.csv               # Test data
│
├── models/
│   └── linear_model.bin       # Saved models
│
├── examples/
│   ├── polars_basics.rs       # Learning examples
│   ├── ndarray_basics.rs
│   └── end_to_end.rs          # Complete pipeline example
│
├── benches/
│   └── transformations.rs     # Performance benchmarks
│
├── tests/
│   └── integration_test.rs    # Integration tests
│
└── docs/
    ├── README.md              # Learning roadmap (this doc)
    ├── 01-getting-started.md  # Phase 1 guide
    ├── 02-feature-engineering.md
    ├── 03-config-system.md
    ├── 04-ml-models.md
    └── project-structure.md   # This file
```

## Module Responsibilities

### `main.rs`
- CLI interface with clap
- Subcommands: `transform`, `train`, `predict`
- Initialize logging
- Call appropriate modules

### `data/loader.rs`
- Load data from various formats
- Basic data validation
- Return Polars DataFrame

### `features/transformations.rs`
- Apply transformations based on config
- Coordinate numerical, categorical, datetime features
- Handle missing data

### `features/numerical.rs`
Functions:
- `normalize()` - Min-max scaling
- `standardize()` - Z-score normalization
- `log_transform()` - Log transformation
- `bin_continuous()` - Binning

### `features/categorical.rs`
Functions:
- `one_hot_encode()` - Create dummy variables
- `label_encode()` - Integer encoding
- `frequency_encode()` - Encode by frequency

### `features/datetime.rs`
Functions:
- `extract_date_parts()` - Year, month, day, day-of-week
- `calculate_time_diff()` - Time differences
- `is_weekend()` - Boolean features

### `config/pipeline.rs`
Structs:
```rust
#[derive(Deserialize)]
struct PipelineConfig {
    input: InputConfig,
    transformations: Vec<Transformation>,
    output: OutputConfig,
}

#[derive(Deserialize)]
enum Transformation {
    Numerical { column: String, method: ScalingMethod },
    Categorical { column: String, method: EncodingMethod },
    DateTime { column: String, extract: Vec<DatePart> },
}
```

### `ml/dataset.rs`
Functions:
- `dataframe_to_array()` - Convert Polars → ndarray
- `train_test_split()` - Split data
- `create_dataset()` - Create linfa Dataset

### `ml/models.rs`
Functions:
- `train_linear_regression()` - Train model
- `train_logistic_regression()`
- `predict()` - Make predictions
- `save_model()` / `load_model()` - Persistence

### `ml/metrics.rs`
Functions:
- `mean_squared_error()`
- `r2_score()`
- `accuracy()`
- `confusion_matrix()`

## Configuration Files

### `config/pipeline.yaml`
```yaml
input:
  path: "data/train.csv"
  format: "csv"

transformations:
  - type: "numerical"
    column: "age"
    method: "standardize"

  - type: "numerical"
    column: "income"
    method: "log"

  - type: "categorical"
    column: "country"
    method: "one_hot"

  - type: "datetime"
    column: "created_at"
    extract: ["year", "month", "day_of_week"]

output:
  path: "data/processed.parquet"
  format: "parquet"
```

### `config/model.yaml`
```yaml
model:
  type: "linear_regression"

  features:
    - "age"
    - "income_log"
    - "country_US"
    - "country_UK"

  target: "purchased"

  train_test_split: 0.8

  hyperparameters:
    alpha: 0.1  # regularization
```

## CLI Usage Examples

```bash
# Transform data
cargo run -- transform \
  --config config/pipeline.yaml \
  --output data/processed.parquet

# Train model
cargo run -- train \
  --data data/processed.parquet \
  --config config/model.yaml \
  --output models/model.bin

# Make predictions
cargo run -- predict \
  --model models/model.bin \
  --data data/test.csv \
  --output predictions.csv

# With verbose logging
cargo run -- train --data data.csv --verbose
```

## Building the Structure

You don't need to create all this upfront. Build incrementally:

1. **Phase 1:** `src/main.rs` + `src/data/loader.rs`
2. **Phase 2:** Add `src/features/` modules
3. **Phase 3:** Add `src/config/`
4. **Phase 4:** Add `src/ml/`
5. **Phase 5:** Expand `src/main.rs` with full CLI
6. **Phase 6:** Add `benches/`
7. **Phase 7:** Add model persistence

## Tips

- Use `mod.rs` files to expose public items from modules
- Keep modules focused on single responsibility
- Use `pub(crate)` for internal-only items
- Add tests alongside code (`#[cfg(test)]` modules)

## Next Steps

Return to [Learning Roadmap](./README.md) and start Phase 1.
