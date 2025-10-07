# Scientific Visualizer

A high-performance scientific data visualization engine in Rust with GPU acceleration, cross-platform support (native + WASM), and real-time rendering capabilities.

## Status: Phase 1 Complete ✓

**GPU Context & Foundation** - GPU initialization, window management, and basic rendering infrastructure are working.

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

## Phase 1 Complete ✓

- [x] Workspace structure created
- [x] GPU context initialization
- [x] Window management with winit
- [x] Basic render loop
- [x] Logging with tracing
- [x] Clear screen to solid color

**GPU Tested On**: Apple M1 Max (Metal backend)

## Next: Phase 2 - Camera & Math

Coming soon: Orbital camera with view/projection matrices, mouse input handling, and basic math utilities.

## License

MIT License - see LICENSE file for details

## Performance Targets

- 1M points @ 60 FPS (GPU instancing)
- 10M points @ 30 FPS
- Frame time: <10ms target
- GPU memory: <2GB for 10M points

## Contributing

This is a portfolio/learning project. Contributions welcome!
