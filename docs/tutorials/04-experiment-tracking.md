# Experiment Tracking: MLflow-Lite Implementation

**Goal:** Build a lightweight experiment tracking system (MLflow-lite) without UI, tracking params, metrics, tags, and artifacts.

---

## Overview

MLflow-lite is a file-based experiment tracking system for your ML pipeline. Unlike MLflow, it has:
- ✅ **No server/UI** - Pure file-based tracking
- ✅ **Lightweight** - Just Rust structs + Serde
- ✅ **Git-friendly** - JSON/YAML files you can commit
- ✅ **Integration** - Works with your existing pipeline

---

## Architecture

```
experiments/
├── 20251114_172412/              # Run ID (timestamp)
│   ├── metadata.json             # Run info, timestamps, status
│   ├── params.json               # Hyperparameters, config paths
│   ├── metrics.json              # Metrics (RMSE, R², etc.)
│   ├── tags.json                 # User metadata (experiment_type, etc.)
│   └── artifacts/                # Saved models, data, configs
│       ├── model.bin
│       ├── features.csv
│       ├── predictions.csv
│       └── config.yaml (copy)
│
└── experiments.json              # Index of all runs
```

---

## Phase 9: Experiment Tracking 🎯 **After Phase 4**

**Timeline:** 3-4 days
**Prerequisites:** Phase 4 complete (model training working)

---

## Implementation

### Step 1: Create Tracking Structs

Create `src/tracking/mod.rs`:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Experiment run tracker
#[derive(Serialize, Deserialize, Debug)]
pub struct ExperimentRun {
    pub run_id: String,
    pub run_dir: PathBuf,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: RunStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RunStatus {
    Running,
    Completed,
    Failed,
}

impl ExperimentRun {
    /// Start a new experiment run
    pub fn start(experiment_name: &str) -> Result<Self, Box<dyn Error>> {
        let run_id = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let run_dir = PathBuf::from("experiments")
            .join(experiment_name)
            .join(&run_id);

        fs::create_dir_all(&run_dir)?;
        fs::create_dir_all(run_dir.join("artifacts"))?;

        let run = Self {
            run_id: run_id.clone(),
            run_dir,
            start_time: Utc::now(),
            end_time: None,
            status: RunStatus::Running,
        };

        // Save metadata
        run.save_metadata()?;

        println!("🔬 Started experiment run: {}", run_id);
        Ok(run)
    }

    /// Log a parameter (hyperparameter, config value)
    pub fn log_param(&self, key: &str, value: serde_json::Value) -> Result<(), Box<dyn Error>> {
        let params_path = self.run_dir.join("params.json");

        let mut params: HashMap<String, serde_json::Value> = if params_path.exists() {
            let file = File::open(&params_path)?;
            serde_json::from_reader(file)?
        } else {
            HashMap::new()
        };

        params.insert(key.to_string(), value);

        let file = File::create(&params_path)?;
        serde_json::to_writer_pretty(file, &params)?;

        Ok(())
    }

    /// Log a metric (RMSE, R², accuracy, etc.)
    pub fn log_metric(&self, key: &str, value: f64, step: Option<usize>) -> Result<(), Box<dyn Error>> {
        let metrics_path = self.run_dir.join("metrics.json");

        let mut metrics: HashMap<String, Vec<MetricValue>> = if metrics_path.exists() {
            let file = File::open(&metrics_path)?;
            serde_json::from_reader(file)?
        } else {
            HashMap::new()
        };

        let metric_value = MetricValue {
            value,
            timestamp: Utc::now(),
            step,
        };

        metrics.entry(key.to_string())
            .or_insert_with(Vec::new)
            .push(metric_value);

        let file = File::create(&metrics_path)?;
        serde_json::to_writer_pretty(file, &metrics)?;

        println!("  📊 {} = {:.4}", key, value);
        Ok(())
    }

    /// Log a tag (metadata: experiment_type, dataset_version, etc.)
    pub fn log_tag(&self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let tags_path = self.run_dir.join("tags.json");

        let mut tags: HashMap<String, String> = if tags_path.exists() {
            let file = File::open(&tags_path)?;
            serde_json::from_reader(file)?
        } else {
            HashMap::new()
        };

        tags.insert(key.to_string(), value.to_string());

        let file = File::create(&tags_path)?;
        serde_json::to_writer_pretty(file, &tags)?;

        Ok(())
    }

