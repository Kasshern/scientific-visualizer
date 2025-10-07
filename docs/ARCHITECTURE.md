# Architecture Overview

## System Design

Scientific Visualizer is a modular, GPU-accelerated visualization engine built in Rust.

## Module Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│                   viz-app / viz-wasm                     │
│              (Application Layer)                         │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                    viz-plots                            │
│         (Plot Implementations: Scatter, Line, etc.)     │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│                    viz-core                              │
│   (Renderer, Camera, Data, Color, Math)                │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              wgpu + winit + egui                        │
│                (GPU Abstraction)                         │
└──────────────────────────────────────────────────────────┘
```

## viz-core

The core library provides foundational GPU rendering capabilities:

### Renderer Module
- **RenderContext**: Manages GPU device, queue, and surface
- **CameraUniforms**: GPU-compatible camera data structure
- **PipelineBuilder**: Helper for creating render pipelines (planned)
- **BufferManager**: GPU buffer management utilities (planned)

### Camera Module ✅
- **OrbitalCamera**: 3D camera with orbit controls (rotate, pan, zoom)
- **OrthographicCamera**: 2D camera for flat projections (planned Phase 4+)

### Data Module ✅
- **Dataset** trait: Generic interface for data (bounds, len, name)
- **PointCloud**: 3D point data with colors, sizes, and metadata
- **TimeSeries**: 1D/2D time-series data (planned Phase 6)
- **Volume**: 3D volumetric data (planned Phase 8)

### Color Module (Phase 5)
- **Colormap** trait: Color mapping interface
- Implementations: Viridis, Plasma, Inferno, etc.
- **ColorScale**: Linear/log scaling

### Math Module ✅
- **Bounds3D**: AABB bounding boxes with intersection/containment tests
- **Transform**: TRS transformation matrices with inverse

### Renderer Module ✅
- **CameraUniforms**: GPU-compatible camera data (80 bytes, Pod + Zeroable)

## viz-plots

High-level plot implementations built on viz-core:

### Scatter Plots ✅
- **Scatter3D**: GPU-accelerated 3D scatter plot renderer
  - Vertex buffer management for point positions and colors
  - Camera uniform buffer binding
  - Point primitive rendering with alpha blending
  - Configurable point size
  - WGSL shader integration (scatter.wgsl)
  - Distance-based point fading
  - Real-time camera updates

### Future Plot Types (Planned)
- **Line2D/Line3D**: Connected line segments (Phase 8)
- **Heatmap2D**: 2D density visualization (Phase 8)
- **Surface3D**: 3D surface plots (Phase 8)
- **Volume3D**: Volumetric rendering (Phase 8)

## GPU Pipeline

```
1. User Data (CPU)
   ↓
2. Upload to GPU Buffers (VRAM)
   ↓
3. Vertex Shader (WGSL)
   ↓
4. Rasterization
   ↓
5. Fragment Shader (WGSL)
   ↓
6. Surface Texture
   ↓
7. Present to Screen
```

## Performance Strategy

### GPU Instancing
Render millions of points with a single draw call by using instanced rendering.

### Frustum Culling
Only render objects within the camera's view frustum.

### LOD (Level of Detail)
Reduce point density for distant objects.

### Compute Shaders
Use GPU compute for data processing (heatmaps, density estimation).

## Current Implementation Status

### Phase 1-3 Complete ✅
- ✅ GPU initialization with wgpu (Phase 1)
- ✅ Window management with winit (Phase 1)
- ✅ Surface configuration (Phase 1)
- ✅ OrbitalCamera with mouse controls (Phase 2)
- ✅ Math utilities (Bounds3D, Transform) (Phase 2)
- ✅ Dataset trait and PointCloud (Phase 3)
- ✅ Scatter3D GPU renderer (Phase 3)
- ✅ WGSL shaders (scatter.wgsl) (Phase 3)
- ✅ 41 tests passing (35 unit + 6 doc tests)
- ✅ 120 FPS @ 1K points, 60+ FPS @ 10K points

## Design Decisions

### Why wgpu?
- Cross-platform (Metal, Vulkan, DX12, WebGL2)
- Future-proof (WebGPU standard)
- Excellent Rust API
- WASM support

### Why Arc<Window>?
wgpu's Surface requires 'static lifetime. Using Arc allows sharing window ownership between event loop and render context.

### Why Separate Crates?
- **viz-core**: Reusable library for any visualization
- **viz-plots**: High-level plot implementations
- **viz-app**: Desktop application
- **viz-wasm**: Web-specific code
- **examples**: Demonstration binaries

This allows users to depend only on viz-core for custom visualizations.

## Error Handling

All errors use thiserror for ergonomic error types:
- **RenderError**: GPU-related errors
- **DataError**: Data loading/parsing errors
- **PlotError**: Plot configuration errors

All public APIs return `Result<T, E>` for proper error propagation.

## Thread Safety

- GPU resources (Device, Queue) are Send + Sync
- Data structures use Arc for shared ownership
- Compute-heavy operations use rayon for parallelism

## Next Phase: UI Integration

Phase 4 will implement:
- egui integration with wgpu render loop
- Control panel for visualization parameters
- Performance metrics display (FPS, frame time)
- Data inspection panel
- Interactive controls (point size, colors, etc.)
