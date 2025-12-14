# Next Steps: From Feature Engineering to Full ML Pipeline

**Current Status:** ✅ Feature Engineering Complete (Phases 1-3)

You've successfully built a production-ready feature engineering pipeline with Polars and Serde. Now it's time to add model training and productionize the system.

---

## What You've Mastered ✅

| Library | Status | What You Built |
|---------|--------|----------------|
| **Polars** | ✅ Complete | DataFrames, lazy evaluation, aggregations, window functions, 8 feature types |
| **Serde** | ✅ Complete | YAML configs, tagged enums, validation, entrypoint pattern |
| **Chrono** | ✅ Complete | Timestamped output folders |
| **Error Handling** | ✅ Complete | `Result` types, validation on load, fail-fast |

**Code Quality:** Production-ready (9.5/10)
- ✅ Clean module structure
- ✅ Method on enum pattern
- ✅ Config-driven pipeline
- ✅ Proper error handling
- ✅ Timestamped outputs

---

## Unused Dependencies (Your Next Tasks) ⚠️

```toml
# In Cargo.toml but not used yet:
anyhow = "1.0.100"           # Better error handling (optional upgrade)
ndarray = "0.16.1"           # ❌ CRITICAL for Linfa!
rayon = "1.11.0"             # ❌ Parallel processing
tracing = "0.1.41"           # ❌ Observability/logging
tracing-subscriber = "0.3"   # ❌ Log formatting
clap = { version = "4.5" }   # ❌ CLI arguments
linfa = "0.8.0"              # ❌ ML algorithms
linfa-linear = "0.8.0"       # ❌ Linear regression
criterion = "0.5"            # ❌ Benchmarking
```

---

## Phase 4: Model Training with Linfa 🎯 **START HERE**

**Timeline:** 1 week
**Goal:** Train linear regression model to predict Total Profit

### Week Plan

**Day 1-2: Learn ndarray**
- ndarray is Rust's NumPy - required for Linfa
- Understand Array1, Array2, shapes, indexing
- Convert Polars DataFrame → ndarray

```rust
// Example: Polars → ndarray conversion
use ndarray::{Array1, Array2};
use polars::prelude::*;

fn df_to_array2(df: &DataFrame, cols: &[&str]) -> Result<Array2<f64>> {
    let ncols = cols.len();
    let nrows = df.height();

    let mut data = Vec::with_capacity(nrows * ncols);

    for col_name in cols {
        let series = df.column(col_name)?;
        let vec = series.f64()?.to_vec();
        data.extend(vec);
    }

    Ok(Array2::from_shape_vec((nrows, ncols), data)?)
}
```

**Resources:**
- ndarray docs: https://docs.rs/ndarray/latest/ndarray/
- ndarray tutorial: https://rust-ml.github.io/book/2_arrays.html

---

**Day 3-4: Integrate Linfa**

Create `src/models/mod.rs`:

