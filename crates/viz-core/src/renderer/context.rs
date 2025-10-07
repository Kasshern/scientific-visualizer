use anyhow::Result;
use std::sync::Arc;
use thiserror::Error;
use tracing::{info, warn, instrument};
use wgpu::{Adapter, Device, Queue, Surface, SurfaceConfiguration};
use winit::window::Window;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("GPU initialization failed: {0}")]
    GpuInitError(String),

    #[error("Failed to find suitable GPU adapter")]
    NoAdapterFound,

    #[error("Failed to request device: {0}")]
    DeviceRequestFailed(String),

    #[error("Surface configuration failed: {0}")]
    SurfaceConfigError(String),

    #[error("Shader compilation failed: {0}")]
    ShaderError(String),

    #[error("Out of GPU memory (tried to allocate {requested} bytes)")]
    OutOfMemory { requested: usize },
}

/// Core GPU rendering context that manages wgpu device, queue, and surface
pub struct RenderContext {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub config: SurfaceConfiguration,
    pub adapter_info: wgpu::AdapterInfo,
}

impl RenderContext {
    /// Creates a new RenderContext with GPU initialization
    ///
    /// # Performance
    /// This async function initializes the GPU and may take 100-500ms on first call.
    ///
    /// # Examples
    /// ```no_run
    /// use viz_core::RenderContext;
    /// use winit::window::Window;
    ///
    /// async fn init(window: &Window) {
    ///     let context = RenderContext::new(window).await.unwrap();
    ///     println!("GPU: {}", context.adapter_info.name);
    /// }
    /// ```
    #[instrument(skip(window))]
    pub async fn new(window: Arc<Window>) -> Result<Self, RenderError> {
        info!("Initializing GPU context");

        // Create wgpu instance with all available backends
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::default(),
            dx12_shader_compiler: wgpu::Dx12Compiler::default(),
            gles_minor_version: wgpu::Gles3MinorVersion::default(),
        });

        // Create surface
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| RenderError::GpuInitError(e.to_string()))?;

        // Request adapter with high performance preference
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(RenderError::NoAdapterFound)?;

        let adapter_info = adapter.get_info();
        info!(
            "Selected GPU adapter: {} ({:?})",
            adapter_info.name, adapter_info.backend
        );

        // Check for required features
        let features = adapter.features();
        info!("GPU features: {:?}", features);

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Main Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| RenderError::DeviceRequestFailed(e.to_string()))?;

        info!("GPU device created successfully");

        // Configure surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, // VSync
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);
        info!(
            "Surface configured: {}x{}, format: {:?}",
            config.width, config.height, config.format
        );

        Ok(Self {
            device,
            queue,
            surface,
            config,
            adapter_info,
        })
    }

    /// Resize the surface (called when window is resized)
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > 0 && new_height > 0 {
            self.config.width = new_width;
            self.config.height = new_height;
            self.surface.configure(&self.device, &self.config);
            info!("Surface resized to {}x{}", new_width, new_height);
        } else {
            warn!("Attempted to resize to invalid dimensions: {}x{}", new_width, new_height);
        }
    }

    /// Get the current surface texture for rendering
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, RenderError> {
        self.surface
            .get_current_texture()
            .map_err(|e| RenderError::SurfaceConfigError(e.to_string()))
    }

    /// Get aspect ratio of the surface
    pub fn aspect_ratio(&self) -> f32 {
        self.config.width as f32 / self.config.height as f32
    }
}
