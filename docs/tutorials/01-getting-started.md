# Phase 1: Getting Started - Data Loading

Goal: Learn Polars basics and build a simple data loader

## Learning Resources (1-2 days)

### 1. Polars Fundamentals

Start here:

- [Polars User Guide - Introduction](https://pola-rs.github.io/polars/user-guide/getting-started/)
- Focus on:
  - What is a DataFrame and Series
  - Reading CSV files with `CsvReader`
  - Basic operations: `head()`, `describe()`, `shape()`
  - Column selection: `select()`

### 2. Key Concepts to Understand

**DataFrame vs Series:**

```rust
// DataFrame = table with rows and columns
// Series = single column (like a Vec, but with a name and type)
```

**Lazy vs Eager:**

- Eager: Operations execute immediately
- Lazy: Operations build a query plan, execute with `.collect()`
- For learning, start with Eager

**Reading data:**

```rust
use polars::prelude::*;

// Eager reading
let df = CsvReader::from_path("data.csv")?
    .has_header(true)
    .finish()?;

// Lazy reading (optimized)
let df = LazyFrame::scan_csv("data.csv", Default::default())?
    .collect()?;
```

### 3. Practice Exercises

Before building the project, try these in a simple `examples/polars_basics.rs`:

1. Create a DataFrame from scratch
2. Read a CSV file
3. Print first 5 rows
4. Get column names and types
5. Select specific columns
6. Filter rows

## Building Phase (1 day)

### Step 1: Create sample data

Create `data/sample.csv`:

```csv
id,age,income,country,purchased
1,25,50000,US,1
2,35,75000,UK,1
3,28,60000,US,0
4,42,90000,DE,1
5,31,65000,UK,0
```

### Step 2: Create data loader module

Create `src/data/mod.rs`:

```rust
pub mod loader;
```

Create `src/data/loader.rs`:

```rust
use anyhow::{Context, Result};
use polars::prelude::*;
use std::path::Path;

/// Load data from CSV file
pub fn load_csv(path: &Path) -> Result<DataFrame> {
    CsvReader::from_path(path)
        .context("Failed to open CSV file")?
        .has_header(true)
        .finish()
        .context("Failed to parse CSV")
}

/// Load data from Parquet file
pub fn load_parquet(path: &Path) -> Result<DataFrame> {
    ParquetReader::from_path(path)
        .context("Failed to open Parquet file")?
        .finish()
        .context("Failed to parse Parquet")
}

/// Print DataFrame info
pub fn describe_dataframe(df: &DataFrame) {
    println!("Shape: {:?}", df.shape());
    println!("\nColumn names and types:");
    for (name, dtype) in df.get_column_names_owned().iter().zip(df.dtypes()) {
        println!("  {} : {:?}", name, dtype);
    }
    println!("\nFirst 5 rows:");
    println!("{}", df.head(Some(5)));
}
```

### Step 3: Update main.rs

Replace `src/main.rs`:

```rust
mod data;

use anyhow::Result;
use std::path::Path;

fn main() -> Result<()> {
    // Load sample data
    let df = data::loader::load_csv(Path::new("data/sample.csv"))?;

    // Print info
    data::loader::describe_dataframe(&df);

    Ok(())
}
```

### Step 4: Test it

```bash
cargo run
```

You should see:

- Shape: (5, 5)
- Column names and types
- First 5 rows

### Step 5: Add CLI (optional, prepares for Phase 5)

Update `src/main.rs` with clap:

```rust
mod data;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "features-pipeline")]
#[command(about = "Feature engineering and ML pipeline")]
struct Cli {
    /// Path to input CSV file
    #[arg(short, long)]
    input: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let df = data::loader::load_csv(&cli.input)?;
    data::loader::describe_dataframe(&df);

    Ok(())
}
```

Test:

```bash
cargo run -- --input data/sample.csv
```

## Checkpoint

You should now be able to:

- ✅ Load CSV files with Polars
- ✅ Understand DataFrame structure
- ✅ Print basic statistics
- ✅ Use proper error handling with anyhow

## Troubleshooting

**"can't find module data"**

- Make sure `src/data/mod.rs` exists
- Check that `mod data;` is in `main.rs`

**"no such file or directory"**

- Check that `data/sample.csv` exists
- Use absolute path if needed

**Type errors with Result**

- Make sure you have `use anyhow::Result;`
- Add `?` after operations that return Result

## Next Steps

Once comfortable with data loading, move to [Phase 2: Feature Engineering](./02-feature-engineering.md)

## Additional Resources

- [Polars API Documentation](https://docs.rs/polars/latest/polars/)
- [Polars Cookbook](https://pola-rs.github.io/polars-book/user-guide/howcani/)
- [Example projects using Polars](https://github.com/pola-rs/polars/tree/main/examples)