```rust
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::{Array1, Array2};
use polars::prelude::*;
use std::error::Error;

pub struct ModelConfig {
    pub target_column: String,
    pub feature_columns: Vec<String>,
}

pub struct ProfitPredictor {
    model: LinearRegression<f64>,
    feature_names: Vec<String>,
}

impl ProfitPredictor {
    /// Train model from DataFrame with engineered features
    pub fn train(df: &DataFrame, config: &ModelConfig) -> Result<Self, Box<dyn Error>> {
        // Extract features
        let feature_cols: Vec<&str> = config.feature_columns
            .iter()
            .map(|s| s.as_str())
            .collect();

        let X = df_to_array2(df, &feature_cols)?;

        // Extract target
        let y_series = df.column(&config.target_column)?;
        let y_vec = y_series.f64()?.to_vec();
        let y = Array1::from_vec(y_vec);

        // Create dataset
        let dataset = Dataset::new(X, y);

        // Train model
        let model = LinearRegression::default().fit(&dataset)?;

        println!("Model trained successfully!");
        println!("  Features: {:?}", config.feature_columns);
        println!("  Target: {}", config.target_column);

        Ok(Self {
            model,
            feature_names: config.feature_columns.clone(),
        })
    }

    /// Make predictions on new data
    pub fn predict(&self, df: &DataFrame) -> Result<Array1<f64>, Box<dyn Error>> {
        let feature_cols: Vec<&str> = self.feature_names
            .iter()
            .map(|s| s.as_str())
            .collect();

        let X = df_to_array2(df, &feature_cols)?;
        Ok(self.model.predict(&X))
    }

    /// Compute model metrics
    pub fn evaluate(&self, df: &DataFrame, target_col: &str) -> Result<ModelMetrics, Box<dyn Error>> {
        let predictions = self.predict(df)?;

        let y_series = df.column(target_col)?;
        let y_true = y_series.f64()?.to_vec();

        let rmse = compute_rmse(&predictions.to_vec(), &y_true);
        let mae = compute_mae(&predictions.to_vec(), &y_true);
        let r2 = compute_r2(&predictions.to_vec(), &y_true);

        Ok(ModelMetrics { rmse, mae, r2 })
    }
}

pub struct ModelMetrics {
    pub rmse: f64,
    pub mae: f64,
    pub r2: f64,
}

fn compute_rmse(predictions: &[f64], actuals: &[f64]) -> f64 {
    let mse: f64 = predictions.iter()
        .zip(actuals)
        .map(|(p, a)| (p - a).powi(2))
        .sum::<f64>() / predictions.len() as f64;
    mse.sqrt()
}

fn compute_mae(predictions: &[f64], actuals: &[f64]) -> f64 {
    predictions.iter()
        .zip(actuals)
        .map(|(p, a)| (p - a).abs())
        .sum::<f64>() / predictions.len() as f64
}

fn compute_r2(predictions: &[f64], actuals: &[f64]) -> f64 {
    let mean = actuals.iter().sum::<f64>() / actuals.len() as f64;
    let ss_tot: f64 = actuals.iter().map(|a| (a - mean).powi(2)).sum();
    let ss_res: f64 = predictions.iter()
        .zip(actuals)
        .map(|(p, a)| (a - p).powi(2))
        .sum();
    1.0 - (ss_res / ss_tot)
}

// Helper function
fn df_to_array2(df: &DataFrame, cols: &[&str]) -> Result<Array2<f64>, Box<dyn Error>> {
    let ncols = cols.len();
    let nrows = df.height();

    let mut data = Vec::with_capacity(nrows * ncols);

    for col_name in cols {
        let series = df.column(col_name)?;
        let vec = series.f64()?
            .to_vec()
            .into_iter()
            .map(|v| v.unwrap_or(0.0))
            .collect::<Vec<_>>();
        data.extend(vec);
    }

    // ndarray uses column-major order, need to transpose
    let arr = Array2::from_shape_vec((ncols, nrows), data)?;
    Ok(arr.t().to_owned())
}
```

---

**Day 5-7: Integrate with Main Pipeline**

Update `src/lib.rs`:
```rust
pub mod config;
pub mod models;  // Add this
```

Create `config/model.yaml`:
```yaml
target_column: Total Profit
feature_columns:
  - feature_avg_units_by_country
  - feature_total_profit_by_item
  - feature_revenue_by_country_item
  - feature_profit_margin
  - Units Sold
  - Unit Price
  - Unit Cost
  - Total Revenue
```

Update `src/main.rs` to train model after features:
```rust
use features_pipeline::models::{ModelConfig, ProfitPredictor};

fn main() -> Result<(), Box<dyn Error>> {
    // ... existing feature engineering code ...

    println!("\n=== Training Model ===");

    // Load model config
    let model_config_file = File::open("config/model.yaml")?;
    let model_config: ModelConfig = serde_yaml::from_reader(model_config_file)?;

    // Train model
    let predictor = ProfitPredictor::train(&df, &model_config)?;

    // Evaluate
    let metrics = predictor.evaluate(&df, &model_config.target_column)?;
    println!("\n=== Model Metrics ===");
    println!("  RMSE: {:.2}", metrics.rmse);
    println!("  MAE: {:.2}", metrics.mae);
    println!("  R²: {:.4}", metrics.r2);

    // Save model (add later)
    // predictor.save(run_dir.join("model.bin"))?;

    Ok(())
}
```

