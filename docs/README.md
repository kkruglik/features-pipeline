# Documentation

Production-ready feature engineering and ML pipeline built with Rust.

---

## 🚀 Quick Start

```bash
# Run feature pipeline
cargo run

# Input:  10,000 rows × 14 columns
# Output: 10,000 rows × 23 columns (9 features added)
```

**Status:** ✅ Feature Engineering Complete (Phase 3)
**Next:** ⏳ Model Training with Linfa (Phase 4)

---

## 📚 Documentation Structure

### **Tutorials** - Learn how to build the pipeline
- **[01-getting-started.md](./tutorials/01-getting-started.md)** - Initial project setup
- **[02-learning-path-phases-4-9.md](./tutorials/02-learning-path-phases-4-9.md)** ⭐ **START HERE**
  - Complete roadmap: Feature engineering → Model training → Production
  - Phase 4: Model Training (Linfa + ndarray)
  - Phase 5: CLI (Clap)
  - Phase 6: Observability (Tracing)
  - Phase 7: Parallelism (Rayon)
  - Phase 8: Benchmarking (Criterion)
  - Phase 9: Experiment Tracking (MLflow-lite)
  - Full code examples for each phase
- **[03-configuration-deep-dive.md](./tutorials/03-configuration-deep-dive.md)** - Config system explained
- **[04-experiment-tracking.md](./tutorials/04-experiment-tracking.md)** 🔬 - MLflow-lite implementation
- **[library-quick-reference.md](./tutorials/library-quick-reference.md)** - Quick reference cheatsheet

### **Project Info** - Project-specific documentation
- **[architecture.md](./project/architecture.md)** - System design, patterns, data flow
- **[current-status.md](./project/current-status.md)** - What's working, metrics, timeline
- **[structure.md](./project/structure.md)** - File organization

---

## 🎯 What to Read First

### If you're new:
1. Read: **[tutorials/02-learning-path-phases-4-9.md](./tutorials/02-learning-path-phases-4-9.md)**
2. Start: Phase 4 - Model Training

### If you want to understand the codebase:
1. **[project/architecture.md](./project/architecture.md)** - How it's built
2. **[project/current-status.md](./project/current-status.md)** - What's done
3. **[project/structure.md](./project/structure.md)** - Where files are

### If you want a specific tutorial:
- **Config system:** [tutorials/03-configuration-deep-dive.md](./tutorials/03-configuration-deep-dive.md)
- **Experiment tracking:** [tutorials/04-experiment-tracking.md](./tutorials/04-experiment-tracking.md)
- **Quick reference:** [tutorials/library-quick-reference.md](./tutorials/library-quick-reference.md)

---

## 🗺️ Learning Path

```
✅ Phase 1-3: Feature Engineering (COMPLETE)
   └─ Polars + Serde + Error Handling
   └─ 8 feature types implemented
   └─ Production-ready (9.5/10)

⏳ Phase 4: Model Training (NEXT - Week 1)
   └─ ndarray + Linfa
   └─ Linear regression
   └─ Metrics evaluation

⏳ Phase 5: CLI Interface (Week 2)
   └─ Clap for command parsing
   └─ Subcommands: features, train, pipeline

⏳ Phase 6: Observability (Week 3)
   └─ Tracing for structured logging
   └─ Performance monitoring

⏳ Phase 7: Parallelism (Week 4)
   └─ Rayon for parallel features
   └─ Performance optimization

⏳ Phase 8: Benchmarking
   └─ Criterion for benchmarks
   └─ Performance analysis

⏳ Phase 9: Experiment Tracking 🔬
   └─ MLflow-lite (file-based)
   └─ Track params, metrics, tags, artifacts
```

**Total:** 4-5 weeks to complete full ML pipeline

---

## 📊 Current Progress

| Component | Status | Quality |
|-----------|--------|---------|
| Data Loading | ✅ Complete | Production-ready |
| Feature Engineering | ✅ Complete | 9.5/10 |
| Config System | ✅ Complete | Production-ready |
| Error Handling | ✅ Complete | Fail-fast validation |
| Output Management | ✅ Complete | Timestamped folders |
| Model Training | ⏳ Next | Not started |
| CLI Interface | ⏳ Future | Not started |
| Observability | ⏳ Future | Not started |
| Experiment Tracking | ⏳ Future | Not started |

---

## 🛠️ Tech Stack

### In Production Use ✅
- **Polars** (0.51.0) - High-performance DataFrames
- **Serde** (1.0) - Config serialization
- **Chrono** (0.4) - Timestamps

### Ready to Use ⏳
- **ndarray** (0.16) - Numerical arrays (Phase 4)
- **linfa** (0.8) - ML algorithms (Phase 4)
- **clap** (4.5) - CLI parsing (Phase 5)
- **tracing** (0.1) - Structured logging (Phase 6)
- **rayon** (1.11) - Data parallelism (Phase 7)
- **criterion** (0.5) - Benchmarking (Phase 8)

---

## 🎓 What You'll Learn

### Already Mastered ✅
- Polars DataFrames (lazy evaluation, aggregations, window functions)
- Serde (derive macros, tagged enums, validation)
- Rust patterns (methods on enums, Result types, module organization)
- Config-driven architecture

### Coming Next ⏳
- **ndarray** - NumPy for Rust
- **Linfa** - scikit-learn for Rust
- **Clap** - Professional CLI tools
- **Tracing** - Structured observability
- **Rayon** - Parallel data processing
- **Criterion** - Performance benchmarking
- **Experiment Tracking** - MLflow-like system

---

## 📖 Documentation Index

### Tutorials (Learning Materials)
```
tutorials/
├── 01-getting-started.md
├── 02-learning-path-phases-4-9.md      ⭐ Main learning path
├── 03-configuration-deep-dive.md
├── 04-experiment-tracking.md           🔬 MLflow-lite
└── library-quick-reference.md
```

### Project Documentation
```
project/
├── architecture.md                     System design
├── current-status.md                   Progress tracking
└── structure.md                        File organization
```

---

## 🚀 Next Steps

1. **Read:** [tutorials/02-learning-path-phases-4-9.md](./tutorials/02-learning-path-phases-4-9.md)
2. **Start:** Phase 4 - Model Training with Linfa
3. **Build:** `src/models/mod.rs` - Train your first Rust ML model

---

## 💡 Quick Reference

```bash
# Run pipeline
cargo run

# Check code quality
cargo clippy

# Format code
cargo fmt

# Run tests (when added)
cargo test

# Build docs
cargo doc --open
```

---

## 🎯 Project Goals

- [x] **Phase 1-3:** Feature engineering pipeline
- [ ] **Phase 4:** Model training (linear regression)
- [ ] **Phase 5:** CLI with subcommands
- [ ] **Phase 6:** Structured logging
- [ ] **Phase 7:** Parallel feature computation
- [ ] **Phase 8:** Performance benchmarking
- [ ] **Phase 9:** Experiment tracking (MLflow-lite)

---

**Ready to continue?** Start with [tutorials/02-learning-path-phases-4-9.md](./tutorials/02-learning-path-phases-4-9.md) 🚀
