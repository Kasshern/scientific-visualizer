# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Phase 2: Camera & Math system
- Phase 3: Basic data structures & rendering
- Phase 4: UI integration with egui
- Phase 5: Colormap system
- Phase 6: Data loading (CSV, Parquet)
- Phase 7: Performance optimization
- Phase 8: Additional plot types
- Phase 9: WASM support
- Phase 10: Examples & documentation

## [0.1.0] - 2025-10-06

### Added - Phase 1: Foundation & GPU Context ✅

#### Infrastructure
- Workspace structure with 5 crates (viz-core, viz-plots, viz-app, viz-wasm, examples)
- Complete dependency configuration
- MIT License
- `.gitignore` for Rust projects
- Professional documentation (README, ARCHITECTURE, this CHANGELOG)

#### Core Library (viz-core)
- `RenderContext` - GPU device, queue, and surface management
  - Automatic GPU adapter selection with high-performance preference
  - Surface configuration with proper format detection
  - Window resize handling
  - Comprehensive error handling with `RenderError`
- Module structure:
  - `renderer/` - GPU rendering infrastructure
  - `camera/` - Camera systems (stub for Phase 2)
  - `data/` - Data structures (stub for Phase 3)
  - `color/` - Color mapping (stub for Phase 5)
  - `math/` - Math utilities (stub for Phase 2)

#### Examples
- `basic_window` - GPU initialization test
  - Creates window with winit
  - Initializes GPU context
  - Renders clear color (dark blue)
  - Handles events (resize, close)
  - Maintains 60 FPS render loop

#### Development Tools
- Structured logging with tracing
- Workspace optimization profiles
- Example binaries infrastructure

### Technical Details

#### Dependencies
- wgpu 0.19 - WebGPU API (cross-platform GPU)
- winit 0.29 - Window management
- bytemuck 1.14 - Safe GPU buffer casting
- glam 0.25 - Fast SIMD math
- pollster 0.3 - Block on async
- egui 0.26 - Immediate mode GUI (configured)
- arrow 50.0, parquet 50.0 - Columnar data (configured)
- tokio 1.35, rayon 1.8 - Async/parallel (configured)
- anyhow 1.0, thiserror 1.0 - Error handling
- tracing 0.1 - Structured logging

#### Tested Platforms
- ✅ macOS (Darwin 24.6.0)
  - GPU: Apple M1 Max
  - Backend: Metal
  - Surface: 2560×1440, Bgra8UnormSrgb
  - Status: Working perfectly

#### Performance Baseline
- GPU init: ~100ms
- Surface config: ~3ms
- Frame time: <1ms (clear only)
- Memory: ~68MB base

### Architecture Decisions

1. **Arc<Window> Pattern**: Used to satisfy wgpu's Surface<'static> requirement
2. **Modular Crates**: Separated core, plots, app, and wasm for clean dependencies
3. **Error Types**: thiserror for ergonomic error handling
4. **Logging**: tracing for structured, filterable logs

### Code Statistics
- Core implementation: ~250 LOC
- Example code: ~100 LOC
- Documentation: ~450 LOC
- Total: ~800 LOC

### Known Issues
- None - Phase 1 complete with all objectives met

### Migration Notes
- No migrations needed - this is the initial release

---

## Version History

- **0.1.0** (2025-10-06) - Phase 1 Complete: Foundation & GPU Context
- **Next**: 0.2.0 - Phase 2: Camera & Math System

---

## Git Workflow

Starting from Phase 2, development will follow this pattern:
- `main` branch - stable, completed phases
- `phase-N` branches - active development for each phase
- Feature branches - `feature/description` for specific features
- Merge to main only when phase is complete and tested