**Test:**
```bash
cargo run
```

**Expected output:**
```
=== Feature Engineering ===
Data before transform: (10000, 14)
Data after transform: (10000, 23)

=== Training Model ===
Model trained successfully!
  Features: ["feature_avg_units_by_country", ...]
  Target: Total Profit

=== Model Metrics ===
  RMSE: 145234.56
  MAE: 98432.12
  R²: 0.7845
```

---

## Phase 5: CLI with Clap 🎯 **Week 2**

**Goal:** Make pipeline configurable via command line

### Implementation

Update `src/main.rs`:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "features-pipeline")]
#[command(version, about = "Feature engineering and ML pipeline", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate features from raw data
    Features {
        /// Path to entrypoint config
        #[arg(short, long, default_value = "config/entrypoint.yaml")]
        config: String,

        /// Output directory
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Train a model on engineered features
    Train {
        /// Path to feature-engineered data
        #[arg(short, long)]
        data: String,

        /// Path to model config
        #[arg(short = 'm', long, default_value = "config/model.yaml")]
        model_config: String,
    },

    /// Run full pipeline (features + training)
    Pipeline {
        /// Path to entrypoint config
        #[arg(short, long, default_value = "config/entrypoint.yaml")]
        config: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Features { config, output } => {
            run_feature_engineering(&config, output.as_deref())?;
        }
        Commands::Train { data, model_config } => {
            run_model_training(&data, &model_config)?;
        }
        Commands::Pipeline { config } => {
            run_full_pipeline(&config)?;
        }
    }

    Ok(())
}

fn run_feature_engineering(config_path: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
    // Your existing feature engineering code
    println!("Running feature engineering...");
    // ...
    Ok(())
}

fn run_model_training(data_path: &str, config_path: &str) -> Result<(), Box<dyn Error>> {
    println!("Training model...");
    // Load data, train model, save
    // ...
    Ok(())
}

fn run_full_pipeline(config_path: &str) -> Result<(), Box<dyn Error>> {
    println!("Running full pipeline...");
    // Features + training
    // ...
    Ok(())
}
```

**Usage:**
```bash
# Generate features only
cargo run -- features --config my_config.yaml

# Train model on existing features
cargo run -- train --data data/output/20251114_172412/output.csv

# Full pipeline
cargo run -- pipeline

# Help
cargo run -- --help
cargo run -- features --help
```

---

## Phase 6: Observability with Tracing 🎯 **Week 3**

**Goal:** Add structured logging throughout pipeline

### Implementation

Update `Cargo.toml` (already have deps):
```toml
[dependencies]
tracing = "0.1.41"
tracing-subscriber = "0.3"
```

Add to `src/main.rs`:
```rust
use tracing::{info, warn, error, debug, instrument, span, Level};
use tracing_subscriber;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("Starting features-pipeline");

    let cli = Cli::parse();
    // ... rest of code
}

#[instrument(skip(df), fields(rows = df.height(), cols = df.width()))]
fn run_feature_engineering(df: &DataFrame, config: &PipelineSteps) -> Result<DataFrame, Box<dyn Error>> {
    info!("Applying {} feature transformations", config.steps.len());

    let mut result = df.clone();

    for (idx, feature) in config.steps.iter().enumerate() {
        let span = span!(Level::DEBUG, "feature",
                        idx = idx,
                        name = feature.name());
        let _enter = span.enter();

        debug!("Processing feature: {:?}", feature);

        result = feature.apply_feature(&result)
            .map_err(|e| {
                error!("Feature {} failed: {}", feature.name(), e);
                e
            })?;
    }

    info!("Feature engineering complete: {} → {} columns",
          df.width(), result.width());

    Ok(result)
}

