# Scientific Visualizer

A high-performance scientific data visualization engine in Rust with GPU acceleration, cross-platform support (native + WASM), and real-time rendering capabilities.

## Status: Phase 5 Complete ✓

**Colormap System** - Scientific color mapping with Viridis, Plasma, Inferno, and Turbo colormaps. Data-driven visualization with linear and logarithmic scaling.

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

# Run the 3D scatter plot example (Phase 3)
cargo run -p examples --bin scatter_3d
# Controls: Left-drag to rotate, Shift-drag to pan, Scroll to zoom
# Press 1 for spiral (1K points), 2 for cube (10K points), R to reset

# Run the 3D scatter plot with UI and Colormaps (Phase 5) ⭐ RECOMMENDED
cargo run -p examples --bin scatter_3d_ui
# Full UI with performance metrics, colormap selector, dataset switching
# Choose from Viridis, Plasma, Inferno, Turbo colormaps
# Toggle linear/log scaling, adjust point size, pick background color
# Press H to toggle UI visibility
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

## Phase 5 Complete ✓

- [x] Colormap trait with 4 scientific colormaps
- [x] Viridis (256-LUT), Plasma, Inferno, Turbo (8-LUT each)
- [x] Linear and logarithmic color scaling
- [x] PointCloud metadata-driven coloring
- [x] UI colormap selector with live preview
- [x] Dynamic colormap switching in real-time
- [x] Enhanced scatter_3d_ui example with metadata
- [x] 52 tests passing (17 new colormap tests)
- [x] <1μs color lookup, 60+ FPS maintained

**Phase 1**: ✅ GPU Context & Foundation
**Phase 2**: ✅ Camera & Math Systems
**Phase 3**: ✅ Basic Data & Rendering
**Phase 4**: ✅ UI Integration
**Phase 5**: ✅ Colormap System

**Tested On**: Apple M1 Max (Metal backend)

## Next: Phase 6 - Data Loading

Load real-world datasets from CSV and Parquet files with async loading and progress indicators.

## License

MIT License - see LICENSE file for details

## Performance Targets

- 1M points @ 60 FPS (GPU instancing)
- 10M points @ 30 FPS
- Frame time: <10ms target
- GPU memory: <2GB for 10M points

## Contributing

This is a portfolio/learning project. Contributions welcome!
