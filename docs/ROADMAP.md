# Scientific Visualizer - Development Roadmap

## Project Vision

Build a production-grade scientific data visualization engine in Rust with:
- GPU acceleration for 1M+ points @ 60 FPS
- Cross-platform support (Linux, macOS, Windows, Web)
- Beautiful, intuitive UI
- Real-time rendering capabilities
- Export to images and videos

## Current Status

**Phase 4 Complete ✅** - UI Integration (2025-10-08)

## Phase Overview

| Phase | Name | Status | Est. LOC | Completion |
|-------|------|--------|----------|------------|
| 1 | Foundation & GPU Context | ✅ Complete | 800 | 2025-10-06 |
| 2 | Camera & Math | ✅ Complete | 1120 | 2025-10-07 |
| 3 | Basic Data & Rendering | ✅ Complete | 1036 | 2025-10-07 |
| 4 | UI Integration | ✅ Complete | 875 | 2025-10-08 |
| 5 | Colormap System | 📋 Planned | 400 | - |
| 6 | Data Loading | 📋 Planned | 500 | - |
| 7 | Performance Optimization | 📋 Planned | 700 | - |
| 8 | Additional Plot Types | 📋 Planned | 1000 | - |
| 9 | WASM Support | 📋 Planned | 600 | - |
| 10 | Examples & Documentation | 📋 Planned | 800 | - |

**Total Estimated LOC**: ~7,000

---

## ✅ Phase 1: Foundation & GPU Context (COMPLETE)

**Goal**: Set up project infrastructure and verify GPU initialization works.

### Completed Tasks
- [x] Create workspace structure (5 crates)
- [x] Configure all dependencies
- [x] Implement RenderContext (GPU initialization)
- [x] Create basic window example
- [x] Setup logging with tracing
- [x] Verify GPU works on target platform
- [x] Write initial documentation

### Deliverables
- ✅ Workspace with viz-core, viz-plots, viz-app, viz-wasm, examples
- ✅ RenderContext managing Device, Queue, Surface
- ✅ basic_window example rendering clear color
- ✅ README.md, LICENSE, ARCHITECTURE.md, CHANGELOG.md
- ✅ Verified on Apple M1 Max (Metal)

### Metrics
- **Lines of Code**: 800
- **Build Time**: ~30s initial, <1s incremental
- **Frame Time**: <1ms (clear only)
- **Memory**: 68MB base

---

## ✅ Phase 2: Camera & Math (COMPLETE)

**Goal**: Implement camera system with view/projection matrices and mouse controls.

### Completed Tasks
- [x] Implement Bounds3D (AABB bounding boxes)
- [x] Implement Transform utilities
- [x] Create OrbitalCamera with view/projection matrices
- [x] Implement mouse input handling (rotate, pan, zoom)
- [x] Add camera movement example
- [x] Test camera updates GPU correctly

### Deliverables
- ✅ OrbitalCamera with smooth controls
- ✅ Math utilities (Bounds3D, Transform)
- ✅ camera_movement example
- ✅ 23 tests passing

### Success Criteria
- [x] Camera movement smooth at 60 FPS - **Achieved 120 FPS**
- [x] Input latency <1ms
- [x] Proper view/projection matrix computation
- [x] Frame bounds auto-calculation

### Metrics
- **Lines of Code**: 1120
- **Development Time**: Completed 2025-10-07
- **Files**: camera/orbital.rs (314 LOC), math/bounds.rs (205 LOC), math/transform.rs (198 LOC)

---

## ✅ Phase 3: Basic Data & Rendering (COMPLETE)

**Goal**: Implement data structures and render actual geometry.

### Completed Tasks
- [x] Implement Dataset trait
- [x] Create PointCloud data structure
- [x] Create scatter plot shader (WGSL)
- [x] Implement Scatter3D renderer
- [x] Test rendering 1K and 10K points
- [x] Add per-point color support
- [x] Implement CameraUniforms for GPU
- [x] Create scatter_3d interactive example

### Deliverables
- ✅ PointCloud with positions, colors, sizes, metadata
- ✅ Scatter3D GPU-accelerated renderer
- ✅ scatter.wgsl shader with distance-based fading
- ✅ scatter_3d example with 2 generators
- ✅ 35 tests passing (all phases)

### Success Criteria
- [x] Render 10K points at 60 FPS - **Exceeded: 60+ FPS @ 10K, 120 FPS @ 1K**
- [x] Point colors work correctly
- [x] Camera controls work with rendered data
- [x] Height-based rainbow coloring

### Metrics
- **Lines of Code**: 1036
- **Development Time**: Completed 2025-10-07
- **Files**: data/point_cloud.rs (329 LOC), scatter/scatter3d.rs (229 LOC), scatter.wgsl (66 LOC)
- **Performance**: 120 FPS @ 1K points, 60+ FPS @ 10K points (Apple M1 Max)