#[instrument(skip(df))]
fn train_model(df: &DataFrame, config: &ModelConfig) -> Result<ProfitPredictor, Box<dyn Error>> {
    info!("Training model with {} features", config.feature_columns.len());

    let predictor = ProfitPredictor::train(df, config)?;

    let metrics = predictor.evaluate(df, &config.target_column)?;
    info!("Model trained: RMSE={:.2}, R²={:.4}", metrics.rmse, metrics.r2);

    Ok(predictor)
}
```

**Run with different log levels:**
```bash
# Info level (default)
RUST_LOG=info cargo run -- pipeline

# Debug level (more detailed)
RUST_LOG=debug cargo run -- pipeline

# Trace level (everything)
RUST_LOG=trace cargo run -- pipeline
```

**Output example:**
```
2025-11-14T17:00:00Z INFO features_pipeline: Starting features-pipeline
2025-11-14T17:00:01Z INFO run_feature_engineering{rows=10000 cols=14}: Applying 10 feature transformations
2025-11-14T17:00:01Z DEBUG feature{idx=0 name="avg_units_by_country"}: Processing feature
2025-11-14T17:00:02Z INFO run_feature_engineering{rows=10000 cols=14}: Feature engineering complete: 14 → 23 columns
2025-11-14T17:00:02Z INFO train_model: Training model with 8 features
2025-11-14T17:00:03Z INFO train_model: Model trained: RMSE=145234.56, R²=0.7845
```

---

## Phase 7: Parallel Processing with Rayon 🎯 **Week 4**

**Goal:** Speed up feature computation using parallelism

### When to Parallelize

Independent features can run in parallel:
```
✅ Can parallelize:
- avg_units_by_country (depends only on raw data)
- total_profit_by_item (depends only on raw data)

❌ Cannot parallelize (has dependencies):
- profit_margin (needs Total Profit and Total Revenue to exist first)
```

### Implementation

Update `src/config/mod.rs`:
```rust
use rayon::prelude::*;

impl PipelineSteps {
    /// Apply independent features in parallel
    pub fn apply_parallel(&self, data: &DataFrame) -> Result<DataFrame, PolarsError> {
        // Group features by dependencies
        let independent = self.get_independent_features();
        let dependent = self.get_dependent_features();

        // Process independent features in parallel
        info!("Processing {} independent features in parallel", independent.len());

        let results: Result<Vec<_>, _> = independent
            .par_iter()
            .map(|feature| {
                debug!("Processing {} (parallel)", feature.name());
                feature.apply_feature(data)
            })
            .collect();

        // Merge parallel results
        let mut df = data.clone();
        for result_df in results? {
            df = self.merge_dataframes(&df, &result_df)?;
        }

        // Process dependent features sequentially
        info!("Processing {} dependent features sequentially", dependent.len());
        for feature in dependent {
            df = feature.apply_feature(&df)?;
        }

        Ok(df)
    }

    fn get_independent_features(&self) -> Vec<&FeatureConfig> {
        // Features that only depend on original columns
        self.steps.iter()
            .filter(|f| !self.has_feature_dependency(f))
            .collect()
    }
}
```

**Benchmark:**
```bash
# Create benches/feature_bench.rs
cargo bench

# Compare:
# - Sequential: 2.5s
# - Parallel: 0.8s (3x speedup!)
```

---

## Phase 8: Benchmarking with Criterion 🎯 **Optimization**

Create `benches/pipeline_bench.rs`:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use features_pipeline::config::PipelineSteps;
use polars::prelude::*;

fn load_test_data() -> DataFrame {
    CsvReader::from_path("data/input/10000 Sales Records.csv")
        .unwrap()
        .finish()
        .unwrap()
}

fn benchmark_features(c: &mut Criterion) {
    let df = load_test_data();
    let config = PipelineSteps::load_from_yaml("config/features/features.yaml").unwrap();

    c.bench_function("apply_all_features", |b| {
        b.iter(|| {
            config.apply(black_box(&df))
        })
    });

    // Benchmark individual features
    for (idx, feature) in config.steps.iter().enumerate() {
        c.bench_with_input(
            BenchmarkId::new("feature", idx),
            &df,
            |b, df| {
                b.iter(|| feature.apply_feature(black_box(df)))
            }
        );
    }
}

criterion_group!(benches, benchmark_features);
criterion_main!(benches);
```

