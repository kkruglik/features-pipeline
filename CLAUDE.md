# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a learning project building an ML feature engineering and training pipeline in Rust. The project is being developed in phases (currently Phase 3 complete, moving to Phase 4). The owner wants to learn Rust concepts themselves - provide guidance and explanations, not complete solutions.

## Commands

### Build and Run
```bash
# Run the feature pipeline
cargo run

# Build without running
cargo build

# Run in release mode
cargo build --release && ./target/release/features-pipeline

# Check for compilation errors
cargo check
```

### Development
```bash
# Format code
cargo fmt

# Run linter
cargo clippy

# Run tests (when implemented)
cargo test

# Run benchmarks (Phase 8)
cargo bench
```

## Architecture

### Config-Driven Pipeline Design

The entire feature engineering pipeline is driven by YAML configs, not hardcoded in Rust:

1. **Entrypoint config** (`config/entrypoint.yaml`) - Points to data and features
2. **Features config** (`config/features/base.yaml`) - Defines feature transformations
3. **Runtime** - Features applied sequentially to DataFrame

Key architectural pattern:
```rust
// Tagged enum with serde deserializes YAML "function" field
#[serde(tag = "function")]
pub enum FeatureConfig {
    #[serde(rename = "mean")]
    Mean { column: String, group_by: Vec<String>, name: String },
    // ... 7 other feature types
}

// Each enum variant implements its own logic
impl FeatureConfig {
    pub fn apply_feature(&self, data: &DataFrame) -> Result<DataFrame, FeatureError> {
        match self { /* Polars lazy operations */ }
    }
}
```

### Module Organization

```
src/
├── errors.rs       - Custom error types (FeatureError, ConfigError)
├── config/mod.rs   - Feature definitions, YAML parsing, DataFrame transformations
├── lib.rs          - Public exports (pub mod config; pub mod errors;)
└── main.rs         - Binary entry point, orchestrates pipeline
```

**Key insight:** All feature logic lives in `config/mod.rs` (321 lines). This will be split into separate modules in Phase 4.

### Error Handling

Uses **custom error enums** (not anyhow or thiserror for learning purposes):

```rust
// Domain-specific errors with context
pub enum FeatureError {
    ColumnNotFound { found: String, available: Vec<String> },  // Shows available columns
    DataframeError(PolarsError),  // Wraps Polars errors
    // ...
}

// From implementations enable ? operator
impl From<PolarsError> for FeatureError { ... }
```

Pattern:
- Library functions → Return typed errors (`Result<T, FeatureError>`)
- main() → Uses `Box<dyn Error>` for convenience
- Never use `.unwrap()` in production code - always use `?`

See `docs/tutorials/error-handling.md` for full reference.

## Data Flow

```
config/entrypoint.yaml
  ↓ (points to files)
data/input/10000 Sales Records.csv (14 columns)
  ↓
config/features/base.yaml (defines 9 features)
  ↓
PipelineSteps::apply(&DataFrame) (sequential feature application)
  ↓
data/output/{timestamp}/output.csv (23 columns = 14 + 9 features)
```

**Important:** Every run creates a timestamped output folder - never overwrites previous runs.

## Feature Types Implemented (Phase 3)

All features use Polars lazy evaluation:

1. **Aggregations with grouping** - mean, sum, max, min, count, count_distinct
   - Uses `.over()` for window functions
2. **Ratio** - Division of two columns
3. **Threshold** - Boolean comparison (gt/lt)

Example YAML:
```yaml
- name: avg_units_by_country
  function: mean
  column: Units Sold
  group_by: [Country]
```

Generates: `feature_avg_units_by_country` column in output DataFrame.

## Key Design Patterns

### 1. Fail-Fast Validation
Configs are validated immediately on load:
```rust
impl EntrypointConfig {
    pub fn load_from_yaml(filepath: &str) -> Result<Self, ConfigError> {
        let config: EntrypointConfig = from_reader(reader)?;
        config.validate()?;  // Check files exist BEFORE proceeding
        Ok(config)
    }
}
```

### 2. Timestamped Outputs
```rust
let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
let run_dir = PathBuf::from("data/output").join(timestamp);
```

### 3. Polars Lazy Evaluation
All features use `.lazy()` → transformations → `.collect()`:
```rust
data.clone()
    .lazy()
    .with_columns([col(column).mean().over(groupby_cols).alias(name)])
    .collect()?
```

## Future Phases (Not Yet Implemented)

- **Phase 4** - Model training with linfa/ndarray
- **Phase 5** - CLI with clap (subcommands: features, train, pipeline)
- **Phase 6** - Observability with tracing
- **Phase 7** - Parallelism with rayon
- **Phase 8** - Benchmarking with criterion
- **Phase 9** - Experiment tracking (MLflow-lite)

Dependencies are already added but unused. Don't suggest using them until the relevant phase.

## Working with This Project

### When Adding Features
1. Add variant to `FeatureConfig` enum in `src/config/mod.rs`
2. Add `#[serde(rename = "function_name")]` attribute
3. Implement logic in `apply_feature()` match arm
4. Update `config/features/base.yaml` to test

### When Modifying Error Handling
- Errors are in `src/errors.rs`
- Add new error variants with descriptive fields
- Implement `Display` for user-facing messages
- Add `From` implementations for automatic conversions with `?`

### When Reading Code
- Start with `src/main.rs` (56 lines) - see the full pipeline flow
- Then `src/config/mod.rs` - understand feature implementations
- Config validation happens in `EntrypointConfig::validate()`
- Feature validation happens in `FeatureConfig::apply_feature()`

## Common Gotchas

1. **Empty group_by arrays** - Features with `group_by: []` don't use `.over()` (global aggregation)
2. **DataFrame cloning** - Currently clones the entire DataFrame for each feature (optimization opportunity in Phase 7)
3. **Column name format** - All feature columns prefixed with `feature_` (e.g., `feature_avg_units_by_country`)
4. **CSV delimiter** - Output uses `;` separator (see `CsvWriter::with_separator(b';')`)

## Documentation

- `docs/project/architecture.md` - Detailed architecture diagrams
- `docs/project/current-status.md` - Current phase status and metrics
- `docs/tutorials/error-handling.md` - Error handling patterns reference
- `docs/tutorials/02-learning-path-phases-4-9.md` - Future phases roadmap

## Learning Project Guidelines

This is a **learning project**. The owner wants to:
- Understand Rust concepts deeply (ownership, traits, error handling, etc.)
- Implement features themselves with guidance
- Learn by doing, not by copying solutions

When helping:
- Explain concepts and trade-offs
- Point to relevant documentation
- Suggest approaches, don't write complete implementations
- Ask clarifying questions about design choices
- Review code and provide feedback

## Performance Notes

Current baseline (Phase 3):
- Dataset: 10,000 rows × 14 columns
- Processing time: ~2-3 seconds
- Output: 10,000 rows × 23 columns
- Peak memory: ~50MB

Optimization is planned for Phase 7 (rayon parallelism).
