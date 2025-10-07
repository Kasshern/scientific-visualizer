# Contributing to Scientific Visualizer

Thank you for your interest in contributing to Scientific Visualizer! This document provides guidelines and instructions for contributing to the project.

## Project Status

This is currently a **portfolio/learning project** in active development. We're building this in phases:

- ✅ **Phase 1** (Complete): Foundation & GPU Context
- 🚧 **Phase 2** (Next): Camera & Math System
- 📋 **Phase 3-10**: See [ROADMAP.md](docs/ROADMAP.md)

## Development Philosophy

### Code Quality Standards

1. **Performance First**: This is a high-performance visualization library
   - Target: 1M+ points @ 60 FPS
   - Profile before optimizing, but optimize aggressively
   - Use GPU for heavy lifting

2. **Production-Grade Code**
   - This is NOT a learning project - write production code
   - Comprehensive error handling (no unwraps in production code)
   - Full documentation for all public APIs
   - Tests for all major features

3. **Clean Architecture**
   - Modular design with clear separation of concerns
   - Follow Rust best practices and idioms
   - Prefer explicit over implicit
   - Zero unsafe code (except where required by GPU interop)

## Getting Started

### Prerequisites

- Rust 1.75+ (latest stable)
- A GPU (for testing)
- Git

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/scientific-visualizer
cd scientific-visualizer

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace

# Run example
cargo run -p examples --bin basic_window
```

### Development Tools

Recommended tools:
```bash
# Auto-rebuild on changes
cargo install cargo-watch
cargo watch -x check

# Linting
cargo clippy --workspace

# Code formatting
cargo fmt --all

# Documentation
cargo doc --open --workspace
```

## Git Workflow

### Branch Strategy

- **`main`**: Stable, completed phases only
- **`phase-N`**: Active development for Phase N
- **`feature/description`**: Specific features

### Workflow for Each Phase

1. **Create phase branch from main**:
   ```bash
   git checkout main
   git pull
   git checkout -b phase-2
   ```

2. **Develop with small, focused commits**:
   ```bash
   git add -p  # Review changes before committing
   git commit -m "feat(camera): implement orbital camera view matrix"
   ```

3. **Merge to main when phase complete**:
   ```bash
   # Ensure all tests pass
   cargo test --workspace
   cargo clippy --workspace

   # Update CHANGELOG.md
   # Update README.md status

   git checkout main
   git merge phase-2
   git push
   ```

### Commit Message Format

Follow conventional commits:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding/updating tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(renderer): add GPU instancing for scatter plots
fix(camera): correct aspect ratio calculation
docs(api): add examples for PointCloud API
perf(scatter): optimize vertex buffer updates
```

## Code Style

### Rust Guidelines

1. **Follow Rust 2021 edition idioms**
2. **Run `cargo fmt` before committing**
3. **Fix all `cargo clippy` warnings**
4. **Document all public APIs with rustdoc**

### Documentation Style

```rust
/// Renders a 3D scatter plot with GPU acceleration.
///
/// # Performance
/// Can render 1M+ points at 60 FPS using GPU instancing.
///
/// # Examples
/// ```
/// use viz_core::RenderContext;
/// use viz_plots::Scatter3D;
///
/// let scatter = Scatter3D::new(&context, point_cloud)?;
/// scatter.render(&mut render_pass);
/// ```
///
/// # Errors
/// Returns `RenderError::OutOfMemory` if GPU allocation fails.
pub fn render(&self, render_pass: &mut wgpu::RenderPass) -> Result<()> {
    // Implementation
}
```

### Error Handling

```rust
// ✅ Good - use thiserror for custom errors
#[derive(Debug, thiserror::Error)]
pub enum PlotError {
    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Render error: {0}")]
    RenderError(#[from] RenderError),
}

// ✅ Good - propagate errors with ?
pub fn load_data(path: &Path) -> Result<PointCloud, DataError> {
    let file = File::open(path)?;
    let data = parse_csv(file)?;
    Ok(PointCloud::new(data))
}

// ❌ Bad - don't use unwrap in library code
pub fn bad_example(path: &Path) -> PointCloud {
    let file = File::open(path).unwrap(); // DON'T DO THIS
    // ...
}
```

### Performance Guidelines

```rust
// ✅ Good - minimize allocations
pub fn process_data(&self, points: &[Vec3]) -> Vec<Vec4> {
    points.iter()
        .map(|p| self.transform(p))
        .collect()
}

// ✅ Good - use rayon for parallel processing
use rayon::prelude::*;
pub fn parallel_process(&self, points: &[Vec3]) -> Vec<Vec4> {
    points.par_iter()
        .map(|p| self.transform(p))
        .collect()
}

// ❌ Bad - unnecessary clones
pub fn bad_example(&self, points: &[Vec3]) -> Vec<Vec4> {
    let cloned = points.to_vec(); // Unnecessary allocation
    cloned.iter().map(|p| self.transform(p)).collect()
}
```

## Testing

### Test Requirements

1. **Unit tests** for all core functionality
2. **Integration tests** for rendering pipeline
3. **Benchmarks** for performance-critical code
4. **Examples** that demonstrate features

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_camera_view_matrix() {
        let camera = OrbitalCamera::new(Vec3::ZERO, 10.0);
        let view = camera.view_matrix();

        // Test properties of view matrix
        assert_relative_eq!(view.determinant(), 1.0, epsilon = 1e-6);
    }
}
```

### Benchmarking

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_scatter_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("scatter");

    group.bench_function("1M points", |b| {
        b.iter(|| {
            // Benchmark code
        });
    });

    group.finish();
}

criterion_group!(benches, benchmark_scatter_render);
criterion_main!(benches);
```

## Documentation

### Documentation Requirements

1. **README.md** - Keep updated with current status
2. **CHANGELOG.md** - Document all changes
3. **API docs** - Rustdoc for all public items
4. **Architecture docs** - Update when design changes
5. **Examples** - One example per major feature

### Building Documentation

```bash
# Generate and view docs
cargo doc --open --workspace --no-deps

# Check for missing docs
cargo rustdoc -- -D missing_docs
```

## Pull Request Process

1. **Create feature branch**
   ```bash
   git checkout -b feature/my-feature
   ```

2. **Make changes with tests**
   - Write code
   - Add tests
   - Update documentation
   - Add example if needed

3. **Ensure quality**
   ```bash
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   cargo fmt --all -- --check
   ```

4. **Update CHANGELOG.md**

5. **Submit PR with description**
   - What does this change?
   - Why is it needed?
   - How was it tested?

6. **Address review feedback**

## Performance Benchmarking

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench --workspace

# Run specific benchmark
cargo bench --bench rendering_perf

# Generate flamegraph (requires cargo-flamegraph)
cargo flamegraph --bench rendering_perf
```

### Performance Targets

- Frame time: <10ms (60+ FPS)
- 1M points: 60 FPS
- 10M points: 30 FPS
- GPU memory: <2GB for 10M points

## Questions?

- Check [README.md](README.md) for overview
- Read [ARCHITECTURE.md](docs/ARCHITECTURE.md) for design
- See [ROADMAP.md](docs/ROADMAP.md) for project plan
- Open an issue for questions

## Code of Conduct

Be respectful and constructive. This is a learning-focused project - help others learn and grow.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