**Run:**
```bash
cargo bench
open target/criterion/report/index.html
```

---

## Complete Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    CLI Entry Point                       │
│              (clap - Phase 5) ✅                        │
└───────────────────┬─────────────────────────────────────┘
                    │
        ┌───────────┴────────────┐
        │                        │
        ▼                        ▼
┌───────────────┐       ┌────────────────┐
│   Features    │       │     Train      │
│   Command     │       │    Command     │
└───────┬───────┘       └────────┬───────┘
        │                        │
        ▼                        ▼
┌──────────────────┐    ┌──────────────────┐
│ Feature Engine   │───▶│  Model Trainer   │
│ (Polars)         │    │  (Linfa/ndarray) │
│ ✅ COMPLETE      │    │  ⏳ PHASE 4      │
└────────┬─────────┘    └─────────┬────────┘
         │                        │
         │  Observability         │
         │  (tracing - Phase 6)   │
         │  ⏳ TODO               │
         │                        │
         ▼                        ▼
    ┌────────────────────────────────┐
    │      Output / Artifacts        │
    │  - Engineered features (CSV)   │
    │  - Trained models (bincode)    │
    │  - Metrics (JSON)              │
    │  - Logs (tracing)              │
    └────────────────────────────────┘
         ▲
         │ Parallelization
         │ (rayon - Phase 7)
         │ ⏳ TODO
```

---

## Timeline Summary

| Phase | What | Duration | Priority |
|-------|------|----------|----------|
| **Phase 4** | Model Training (Linfa) | 1 week | P0 - Start here |
| **Phase 5** | CLI (Clap) | 2-3 days | P1 |
| **Phase 6** | Logging (Tracing) | 2-3 days | P1 |
| **Phase 7** | Parallelism (Rayon) | 2-3 days | P2 |
| **Phase 8** | Benchmarking (Criterion) | 1-2 days | P3 |

**Total:** 3-4 weeks

---

## Resources

### Documentation
- **Linfa Book:** https://rust-ml.github.io/linfa/
- **ndarray Guide:** https://docs.rs/ndarray/latest/ndarray/
- **Clap Tutorial:** https://docs.rs/clap/latest/clap/_derive/
- **Tracing Docs:** https://docs.rs/tracing/latest/tracing/
- **Rayon Guide:** https://github.com/rayon-rs/rayon

### Example Projects
- Linfa examples: https://github.com/rust-ml/linfa/tree/master/algorithms
- Polars + Linfa: https://github.com/pola-rs/polars/discussions

### Community
- r/rust - Reddit
- Rust ML Discord: https://discord.gg/rust-ml
- Linfa GitHub Discussions

---

## Next Action: Start Phase 4

1. Read ndarray basics (30 mins)
2. Create `src/models/mod.rs`
3. Implement `ProfitPredictor::train()`
4. Test with: `cargo run`

**You're building a production ML pipeline in Rust! 🚀**

---

## Phase 9: Experiment Tracking (MLflow-Lite) 🎯 **After Phase 4**

**Timeline:** 3-4 days  
**Goal:** Track experiments like MLflow but file-based (no UI)

### What You'll Build

A lightweight experiment tracking system that logs:
- **Params** - Hyperparameters, config paths
- **Metrics** - RMSE, R², MAE, training time
- **Tags** - Experiment type, dataset version, git commit
- **Artifacts** - Models, predictions, configs

### Implementation

See detailed implementation in **[04-experiment-tracking-mlflow-lite.md](./04-experiment-tracking-mlflow-lite.md)**

Quick preview:

```rust
// Start tracking
let mut run = ExperimentRun::start("feature-pipeline")?;