    /// Log an artifact (file: model, dataset, config, etc.)
    pub fn log_artifact(&self, file_path: &Path) -> Result<(), Box<dyn Error>> {
        let file_name = file_path.file_name()
            .ok_or("Invalid file path")?;

        let dest = self.run_dir.join("artifacts").join(file_name);
        fs::copy(file_path, &dest)?;

        println!("  💾 Saved artifact: {}", file_name.to_string_lossy());
        Ok(())
    }

    /// Log multiple params at once
    pub fn log_params(&self, params: HashMap<String, serde_json::Value>) -> Result<(), Box<dyn Error>> {
        for (key, value) in params {
            self.log_param(&key, value)?;
        }
        Ok(())
    }

    /// Log multiple metrics at once
    pub fn log_metrics(&self, metrics: HashMap<String, f64>) -> Result<(), Box<dyn Error>> {
        for (key, value) in metrics {
            self.log_metric(&key, value, None)?;
        }
        Ok(())
    }

    /// End experiment run
    pub fn end(&mut self, status: RunStatus) -> Result<(), Box<dyn Error>> {
        self.end_time = Some(Utc::now());
        self.status = status;
        self.save_metadata()?;

        let duration = self.end_time.unwrap() - self.start_time;
        println!("✅ Experiment completed in {:.2}s", duration.num_milliseconds() as f64 / 1000.0);

        Ok(())
    }

