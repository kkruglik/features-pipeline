# Criterion - Benchmarking for Rust

Criterion is a statistics-driven micro-benchmarking library. It provides accurate measurements, detects performance regressions, and generates HTML reports. It's the de facto standard for Rust benchmarking.

## Setup

```toml
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "feature_pipeline"
harness = false  # disable built-in benchmark harness
```

Create directory:
```bash
mkdir benches
```

## Basic Benchmark

Create `benches/feature_pipeline.rs`:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

Run:
```bash
cargo bench
```

Output:
```
fib 20                  time:   [26.029 µs 26.251 µs 26.505 µs]
```

## Understanding the Output

```
feature pipeline        time:   [245.32 ms 248.17 ms 251.43 ms]
                              ──────────  ────────── ──────────
                              lower bound  estimate   upper bound
                              (95% CI)

change: [-12.4% -10.2% -8.1%] (p = 0.00 < 0.05)
        Performance has improved.
```

- **time**: Statistical estimate with confidence interval
- **change**: Comparison to previous run
- **p-value**: Statistical significance (< 0.05 = significant)

## `black_box` - Prevent Compiler Optimization

The compiler might optimize away unused results. `black_box` prevents this:

```rust
// BAD: compiler might skip the computation
b.iter(|| fibonacci(20));

// GOOD: forces computation to happen
b.iter(|| black_box(fibonacci(black_box(20))));
```

## Benchmark Your Feature Pipeline

```rust
// benches/feature_pipeline.rs
use criterion::{criterion_group, criterion_main, Criterion};
use polars::prelude::*;
use features_pipeline::pipeline::features::FeaturePipeline;

fn load_test_data() -> DataFrame {
    let file = std::fs::File::open("data/input/adult.csv").unwrap();
    CsvReader::new(file).finish().unwrap()
}

fn bench_feature_pipeline(c: &mut Criterion) {
    let df = load_test_data();
    let pipeline = FeaturePipeline::from_yaml("config/features/adult.yaml").unwrap();

    c.bench_function("full pipeline", |b| {
        b.iter(|| pipeline.apply(&df))
    });
}

fn bench_individual_features(c: &mut Criterion) {
    let df = load_test_data();
    let pipeline = FeaturePipeline::from_yaml("config/features/adult.yaml").unwrap();

    let mut group = c.benchmark_group("individual features");

    for (i, step) in pipeline.steps.iter().enumerate() {
        let name = step.name().unwrap_or("ohe");
        group.bench_function(name, |b| {
            b.iter(|| step.apply_feature(&df))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_feature_pipeline, bench_individual_features);
criterion_main!(benches);
```

## Customizing Benchmarks

```rust
use std::time::Duration;

fn bench_with_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("configured");

    // Customize timing
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    group.bench_function("my_function", |b| {
        b.iter(|| expensive_operation())
    });

    group.finish();
}
```

## Comparing Implementations

```rust
fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("implementations");
    let data = load_test_data();

    group.bench_function("sequential", |b| {
        b.iter(|| apply_sequential(&data))
    });

    group.bench_function("parallel_rayon", |b| {
        b.iter(|| apply_parallel(&data))
    });

    group.finish();
}
```

## Parameterized Benchmarks

Test with different input sizes:

```rust
fn bench_with_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("varying_size");

    for size in [1000, 10000, 50000].iter() {
        let data = generate_data(*size);

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &data,
            |b, data| {
                b.iter(|| process(data))
            },
        );
    }

    group.finish();
}
```

## HTML Reports

After running `cargo bench`, find reports at:
```
target/criterion/report/index.html
```

Features:
- Performance over time graphs
- Violin plots showing distribution
- Comparison between runs
- Regression detection

## Baseline Comparisons

Save a baseline:
```bash
cargo bench -- --save-baseline main
```

Compare against baseline:
```bash
cargo bench -- --baseline main
```

## Tips

1. **Run on quiet system** - Close other apps, disable turbo boost for consistent results

2. **Use release mode** - Benchmarks automatically use `--release`

3. **Warm up** - Criterion warms up by default, ensuring CPU caches are hot

4. **Check statistical significance** - Ignore changes where p > 0.05

5. **Benchmark realistic data** - Use production-like data sizes

## Example Output After Optimization

```
full pipeline           time:   [245.32 ms 248.17 ms 251.43 ms]
                        change: [-12.4% -10.2% -8.1%] (p = 0.00 < 0.05)
                        Performance has improved.

individual features/mean_by_country
                        time:   [12.429 ms 12.512 ms 12.604 ms]
individual features/ohe
                        time:   [89.234 ms 90.112 ms 91.043 ms]
                        ^^^^^^^ this is the bottleneck!
```

## Resources

- [Official Documentation](https://bheisler.github.io/criterion.rs/book/)
- [API Docs](https://docs.rs/criterion/latest/criterion/)
- [GitHub](https://github.com/bheisler/criterion.rs)
- [Rustfinity Tutorial (2024)](https://www.rustfinity.com/blog/rust-benchmarking-with-criterion)
- [Bencher Guide](https://bencher.dev/learn/benchmarking/rust/criterion/)