---

## ✅ Phase 4: UI Integration (COMPLETE)

**Goal**: Add beautiful UI with controls and performance metrics.

### Completed Tasks
- [x] Integrate egui with wgpu (UiContext)
- [x] Create ControlPanel with basic controls
- [x] Add FPS counter and performance metrics
- [x] Implement point size slider
- [x] Add grid toggle (prepared for future)
- [x] Create background color picker
- [x] Test UI doesn't impact rendering performance
- [x] Add dataset selector dropdown
- [x] Implement H key to toggle UI visibility

### Deliverables
- ✅ egui integrated with render loop
- ✅ Control panel with sliders, toggles, and color picker
- ✅ Performance metrics panel with FPS graph
- ✅ Responsive UI at 60+ FPS
- ✅ scatter_3d_ui example

### Success Criteria
- [x] UI renders at 60 FPS - **Achieved: 60+ FPS @ 10K points**
- [x] Controls update visualization in real-time
- [x] FPS counter accurate with frame time graph
- [x] UI doesn't block rendering - event routing properly implemented

### Metrics
- **Lines of Code**: 875
- **Development Time**: Completed 2025-10-08
- **Files**: ui/context.rs (130 LOC), ui/metrics.rs (180 LOC), ui/panels.rs (190 LOC)
- **Performance**: <1ms UI overhead, 60+ FPS maintained

---

## 📋 Phase 5: Colormap System

**Goal**: Implement color mapping for data visualization.

### Planned Tasks
- [ ] Create Colormap trait
- [ ] Implement Viridis colormap
- [ ] Implement Plasma colormap
- [ ] Implement Inferno colormap
- [ ] Add Turbo colormap
- [ ] Create ColorScale (linear/log scaling)
- [ ] Apply colormaps to points based on metadata
- [ ] Add colormap selector in UI

### Deliverables
- Multiple scientific colormaps
- ColorScale with different scaling modes
- UI to select colormaps
- Example showing color-coded data

### Success Criteria
- [ ] Colormaps perceptually uniform
- [ ] Colorblind-friendly options
- [ ] Fast color lookup (<1μs)
- [ ] Smooth gradients (no banding)

### Estimated Effort
- **Lines of Code**: ~400
- **Development Time**: 1-2 days
- **Files**: color/colormap.rs, color/scale.rs

---

## 📋 Phase 6: Data Loading

**Goal**: Load data from files (CSV, Parquet).

### Planned Tasks
- [ ] Implement CSV loader with csv crate
- [ ] Implement Parquet loader with arrow/parquet
- [ ] Add async file loading
- [ ] Create progress indicators
- [ ] Handle errors gracefully
- [ ] Add file drag-and-drop (native)
- [ ] Test with large datasets (1M+ rows)

### Deliverables
- CSV and Parquet loaders
- Example loading real datasets
- Sample data files (iris.csv, etc.)
- Progress indicators for large files

### Success Criteria
- [ ] Load 1M row CSV in <2s
- [ ] Load 10M row Parquet in <5s
- [ ] Memory efficient (streaming if needed)
- [ ] Proper error messages

### Estimated Effort
- **Lines of Code**: ~500
- **Development Time**: 1-2 days
- **Files**: io/loaders.rs, io/exporters.rs

---

## 📋 Phase 7: Performance Optimization

**Goal**: Achieve 1M+ points @ 60 FPS.

### Planned Tasks
- [ ] Implement GPU instancing for scatter plots
- [ ] Add frustum culling
- [ ] Implement LOD (Level of Detail) system
- [ ] Optimize buffer updates (staging buffers)
- [ ] Add compute shader for data processing
- [ ] Profile and optimize hot paths
- [ ] Benchmark against targets
- [ ] Document performance characteristics

### Deliverables
- GPU instancing for massive point counts
- Frustum culling implementation
- LOD system for distance-based detail
- Comprehensive benchmarks

### Success Criteria
- [ ] 1M points @ 60 FPS
- [ ] 10M points @ 30 FPS
- [ ] Frame time <10ms
- [ ] GPU memory <2GB for 10M points

### Estimated Effort
- **Lines of Code**: ~700
- **Development Time**: 3-4 days
- **Files**: scatter/instancing.rs, scatter/lod.rs, benches/rendering_perf.rs

---

## 📋 Phase 8: Additional Plot Types

**Goal**: Implement line charts, heatmaps, surfaces, volumes.

### Planned Tasks
- [ ] Implement 2D line chart
- [ ] Add streaming line chart (real-time data)
- [ ] Implement heatmap with compute shader
- [ ] Create 3D surface plot
- [ ] Add volume rendering (ray casting)
- [ ] Implement network graph visualization
- [ ] Add force-directed layout
- [ ] Create examples for each plot type

