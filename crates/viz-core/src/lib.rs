pub mod renderer;
pub mod camera;
pub mod data;
pub mod color;
pub mod math;
pub mod ui;

pub use renderer::{RenderContext, RenderError, CameraUniforms};
pub use math::{Bounds3D, Transform};
pub use camera::OrbitalCamera;
pub use data::{Dataset, PointCloud};
pub use color::{Colormap, Viridis, Plasma, Inferno, Turbo, ColorScale, ScaleType};
pub use ui::{UiContext, PerformanceMetrics, ControlPanel, performance_panel};
