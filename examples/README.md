# Examples

This folder contains reference examples for learning the libraries used in this project.

## Running Examples

```bash
# Polars - DataFrame operations
cargo run --example polars_exploration

# Serde - Configuration and serialization
cargo run --example serde_basics

# Run main.rs (clean starting point)
cargo run
```

## What's in polars_exploration.rs

A comprehensive reference with 11 sections:

1. **Basic Operations** - Loading, selecting, null counts
2. **Filtering** - Filter rows by conditions
3. **Expressions & Calculations** - Math operations on columns
4. **Working with Nulls** - Handling missing data
5. **GroupBy Aggregations** - Grouping and aggregating data
6. **Window Functions** - Add aggregated values without changing row count (like pandas transform)
7. **String Operations** - Uppercase, lowercase, contains, length
8. **DateTime Operations** - Extract year, month, day, weekday from dates
9. **Joins** - Combine DataFrames
10. **Saving Data** - Write to CSV and Parquet
11. **Complex Pipeline** - Chaining multiple operations

## Use as Reference

When building your pipeline:
1. Need to do something with strings? → Check section 7
2. Need to join data? → Check section 9
3. Need window functions? → Check section 6

Copy and adapt the code for your needs!

## What's in serde_basics.rs

A comprehensive reference with 10 sections:

1. **Basic JSON Serialization** - Struct → JSON → Struct
2. **JSON File I/O** - Save and load from files
3. **YAML Serialization** - Working with YAML format
4. **Serde Attributes** - `#[serde(rename)]`, `#[serde(default)]`, `#[serde(skip)]`
5. **Nested Structures** - Complex configs with nested objects
6. **Feature Engineering Config** - Real ML pipeline configuration example
7. **Enum-based Configuration** - Type-safe config choices
8. **Dynamic Config with HashMap** - Flexible key-value configs
9. **Error Handling & Validation** - Custom validation after deserialization
10. **Optional Fields** - `Option<T>` and default values

## Use as Configuration Reference

When building your pipeline configs:
1. Need to load YAML config? → Check section 3
2. Need optional fields with defaults? → Check section 10
3. Need to validate loaded config? → Check section 9
4. Need feature definitions? → Check section 6

Copy and adapt for your config files!

## Output Files

Running polars_exploration creates:
- `data/output_example.csv` - CSV export example
- `data/output_example.parquet` - Parquet export example

Running serde_basics creates:
- `data/example_config.json` - JSON config example
- `data/example_config.yaml` - YAML config example
- `data/pipeline_config.yaml` - Pipeline configuration
- `data/features_config.yaml` - Feature definitions
