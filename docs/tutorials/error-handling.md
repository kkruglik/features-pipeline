# Rust Error Handling Reference

## Quick Decision Tree

```
Writing a library?
├─ Yes → Use thiserror or manual enums
└─ No (writing a binary) → Use anyhow or Box<dyn Error>

main() function?
└─ Use Box<dyn Error> or anyhow::Result<()>
```

## Four Error Patterns

### 1. Box<dyn Error> - Quick & Simple

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config("config.yaml")?;
    Ok(())
}
```

**When:** `main()`, prototypes, quick scripts
**Pros:** Works with any error type via `?`
**Cons:** No type information, basic error messages

---

### 2. Manual Enums - Full Control

```rust
#[derive(Debug)]
pub enum ConfigError {
    FileNotFound { path: String, kind: String },
    IoError(std::io::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::FileNotFound { path, kind } =>
                write!(f, "{} file not found: {}", kind, path),
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<std::io::Error> for ConfigError {
    fn from(value: std::io::Error) -> Self {
        ConfigError::IoError(value)
    }
}
```

**When:** Libraries, learning, zero dependencies
**Pros:** Full control, no dependencies
**Cons:** Verbose boilerplate

---

### 3. thiserror - Less Boilerplate

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{kind} file not found: {path}")]
    FileNotFound { path: String, kind: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),  // Auto From impl!

    #[error(transparent)]
    SerdeError(#[from] serde_yaml::Error),
}
```

**When:** Libraries, public APIs
**Pros:** Less verbose than manual, typed errors, auto `From` impls
**Cons:** Small dependency

**Key helpers:**

- `#[error("...")]` - Display message template
- `#[from]` - Auto generates `impl From<T>`
- `#[transparent]` - Forwards wrapped error's Display
- `#[source]` - Marks error cause for `.source()` method

---

### 4. anyhow - Application Error Handling

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<Data> {  // anyhow::Result
    let data = std::fs::read_to_string(path)
        .context(format!("Failed to read {}", path))?;

    let parsed = serde_yaml::from_str(&data)
        .context("Failed to parse YAML")?;

    Ok(parsed)
}
```

**When:** Binaries/applications, not libraries
**Pros:** Rich context, backtraces, easy chaining
**Cons:** Forces dependency on callers, can't pattern match

**Key methods:**

- `.context("msg")` - Add static context
- `.with_context(|| format!(...))` - Add dynamic context (lazy)
- `bail!("error")` - Early return with error
- `ensure!(condition, "msg")` - Assert or error

---

## Combining Patterns (Recommended)

```rust
// src/errors.rs - Domain errors with thiserror
#[derive(Error, Debug)]
pub enum FeatureError {
    #[error("Column '{found}' not found")]
    ColumnNotFound { found: String, available: Vec<String> },

    #[error(transparent)]
    DataframeError(#[from] PolarsError),
}

// src/config.rs - Library functions return typed errors
pub fn load_config(path: &str) -> Result<Config, ConfigError> {
    // ...
}

// src/main.rs - Binary uses anyhow for context
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let config = load_config("config.yaml")
        .context("Failed to load config")?;  // ConfigError → anyhow::Error

    let pipeline = load_pipeline(&config.features)
        .context("Failed to load pipeline")?;  // FeatureError → anyhow::Error

    Ok(())
}
```

---

## Proc Macros & Attributes Explained

### What is `#[...]`?

Attributes - add metadata or generate code at compile time.

### Three Types

```rust
// 1. Derive macro - generates trait implementations
#[derive(Debug, Clone)]
struct Point { x: i32 }

// 2. Attribute macro - transforms code
#[test]
fn test_feature() { }

// 3. Function-like macro
vec![1, 2, 3]
```

### How `#[derive(Error)]` Works

```rust
#[derive(Error, Debug)]  // ← Derive macro
pub enum MyError {
    #[error("Failed: {0}")]  // ← Helper attribute (read by Error derive)
    Failed(String),
}

// Generates:
// - impl Display for MyError
// - impl Error for MyError
// - impl From<T> for marked fields
```

**Key:** `#[error(...)]` is **not** a separate macro. It's configuration data for `#[derive(Error)]`.

### Built-in vs External Derives

```rust
// Built-in (no dependencies)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]

// External (from crates)
#[derive(Error)]          // thiserror
#[derive(Serialize)]      // serde
#[derive(Deserialize)]    // serde
```

---

## Common Patterns

### Pattern 1: Wrapping External Errors

```rust
#[derive(Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Polars(#[from] PolarsError),
}

// Now ? operator auto-converts io::Error and PolarsError to MyError
```

### Pattern 2: Adding Context

```rust
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config file not found: {path}")]
    FileNotFound { path: String },

    #[error("Failed to parse {path}: {error}")]
    ParseError { path: String, error: String },
}

// Use .map_err() to add context
serde_yaml::from_reader(reader)
    .map_err(|e| ConfigError::ParseError {
        path: filepath.to_string(),
        error: e.to_string(),
    })?
```

### Pattern 3: Error Source Chain

```rust
use std::error::Error;

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Polars(e) => Some(e),
            _ => None,
        }
    }
}

// Walk error chain
let mut err: &dyn Error = &my_error;
while let Some(source) = err.source() {
    eprintln!("Caused by: {}", source);
    err = source;
}
```

---

## Quick Reference Table

| Use Case | Pattern | Return Type |
|----------|---------|-------------|
| `main()` | Box or anyhow | `Result<(), Box<dyn Error>>` |
| Library public API | thiserror or manual | `Result<T, MyError>` |
| Internal app logic | anyhow | `anyhow::Result<T>` |
| Prototypes | Box | `Result<T, Box<dyn Error>>` |

---

## Cargo.toml Dependencies

```toml
[dependencies]
thiserror = "2.0"  # For library errors
anyhow = "1.0"     # For application error handling
```

---

## Key Takeaways

1. **Libraries** → Typed errors (thiserror/manual enums)
2. **Binaries** → anyhow for context, Box for simplicity
3. **Never** use `unwrap()` in production - use `?`
4. **Add context** when crossing boundaries (file I/O, parsing, etc.)
5. **Use `#[from]`** to auto-convert wrapped errors
6. **Pattern matching** works on typed errors, not Box/anyhow
