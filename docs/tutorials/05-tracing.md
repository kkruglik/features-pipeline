Tracing - Structured Logging for Rust

`tracing` is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information. Unlike simple `println!`, it provides log levels, timestamps, structured data, and spans that show execution flow.

## Setup

```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

## Quick Start

### Basic Initialization

```rust
use tracing::{info, warn, error, debug, trace};
use tracing_subscriber;

fn main() {
    // Initialize default subscriber (prints to stdout)
    tracing_subscriber::fmt::init();

    info!("application started");
    debug!("this only shows with RUST_LOG=debug");
}
```

Run with log level:

```bash
RUST_LOG=debug cargo run
```

### Log Levels

```rust
error!("something went wrong");           // Always shown
warn!("this might be a problem");         // Important warnings
info!("normal operation info");           // Default level
debug!("detailed debugging info");        // Development
trace!("very verbose tracing");           // Maximum detail
```

## Structured Logging

Unlike `println!`, tracing captures structured key-value data:

```rust
// Simple message
info!("processing data");

// With structured fields
info!(rows = 48842, columns = 15, "loaded dataset");

// With variables
let feature_count = 19;
let null_count = 1523;
info!(features = feature_count, nulls = null_count, "feature engineering complete");

// Output:
// 2024-01-22T16:14:29 INFO loaded dataset rows=48842 columns=15
```

## Spans - Track Execution Flow

Spans represent a period of time (not just a point like events):

```rust
use tracing::{span, Level};

fn process_features(data: &DataFrame) {
    let span = span!(Level::INFO, "process_features", rows = data.height());
    let _guard = span.enter();  // span active until _guard dropped

    info!("starting feature processing");
    // ... do work ...
    info!("finished");
}

// Output:
// 2024-01-22T16:14:29 INFO process_features{rows=48842}: starting feature processing
// 2024-01-22T16:14:30 INFO process_features{rows=48842}: finished
```

## The `#[instrument]` Attribute

Automatically create spans for functions:

```rust
use tracing::instrument;

#[instrument]  // logs function entry with all arguments
fn apply_feature(name: &str, data: &DataFrame) -> Result<DataFrame, Error> {
    info!("applying transformation");
    // ...
}

// Output:
// 2024-01-22T16:14:29 INFO apply_feature{name="mean_by_country"}: applying transformation
```

Skip sensitive or large arguments:

```rust
#[instrument(skip(data), fields(rows = data.height()))]
fn apply_feature(name: &str, data: &DataFrame) -> Result<DataFrame, Error> {
    // data not logged, but rows count is
}
```

## JSON Output for Production

```rust
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    tracing_subscriber::fmt()
        .json()  // JSON format
        .with_span_events(FmtSpan::CLOSE)  // log span duration
        .init();
}

// Output:
// {"timestamp":"2024-01-22T16:14:29","level":"INFO","message":"loaded dataset","rows":48842}
```

## Filter by Module

```bash
# Only your crate at debug, dependencies at warn
RUST_LOG=features_pipeline=debug,warn cargo run

# Specific module
RUST_LOG=features_pipeline::pipeline::features=trace cargo run
```

## Example: Instrument Your Pipeline

```rust
use tracing::{info, warn, instrument};

impl FeaturePipeline {
    #[instrument(skip(self, data), fields(steps = self.steps.len()))]
    pub fn apply(&self, data: &DataFrame) -> Result<DataFrame, PipelineStepError> {
        info!(rows = data.height(), "starting feature pipeline");

        let mut result = data.clone();
        for (i, step) in self.steps.iter().enumerate() {
            let step_name = step.name().unwrap_or("ohe");
            info!(step = i + 1, name = step_name, "applying feature");
            result = step.apply_feature(&result)?;
        }

        info!(output_cols = result.width(), "pipeline complete");
        Ok(result)
    }
}
```

## Ecosystem

| Crate | Purpose |
|-------|---------|
| `tracing-subscriber` | Format and output logs |
| `tracing-appender` | Write to files, rotation |
| `tracing-opentelemetry` | Send to Jaeger, Zipkin |
| `tracing-loki` | Send to Grafana Loki |

## Resources

- [Official Docs](https://docs.rs/tracing)
- [Tokio Tracing Guide](https://tokio.rs/tokio/topics/tracing)
- [Shuttle Tutorial (2024)](https://www.shuttle.dev/blog/2024/01/09/getting-started-tracing-rust)
- [GitHub](https://github.com/tokio-rs/tracing)
