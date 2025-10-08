# Scientific Visualizer

A high-performance scientific data visualization engine in Rust with GPU acceleration, cross-platform support (native + WASM), and real-time rendering capabilities.

## Status: Phase 4 Complete ✓

**UI Integration** - Professional egui interface with performance metrics, interactive controls, and real-time visualization tuning.

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

# Run the 3D scatter plot with UI (Phase 4) ⭐ RECOMMENDED
cargo run -p examples --bin scatter_3d_ui
# Full UI with performance metrics, point size control, dataset selector
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

## Phase 4 Complete ✓

- [x] egui-wgpu integration (UiContext)
- [x] Performance metrics tracking and display
- [x] Real-time FPS counter with frame time graph
- [x] Control panel with interactive sliders
- [x] Point size adjustment (1-20 pixels)
- [x] Dataset selector dropdown
- [x] Background color picker
- [x] H key to toggle UI visibility
- [x] 39 tests passing (all phases)
- [x] 60+ FPS with UI enabled @ 10K points

**Phase 1**: ✅ GPU Context & Foundation
**Phase 2**: ✅ Camera & Math Systems
**Phase 3**: ✅ Basic Data & Rendering
**Phase 4**: ✅ UI Integration

**Tested On**: Apple M1 Max (Metal backend)

## Next: Phase 5 - Colormap System

Coming soon: Scientific colormaps (Viridis, Plasma, Inferno) for data-driven coloring.

## License

MIT License - see LICENSE file for details

## Performance Targets

- 1M points @ 60 FPS (GPU instancing)
- 10M points @ 30 FPS
- Frame time: <10ms target
- GPU memory: <2GB for 10M points

## Contributing

This is a portfolio/learning project. Contributions welcome!
