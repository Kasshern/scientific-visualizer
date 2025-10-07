# Scientific Visualizer

A high-performance scientific data visualization engine in Rust with GPU acceleration, cross-platform support (native + WASM), and real-time rendering capabilities.

## Status: Phase 2 Complete ✓

**Camera & Math Systems** - Orbital camera with full controls, bounding boxes, and transformation utilities.

## Features (Planned)

- 🚀 Real-time 3D scatter plots (target: 1M+ points @ 60 FPS)
- 📊 2D line charts with streaming data support
- 🔥 Heatmaps and density visualizations
- 🌐 Network/graph visualizations with force-directed layouts
- 🎨 Beautiful UI with immediate-mode GUI (egui)
- 🌍 Cross-platform: Linux, macOS, Windows, Web (WASM)
- ⚡ GPU-accelerated rendering with WebGPU (wgpu)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/scientific-visualizer
cd scientific-visualizer

# Run the basic window example
cargo run -p examples --bin basic_window

# Run the interactive camera example
cargo run -p examples --bin camera_movement
```

## Project Structure

```
scientific-visualizer/
├── crates/
│   ├── viz-core/      # Core visualization library
│   ├── viz-plots/     # Plot implementations
│   ├── viz-app/       # Desktop application
│   ├── viz-wasm/      # Web version
│   └── examples/      # Example binaries
├── shaders/           # WGSL shaders
├── examples/          # Example code
└── docs/             # Documentation
```

## Technology Stack

- **Graphics**: wgpu 0.19 (WebGPU), winit 0.29
- **Math**: glam 0.25, nalgebra 0.32
- **UI**: egui 0.26
- **Data**: arrow 50.0, parquet 50.0, ndarray 0.15
- **Async**: tokio 1.35, rayon 1.8

## Phase 2 Complete ✓

- [x] Bounds3D (AABB) implementation
- [x] Transform utilities (TRS matrices)
- [x] OrbitalCamera with full 3D controls
- [x] Mouse input (rotate, pan, zoom)
- [x] Interactive camera_movement example
- [x] 23 tests passing
- [x] 120 FPS performance

**Phase 1**: ✅ GPU Context & Foundation
**Phase 2**: ✅ Camera & Math Systems

**Tested On**: Apple M1 Max (Metal backend)

## Next: Phase 3 - Basic Data & Rendering

Coming soon: PointCloud data structure, scatter plot rendering, and WGSL shaders.

## License

MIT License - see LICENSE file for details

## Performance Targets

- 1M points @ 60 FPS (GPU instancing)
- 10M points @ 30 FPS
- Frame time: <10ms target
- GPU memory: <2GB for 10M points

## Contributing

This is a portfolio/learning project. Contributions welcome!
