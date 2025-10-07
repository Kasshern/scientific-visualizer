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
- **PipelineBuilder**: Helper for creating render pipelines
- **BufferManager**: GPU buffer management utilities

### Camera Module (Phase 2)
- **OrbitalCamera**: 3D camera with orbit controls
- **OrthographicCamera**: 2D camera for flat projections

### Data Module (Phase 3)
- **Dataset** trait: Generic interface for data
- **PointCloud**: 3D point data with colors and metadata
- **TimeSeries**: 1D/2D time-series data
- **Volume**: 3D volumetric data

### Color Module (Phase 5)
- **Colormap** trait: Color mapping interface
- Implementations: Viridis, Plasma, Inferno, etc.
- **ColorScale**: Linear/log scaling

### Math Module (Phase 2)
- **Bounds3D**: Bounding boxes, AABB
- **Transform**: Transformation matrices

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

## Phase 1 Implementation

Current state:
- ✅ GPU initialization with wgpu
- ✅ Window management with winit
- ✅ Surface configuration
- ✅ Basic render loop with clear color
- ✅ Logging infrastructure

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

## Next Phase: Camera System

Phase 2 will implement:
- Orbital camera with smooth controls
- View/projection matrix computation
- Mouse input handling (rotate, pan, zoom)
- Camera uniforms uploaded to GPU
