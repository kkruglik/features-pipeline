# Rayon - Data Parallelism for Rust

Rayon is a data-parallelism library that makes it trivial to convert sequential code into parallel code. Just change `.iter()` to `.par_iter()` and Rayon handles the rest.

## Setup

```toml
# Cargo.toml
[dependencies]
rayon = "1.10"
```

```rust
use rayon::prelude::*;
```

## Basic Example

**Sequential:**
```rust
let sum: i32 = (0..1000).map(|x| x * 2).sum();
```

**Parallel (one character change):**
```rust
use rayon::prelude::*;

let sum: i32 = (0..1000).into_par_iter().map(|x| x * 2).sum();
//                      ^^^^^^^^^^^^^^^ just add this
```

## Core Concepts

### Parallel Iterators

| Sequential | Parallel | Use for |
|------------|----------|---------|
| `.iter()` | `.par_iter()` | Immutable references |
| `.iter_mut()` | `.par_iter_mut()` | Mutable references |
| `.into_iter()` | `.into_par_iter()` | Taking ownership |

### Common Operations

```rust
use rayon::prelude::*;

let numbers: Vec<i32> = (0..10000).collect();

// Map
let doubled: Vec<i32> = numbers.par_iter().map(|x| x * 2).collect();

// Filter
let evens: Vec<i32> = numbers.par_iter().filter(|x| *x % 2 == 0).cloned().collect();

// Find any match (returns first found, not first in order)
let found = numbers.par_iter().find_any(|x| **x > 5000);

// All/Any checks
let all_positive = numbers.par_iter().all(|x| *x >= 0);
let any_negative = numbers.par_iter().any(|x| *x < 0);

// Sum/Reduce
let sum: i32 = numbers.par_iter().sum();
let product: i32 = numbers.par_iter().cloned().reduce(|| 1, |a, b| a * b);

// For each (side effects)
numbers.par_iter().for_each(|x| {
    println!("{}", x);  // order not guaranteed!
});
```

## How It Works

Rayon uses **work-stealing**:

```
┌─────────────────────────────────────────────────┐
│                  Thread Pool                     │
├────────────┬────────────┬────────────┬──────────┤
│  Thread 1  │  Thread 2  │  Thread 3  │ Thread 4 │
│  [██████]  │  [████░░]  │  [done]    │ [done]   │
│            │            │     ↓      │    ↓     │
│            │            │  steals    │ steals   │
│            │            │  work      │ work     │
└────────────┴────────────┴────────────┴──────────┘
```

- Creates a global thread pool (one per CPU core by default)
- Divides work into chunks
- Idle threads steal work from busy threads
- Automatically balances load

## Join: The Core Primitive

For explicit parallelism without iterators:

```rust
use rayon::join;

let (left, right) = rayon::join(
    || expensive_computation_a(),
    || expensive_computation_b(),
);
// Both run in parallel, returns when both complete
```

## Parallel Sorting

```rust
use rayon::prelude::*;

let mut data = vec![5, 2, 8, 1, 9, 3];
data.par_sort();  // parallel sort

// With custom comparator
data.par_sort_by(|a, b| b.cmp(a));  // descending
```

## Example: Parallelize Your Feature Pipeline

**Current (sequential):**
```rust
pub fn apply(&self, data: &DataFrame) -> Result<DataFrame, PipelineStepError> {
    let mut result = data.clone();
    for step in &self.steps {
        result = step.apply_feature(&result)?;  // one at a time
    }
    Ok(result)
}
```

**With Rayon (parallel feature computation):**
```rust
use rayon::prelude::*;

pub fn apply(&self, data: &DataFrame) -> Result<DataFrame, PipelineStepError> {
    // Compute all features in parallel
    let new_columns: Vec<Series> = self.steps
        .par_iter()
        .map(|step| step.compute_column(data))
        .collect::<Result<Vec<_>, _>>()?;

    // Combine into single DataFrame
    let mut result = data.clone();
    for col in new_columns {
        result.with_column(col)?;
    }
    Ok(result)
}
```