// Log params
run.log_param("num_features", serde_json::json!(10))?;
run.log_param("target_column", serde_json::json!("Total Profit"))?;

// Log metrics
run.log_metric("rmse", 145234.56, None)?;
run.log_metric("r2", 0.7845, None)?;

// Log tags
run.log_tag("experiment_type", "feature_engineering")?;
run.log_tag("dataset", "sales_records")?;

// Log artifacts
run.log_artifact(Path::new("model.bin"))?;
run.log_artifact(Path::new("features.csv"))?;

// End experiment
run.end(RunStatus::Completed)?;
```

### Output Structure

```
experiments/
└── feature-pipeline/
    └── 20251114_172412/
        ├── metadata.json       # Run info, timestamps
        ├── params.json         # Hyperparameters
        ├── metrics.json        # Performance metrics
        ├── tags.json           # User metadata
        └── artifacts/          # Models, data, configs
            ├── model.bin
            ├── features.csv
            └── config.yaml
```

### Comparison Tool

```bash
cargo run --bin compare-experiments

# Output:
# Run ID               RMSE         R²           Features
# ============================================================
# 20251114_172412      145234.56    0.7845       9
# 20251114_180530      142103.22    0.8012       12
# 20251114_182145      139876.45    0.8156       15
#
# ✨ Best R² score: 20251114_182145 (R² = 0.8156)
```

### Benefits

- ✅ No server setup - pure file-based
- ✅ Git-friendly - commit results with code
- ✅ Human-readable - JSON files
- ✅ Lightweight - no Python deps
- ✅ Extensible - easy to add custom tracking

**Full implementation guide:** [04-experiment-tracking-mlflow-lite.md](./04-experiment-tracking-mlflow-lite.md)

---

## Complete Pipeline Timeline

| Phase | What | Duration | Priority |
|-------|------|----------|----------|
| **Phase 1-3** | Feature Engineering | ✅ Complete | - |
| **Phase 4** | Model Training (Linfa) | 1 week | P0 - Start here |
| **Phase 5** | CLI (Clap) | 2-3 days | P1 |
| **Phase 6** | Logging (Tracing) | 2-3 days | P1 |
| **Phase 7** | Parallelism (Rayon) | 2-3 days | P2 |
| **Phase 8** | Benchmarking (Criterion) | 1-2 days | P3 |
| **Phase 9** | Experiment Tracking | 3-4 days | P1 |

**Total:** 4-5 weeks to complete ML pipeline

---

## Final Architecture (All Phases Complete)

```
┌─────────────────────────────────────────────────────────┐
│                    CLI Entry Point                       │
│              (clap - Phase 5)                           │
└───────────────────┬─────────────────────────────────────┘
                    │
        ┌───────────┴────────────┐
        │                        │
        ▼                        ▼
┌───────────────┐       ┌────────────────┐
│   Features    │       │     Train      │
│   Command     │       │    Command     │
└───────┬───────┘       └────────┬───────┘
        │                        │
        ▼                        ▼
┌──────────────────┐    ┌──────────────────┐
│ Feature Engine   │───▶│  Model Trainer   │
│ (Polars)         │    │  (Linfa/ndarray) │
│ ✅ COMPLETE      │    │  Phase 4         │
└────────┬─────────┘    └─────────┬────────┘
         │                        │
         │  Observability         │
         │  (tracing - Phase 6)   │
         │                        │
         │  Parallelization       │
         │  (rayon - Phase 7)     │
         │                        │
         ▼                        ▼
    ┌────────────────────────────────┐
    │   Experiment Tracking          │
    │   (MLflow-Lite - Phase 9)      │
    │   - Params, Metrics, Tags      │
    │   - Artifacts, Comparison      │
    └────────┬───────────────────────┘
             │
             ▼
    ┌────────────────────────────────┐
    │      Output / Artifacts        │
    │  - Engineered features         │
    │  - Trained models              │
    │  - Experiment tracking files   │
    │  - Metrics & comparisons       │
    └────────────────────────────────┘
```

---