### Deliverables
- 6 plot types (scatter, line, heatmap, surface, volume, graph)
- Examples demonstrating each
- Performance benchmarks for each

### Success Criteria
- [ ] Each plot type performs well
- [ ] Consistent API across plot types
- [ ] Examples are clear and useful
- [ ] Documentation complete

### Estimated Effort
- **Lines of Code**: ~1000
- **Development Time**: 4-5 days
- **Files**: line/*.rs, heatmap/*.rs, surface/*.rs, volume/*.rs, graph/*.rs

---

## 📋 Phase 9: WASM Support

**Goal**: Make it work in the browser.

### Planned Tasks
- [ ] Setup wasm-pack build
- [ ] Create web entry point
- [ ] Handle browser-specific APIs (FileReader, etc.)
- [ ] Add HTML/CSS for demo page
- [ ] Optimize WASM bundle size
- [ ] Test on multiple browsers
- [ ] Deploy demo to GitHub Pages
- [ ] Add web-specific examples

### Deliverables
- viz-wasm crate compiled to WASM
- Demo website with examples
- Build script for WASM
- Documentation for web usage

### Success Criteria
- [ ] Works in Chrome, Firefox, Safari
- [ ] Performance 70-80% of native
- [ ] Bundle size <5MB
- [ ] Demo deployed and accessible

### Estimated Effort
- **Lines of Code**: ~600
- **Development Time**: 2-3 days
- **Files**: viz-wasm/src/lib.rs, www/index.html, scripts/build_web.sh

---

## 📋 Phase 10: Examples & Documentation

**Goal**: Polish, document, and create comprehensive examples.

### Planned Tasks
- [ ] Create 10+ examples covering all features
- [ ] Write comprehensive API documentation
- [ ] Create user guide with screenshots
- [ ] Document performance characteristics
- [ ] Create architecture diagrams
- [ ] Record demo videos
- [ ] Write blog post about the project
- [ ] Create presentation/slides

### Deliverables
- 10+ examples
- Complete API documentation
- User guide with visuals
- Performance guide
- Architecture diagrams
- Demo videos

### Success Criteria
- [ ] Anyone can create first plot in <10 minutes
- [ ] All features documented
- [ ] Examples run without errors
- [ ] Professional presentation

### Estimated Effort
- **Lines of Code**: ~800 (examples + docs)
- **Development Time**: 3-4 days
- **Files**: examples/*.rs, docs/*.md

---

## Post-Phase 10: Future Enhancements

### Potential Future Work
- **Python Bindings** (PyO3) - Use from Python
- **REST API Server** (Axum) - Visualization as a service
- **Advanced Interactions** - Selection, annotation
- **Animation System** - Time-series playback
- **Statistical Overlays** - Regression, confidence intervals
- **Specialized Plots** - Parallel coordinates, Sankey, treemaps
- **Collaboration Features** - Share plots, real-time multi-user
- **Mobile Support** - Android/iOS via wgpu

### Research Integration
- Integration with your Multi-Modal RAG system
- JSON API for plot specifications
- Automatic visualization from natural language

---

## Milestones

| Milestone | Phases | Target Date | Status |
|-----------|--------|-------------|--------|
| MVP | 1-3 | - | Phase 1 ✅ |
| Alpha | 1-6 | - | Phase 1 ✅ |
| Beta | 1-9 | - | - |
| v1.0 | 1-10 | - | - |

---

## Success Metrics

### Performance Targets
- ✅ GPU initialization works
- [ ] 60 FPS with 100K points
- [ ] 60 FPS with 1M points (Phase 7 goal)
- [ ] 30 FPS with 10M points (Phase 7 goal)
- [ ] <10ms frame time
- [ ] <2GB GPU memory for 10M points

### Code Quality Targets
- [ ] >80% test coverage (Phase 10)
- [ ] Zero clippy warnings
- [ ] All public APIs documented
- [ ] <10 unsafe blocks total

### User Experience Targets
- [ ] <10 minute time-to-first-plot (Phase 10)
- [ ] Works on 3+ platforms (Linux, macOS, Windows)
- [ ] Works in browser (Phase 9)
- [ ] Beautiful visuals (subjective but important!)

---

## Dependencies & Risks

### Critical Dependencies
- wgpu (WebGPU API) - core dependency
- winit (window management) - core dependency
- egui (UI) - important for usability

### Risks & Mitigations
1. **GPU compatibility**: Test on multiple GPUs early (Phase 1 ✅)
2. **Performance**: Profile early, optimize continuously (Phase 7)
3. **WASM limitations**: Plan for feature parity (Phase 9)
4. **Scope creep**: Stick to roadmap, defer nice-to-haves

---

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to contribute to this roadmap.

---

**Last Updated**: 2025-10-06
**Current Phase**: Phase 1 Complete ✅, Phase 2 Next 📋