**Note:** This only works if features are independent. If feature B depends on feature A's output, they must stay sequential.

## Scope: Borrowing in Parallel

When you need to borrow data across parallel tasks:

```rust
use rayon::scope;

let mut results = vec![0; 10];

rayon::scope(|s| {
    for (i, result) in results.iter_mut().enumerate() {
        s.spawn(move |_| {
            *result = expensive_computation(i);
        });
    }
});
// All spawned tasks complete before scope ends
```

## Thread Pool Configuration

```rust
use rayon::ThreadPoolBuilder;

// Custom thread count
ThreadPoolBuilder::new()
    .num_threads(4)
    .build_global()
    .unwrap();

// Or create a local pool
let pool = ThreadPoolBuilder::new()
    .num_threads(2)
    .build()
    .unwrap();

pool.install(|| {
    // Code here uses this pool instead of global
    (0..100).into_par_iter().for_each(|x| println!("{}", x));
});
```

## When NOT to Use Rayon

1. **Small workloads** - Overhead exceeds benefit
   ```rust
   // Bad: too small
   (0..10).into_par_iter().sum()

   // Good: large enough
   (0..1_000_000).into_par_iter().sum()
   ```

2. **I/O bound work** - Use async (tokio) instead
   ```rust
   // Bad: network calls
   urls.par_iter().map(|url| fetch(url)).collect()

   // Better: use tokio for I/O
   ```

3. **Dependencies between iterations**
   ```rust
   // Bad: each step depends on previous
   let mut acc = 0;
   for x in data {
       acc = process(acc, x);  // can't parallelize
   }
   ```

## Performance Tips

1. **Chunk size matters**
   ```rust
   // Let Rayon decide (usually best)
   data.par_iter().map(process).collect()

   // Manual chunks if needed
   data.par_chunks(1000).map(process_chunk).collect()
   ```

2. **Avoid allocation in hot loops**
   ```rust
   // Bad: allocates per iteration
   data.par_iter().map(|x| x.to_string()).collect()

   // Better: reuse where possible
   ```

3. **Profile first**
   ```bash
   cargo bench  # use criterion to measure actual speedup
   ```

## Safety Guarantees

Rayon guarantees **data-race freedom**. If it compiles, it's safe:

```rust
// This won't compile - Rayon catches the race
let mut sum = 0;
(0..100).into_par_iter().for_each(|x| {
    sum += x;  // ERROR: can't mutate shared state
});

// Correct: use reduce
let sum: i32 = (0..100).into_par_iter().sum();
```

## Quick Reference

```rust
use rayon::prelude::*;

// Parallel iteration
vec.par_iter()           // immutable
vec.par_iter_mut()       // mutable
vec.into_par_iter()      // owned

// Parallel ranges
(0..n).into_par_iter()

// Operations
.map(f)                  // transform
.filter(f)               // filter
.for_each(f)             // side effects
.reduce(identity, f)     // reduce
.sum()                   // sum
.collect()               // collect results

// Join two tasks
rayon::join(|| a(), || b())

// Parallel sort
vec.par_sort()
```

## Resources

- [Official Docs](https://docs.rs/rayon)
- [GitHub](https://github.com/rayon-rs/rayon)
- [Rust Cookbook - Parallelism](https://rust-lang-nursery.github.io/rust-cookbook/concurrency/parallel.html)
- [Shuttle Tutorial (2024)](https://www.shuttle.dev/blog/2024/04/11/using-rayon-rust)
- [Optimization Deep-dive (2024)](https://gendignoux.com/blog/2024/11/18/rust-rayon-optimized.html)
- [Red Hat Blog](https://developers.redhat.com/blog/2021/04/30/how-rust-makes-rayons-data-parallelism-magical)
