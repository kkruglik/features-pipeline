# Project Architecture

## Current Architecture (Phase 3 Complete)

```
┌─────────────────────────────────────────────┐
│            Binary Entry Point               │
│            src/main.rs                      │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│          Configuration Loading              │
│     EntrypointConfig (YAML → Struct)        │
│   - Data path validation                    │
│   - Features config path                    │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│         Feature Engineering                 │
│      PipelineSteps::apply()                 │
│   - Loads DataFrame (Polars)                │
│   - Applies 8 feature transformations       │
│   - Returns enriched DataFrame              │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│           Output Management                 │
│   - Timestamped folders                     │
│   - CSV export with custom delimiter        │
└─────────────────────────────────────────────┘
```

---

## Code Organization

```
src/
├── lib.rs              # Public library API
│                       # Exports: config module
│
├── main.rs             # Binary entry point (56 lines)
│                       # - Loads configs
│                       # - Runs feature pipeline
│                       # - Saves output
│
└── config/
    └── mod.rs          # Feature definitions (321 lines)
                        # - EntrypointConfig
                        # - FeatureConfig enum (8 variants)
                        # - PipelineSteps
                        # - Feature application logic
```

---

## Data Flow

```
Input CSV (14 cols)
      ↓
EntrypointConfig::load_from_yaml()
      ↓
PipelineSteps::load_from_yaml()
      ↓
DataFrame loaded via Polars
      ↓
for feature in steps:
    df = feature.apply_feature(df)
      ↓
Output CSV (23 cols = 14 original + 9 features)
      ↓
Saved to: data/output/{timestamp}/output.csv
```

---

## Module Responsibilities

### `src/main.rs`
**Role:** Application entry point
**Responsibilities:**
- Load entrypoint config
- Create output folder with timestamp
- Run feature pipeline
- Save results to CSV
- Error handling at top level

### `src/config/mod.rs`
**Role:** Feature engineering logic
**Responsibilities:**
- Define config structures
- Parse YAML configs
- Validate configs on load
- Implement feature transformations
- Apply features to DataFrames

### `src/lib.rs`
**Role:** Library interface
**Responsibilities:**
- Export public modules
- Provide reusable API for other binaries

---

## Design Patterns Used

### 1. **Config-Driven Pipeline**
All features defined in YAML, not code:
```yaml
steps:
  - name: avg_units_by_country
    function: mean
    column: Units Sold
    group_by: [Country]
```

### 2. **Tagged Enum for Variants**
Different feature types in single enum:
```rust
#[serde(tag = "function")]
enum FeatureConfig {
    Mean { ... },
    Sum { ... },
    Ratio { ... },
}
```

### 3. **Method on Enum**
Business logic lives with data:
```rust
impl FeatureConfig {
    pub fn apply_feature(&self, data: &DataFrame) -> Result<DataFrame> {
        match self { ... }
    }
}
```

### 4. **Fail-Fast Validation**
Validate on load, not on use:
```rust
impl EntrypointConfig {
    pub fn load_from_yaml(...) -> Result<Self> {
        let config = from_reader(reader)?;
        config.validate()?;  // ← Fail immediately
        Ok(config)
    }
}
```

### 5. **Timestamped Outputs**
Never overwrite previous runs:
```rust
let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
let run_dir = PathBuf::from("data/output").join(timestamp);
```

---

## Future Architecture (Phases 4-9)

```
┌─────────────────────────────────────────────┐
│           CLI Entry (Phase 5)               │
│     features | train | pipeline            │
└──────────────────┬──────────────────────────┘
                   │
        ┌──────────┴──────────┐
        │                     │
        ▼                     ▼
┌───────────────┐    ┌────────────────┐
│   Features    │    │     Train      │
│   (Phase 3✅) │───▶│  (Phase 4⏳)   │
└───────┬───────┘    └────────┬───────┘
        │                     │
        │                     │
        ▼                     ▼
┌──────────────────────────────────────┐
│        Experiment Tracking           │
│         (Phase 9)                    │
│   - Params, Metrics, Tags            │
│   - Artifacts (models, data)         │
└──────────────┬───────────────────────┘
               │
               ▼
      ┌────────────────┐
      │   Observability│
      │  (Phase 6)     │
      │   - Tracing    │
      └────────────────┘
               │
               ▼
      ┌────────────────┐
      │  Parallelism   │
      │  (Phase 7)     │
      │   - Rayon      │
      └────────────────┘
```

---

## Technology Stack

### Production (In Use)
- **Polars** - DataFrame operations
- **Serde** - Config serialization
- **Chrono** - Timestamps

### Ready (Not Yet Used)
- **ndarray** - Numerical arrays (Phase 4)
- **linfa** - ML algorithms (Phase 4)
- **clap** - CLI parsing (Phase 5)
- **tracing** - Logging (Phase 6)
- **rayon** - Parallelism (Phase 7)
- **criterion** - Benchmarking (Phase 8)

---

## Performance Considerations

### Current Performance
- **Dataset:** 10,000 rows × 14 columns
- **Processing time:** ~2-3 seconds
- **Output:** 10,000 rows × 23 columns
- **Features added:** 9

### Optimization Opportunities (Phase 7)
1. Parallelize independent features
2. Use Polars lazy evaluation more
3. Reduce DataFrame clones
4. Stream processing for large datasets

---

## Error Handling Strategy

### Level 1: Config Validation
```rust
EntrypointConfig::validate() -> Result<()>
// Checks: File paths exist
```

### Level 2: Feature Validation
```rust
FeatureConfig::apply_feature() -> Result<DataFrame, PolarsError>
// Checks: Column exists in DataFrame
```

### Level 3: Top-Level Error Propagation
```rust
fn main() -> Result<(), Box<dyn Error>>
// All errors bubble up with ? operator
```

---

## Testing Strategy (Future)

### Unit Tests
```rust
#[test]
fn test_mean_feature() {
    let df = create_test_dataframe();
    let config = FeatureConfig::Mean { ... };
    let result = config.apply_feature(&df).unwrap();
    assert_eq!(result.column("feature_avg").unwrap().len(), 3);
}
```

### Integration Tests
```rust
#[test]
fn test_full_pipeline() {
    let config = PipelineSteps::load_from_yaml("tests/fixtures/test_config.yaml").unwrap();
    let df = load_test_data();
    let result = config.apply(&df).unwrap();
    assert_eq!(result.width(), 23);
}
```

### Benchmarks (Phase 8)
```rust
fn benchmark_features(c: &mut Criterion) {
    c.bench_function("apply_all_features", |b| {
        b.iter(|| config.apply(black_box(&df)))
    });
}
```

---

## Security Considerations

### Input Validation
- ✅ File paths validated on load
- ✅ YAML parsing with safe deserializer
- ✅ No user input execution

### Future Considerations
- Input sanitization for CLI args
- Rate limiting for API endpoints (if added)
- Model file integrity checks