    fn save_metadata(&self) -> Result<(), Box<dyn Error>> {
        let metadata_path = self.run_dir.join("metadata.json");
        let file = File::create(&metadata_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricValue {
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub step: Option<usize>,
}
```

---

### Step 2: Integrate with Pipeline

Update `src/lib.rs`:
```rust
pub mod config;
pub mod models;
pub mod tracking;  // Add this
```

Update `src/main.rs` to track experiments:

```rust
use features_pipeline::tracking::ExperimentRun;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    // Start experiment tracking
    let mut run = ExperimentRun::start("feature-pipeline")?;

    // Log tags
    run.log_tag("experiment_type", "feature_engineering")?;
    run.log_tag("dataset", "sales_records")?;
    run.log_tag("version", env!("CARGO_PKG_VERSION"))?;

    // Load configs
    let entrypoint_config = EntrypointConfig::load_from_yaml("config/entrypoint.yaml")?;

    // Log params
    run.log_param("data_path", serde_json::json!(entrypoint_config.data))?;
    run.log_param("features_config", serde_json::json!(entrypoint_config.features))?;

    // Feature engineering
    println!("\n=== Feature Engineering ===");
    let features_pipeline = PipelineSteps::load_from_yaml(&entrypoint_config.features)?;

    run.log_param("num_features", serde_json::json!(features_pipeline.steps.len()))?;

    let csv_file = File::open(&entrypoint_config.data)?;
    let mut df = CsvReader::new(csv_file).finish()?;

    let original_cols = df.width();

    df = features_pipeline.apply(&df)?;

    let new_cols = df.width();
    let features_added = new_cols - original_cols;

    // Log metrics
    run.log_metric("original_columns", original_cols as f64, None)?;
    run.log_metric("final_columns", new_cols as f64, None)?;
    run.log_metric("features_added", features_added as f64, None)?;
    run.log_metric("num_rows", df.height() as f64, None)?;

    // Save output
    let run_dir = create_run_folder()?;
    let output_path = run_dir.join("output.csv");
    let output_file = File::create(&output_path)?;

    CsvWriter::new(&output_file)
        .include_header(true)
        .with_separator(b';')
        .finish(&mut df)?;

    // Log artifacts
    run.log_artifact(&output_path)?;
    run.log_artifact(Path::new("config/entrypoint.yaml"))?;
    run.log_artifact(Path::new(&entrypoint_config.features))?;

    // Model training (Phase 4)
    if let Ok(model_config_file) = File::open("config/model/model.yaml") {
        println!("\n=== Model Training ===");
        run.log_tag("phase", "training")?;

        let model_config: ModelConfig = serde_yaml::from_reader(model_config_file)?;

        // Log model params
        run.log_param("target_column", serde_json::json!(model_config.target_column))?;
        run.log_param("feature_columns", serde_json::json!(model_config.feature_columns))?;
        run.log_param("num_training_features", serde_json::json!(model_config.feature_columns.len()))?;

        let predictor = ProfitPredictor::train(&df, &model_config)?;

        // Evaluate and log metrics
        let metrics = predictor.evaluate(&df, &model_config.target_column)?;

        run.log_metric("rmse", metrics.rmse, None)?;
        run.log_metric("mae", metrics.mae, None)?;
        run.log_metric("r2", metrics.r2, None)?;

        // Save model
        let model_path = run_dir.join("model.bin");
        predictor.save(&model_path)?;
        run.log_artifact(&model_path)?;
    }

    // End experiment
    run.end(RunStatus::Completed)?;

    Ok(())
}
```

---

### Step 3: Example Output

After running `cargo run`, you'll have:

```
experiments/
└── feature-pipeline/
    └── 20251114_172412/
        ├── metadata.json
        ├── params.json
        ├── metrics.json
        ├── tags.json
        └── artifacts/
            ├── output.csv
            ├── model.bin
            ├── entrypoint.yaml
            └── features.yaml
```

**metadata.json:**
```json
{
  "run_id": "20251114_172412",
  "run_dir": "experiments/feature-pipeline/20251114_172412",
  "start_time": "2025-11-14T17:24:12.123456Z",
  "end_time": "2025-11-14T17:24:45.789012Z",
  "status": "Completed"
}
```

**params.json:**
```json
{
  "data_path": "data/input/10000 Sales Records.csv",
  "features_config": "config/features/features.yaml",
  "num_features": 10,
  "target_column": "Total Profit",
  "feature_columns": ["feature_avg_units_by_country", ...],
  "num_training_features": 8
}
```

**metrics.json:**
```json
{
  "original_columns": [{
    "value": 14.0,
    "timestamp": "2025-11-14T17:24:15.123Z",
    "step": null
  }],
  "final_columns": [{
    "value": 23.0,
    "timestamp": "2025-11-14T17:24:20.456Z",
    "step": null
  }],
  "features_added": [{
    "value": 9.0,
    "timestamp": "2025-11-14T17:24:20.457Z",
    "step": null
  }],
  "rmse": [{
    "value": 145234.56,
    "timestamp": "2025-11-14T17:24:40.123Z",
    "step": null
  }],
  "r2": [{
    "value": 0.7845,
    "timestamp": "2025-11-14T17:24:40.124Z",
    "step": null
  }]
}
```

**tags.json:**
```json
{
  "experiment_type": "feature_engineering",
  "dataset": "sales_records",
  "version": "0.1.0",
  "phase": "training"
}
```

---

## Step 4: Experiment Comparison Tool

Create `src/bin/compare_experiments.rs`:

```rust
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let experiments_dir = Path::new("experiments/feature-pipeline");

    if !experiments_dir.exists() {
        println!("No experiments found");
        return Ok(());
    }

    println!("📊 Experiment Comparison\n");
    println!("{:<20} {:<12} {:<12} {:<12}", "Run ID", "RMSE", "R²", "Features");
    println!("{}", "=".repeat(60));

    let mut runs = Vec::new();

    for entry in fs::read_dir(experiments_dir)? {
        let entry = entry?;
        let run_dir = entry.path();

        if !run_dir.is_dir() {
            continue;
        }

        let run_id = run_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Read metrics
        let metrics_path = run_dir.join("metrics.json");
        if !metrics_path.exists() {
            continue;
        }

        let metrics_file = fs::File::open(&metrics_path)?;
        let metrics: HashMap<String, Vec<Value>> = serde_json::from_reader(metrics_file)?;

        let rmse = metrics.get("rmse")
            .and_then(|v| v.last())
            .and_then(|v| v.get("value"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let r2 = metrics.get("r2")
            .and_then(|v| v.last())
            .and_then(|v| v.get("value"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let features = metrics.get("features_added")
            .and_then(|v| v.last())
            .and_then(|v| v.get("value"))
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        println!("{:<20} {:<12.2} {:<12.4} {:<12.0}",
                 run_id, rmse, r2, features);

        runs.push((run_id.to_string(), rmse, r2));
    }

    if !runs.is_empty() {
        println!("\n✨ Best R² score:");
        let best = runs.iter().max_by(|a, b| a.2.partial_cmp(&b.2).unwrap()).unwrap();
        println!("   Run: {} (R² = {:.4})", best.0, best.2);
    }

    Ok(())
}
```

**Add to Cargo.toml:**
```toml
[[bin]]
name = "compare-experiments"
path = "src/bin/compare_experiments.rs"
```

**Usage:**
```bash
cargo run --bin compare-experiments

# Output:
# 📊 Experiment Comparison
#
# Run ID               RMSE         R²           Features
# ============================================================
# 20251114_172412      145234.56    0.7845       9
# 20251114_180530      142103.22    0.8012       12
# 20251114_182145      139876.45    0.8156       15
#
# ✨ Best R² score:
#    Run: 20251114_182145 (R² = 0.8156)
```

---

## Advanced Features (Optional)

### 1. Git Integration
```rust
use std::process::Command;

impl ExperimentRun {
    pub fn log_git_info(&self) -> Result<(), Box<dyn Error>> {
        // Get current commit hash
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()?;

        let commit = String::from_utf8(output.stdout)?
            .trim()
            .to_string();

        self.log_tag("git_commit", &commit)?;

        // Check if repo is dirty
        let status_output = Command::new("git")
            .args(&["status", "--porcelain"])
            .output()?;

        let is_dirty = !status_output.stdout.is_empty();
        self.log_tag("git_dirty", &is_dirty.to_string())?;

        Ok(())
    }
}
```

### 2. System Info Logging
```rust
use std::env;

impl ExperimentRun {
    pub fn log_system_info(&self) -> Result<(), Box<dyn Error>> {
        self.log_tag("os", env::consts::OS)?;
        self.log_tag("arch", env::consts::ARCH)?;

        // Number of CPUs
        let num_cpus = num_cpus::get();
        self.log_param("num_cpus", serde_json::json!(num_cpus))?;

        Ok(())
    }
}
```

### 3. Query API
```rust
// src/tracking/query.rs

pub struct ExperimentQuery;

impl ExperimentQuery {
    /// List all runs
    pub fn list_runs(experiment_name: &str) -> Result<Vec<ExperimentRun>, Box<dyn Error>> {
        // Implementation
    }

    /// Find best run by metric
    pub fn find_best_run(experiment_name: &str, metric: &str, maximize: bool)
        -> Result<Option<ExperimentRun>, Box<dyn Error>> {
        // Implementation
    }

    /// Compare runs
    pub fn compare_runs(run_ids: &[String]) -> Result<ComparisonTable, Box<dyn Error>> {
        // Implementation
    }
}
```

---

## Integration Timeline

| When | What to Track | Phase |
|------|---------------|-------|
| **Phase 4** | Model metrics (RMSE, R², MAE) | Model training |
| **Phase 5** | CLI args, execution time | CLI |
| **Phase 6** | Tracing integration | Observability |
| **Phase 7** | Parallel execution stats | Parallelism |
| **Phase 8** | Benchmark results | Performance |

---

## Comparison with MLflow

| Feature | MLflow | Your MLflow-Lite | Status |
|---------|--------|------------------|--------|
| UI/Server | ✅ | ❌ | File-based only |
| Params tracking | ✅ | ✅ | Phase 9 |
| Metrics tracking | ✅ | ✅ | Phase 9 |
| Tags | ✅ | ✅ | Phase 9 |
| Artifacts | ✅ | ✅ | Phase 9 |
| Model registry | ✅ | ⏳ | Future |
| Comparison tool | ✅ | ✅ | Phase 9 |
| Git integration | ✅ | ✅ | Optional |
| Remote storage | ✅ | ❌ | Local only |

---

## Benefits of This Approach

1. **Simple** - No server setup, just files
2. **Git-friendly** - Commit experiment results with code
3. **Portable** - Copy experiment dir anywhere
4. **Auditable** - JSON files are human-readable
5. **Extensible** - Easy to add custom tracking
6. **Lightweight** - Pure Rust, no Python deps

---

## Next Steps

1. ✅ Complete Phase 4 (Model Training)
2. ⏳ Implement basic experiment tracking
3. ⏳ Add comparison tool
4. ⏳ Integrate with all phases

---

**This gives you professional experiment tracking without the complexity of MLflow!** 🎯
