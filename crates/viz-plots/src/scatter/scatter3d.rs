use anyhow::Result;
use bytemuck::{Pod, Zeroable};
use glam::{Vec3, Vec4};
use viz_core::{CameraUniforms, Dataset, OrbitalCamera, PointCloud, RenderContext};
use wgpu::util::DeviceExt;

/// Vertex format for scatter plot points
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// 3D scatter plot renderer
pub struct Scatter3D {
    /// GPU pipeline for rendering
    pipeline: wgpu::RenderPipeline,

    /// Vertex buffer containing point data
    vertex_buffer: wgpu::Buffer,

    /// Uniform buffer for camera data
    uniform_buffer: wgpu::Buffer,

    /// Bind group for uniforms
    bind_group: wgpu::BindGroup,

    /// Number of points to render
    point_count: u32,

    /// Point size in pixels
    point_size: f32,
}

impl Scatter3D {
    /// Create a new 3D scatter plot from a point cloud
    pub fn new(context: &RenderContext, point_cloud: &PointCloud) -> Result<Self> {
        // Convert point cloud to vertices
        let mut vertices = Vec::with_capacity(point_cloud.len());

        for i in 0..point_cloud.len() {
            let position = point_cloud.positions()[i];
            let color = point_cloud
                .colors()
                .map(|colors| colors[i])
                .unwrap_or(Vec4::ONE);

            vertices.push(Vertex {
                position: position.to_array(),
                color: color.to_array(),
            });
        }

        // Create vertex buffer
        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Scatter Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        // Create uniform buffer for camera
        let uniform_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: std::mem::size_of::<CameraUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout
        let bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Camera Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        // Create bind group
        let bind_group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Load shader
        let shader = context
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Scatter Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!("../../../../shaders/scatter.wgsl").into(),
                ),
            });

        // Create pipeline layout
        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Scatter Pipeline Layout"),
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        // Create render pipeline
        let pipeline = context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Scatter Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: context.config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::PointList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None, // TODO: Add depth buffer in future
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });

        Ok(Self {
            pipeline,
            vertex_buffer,
            uniform_buffer,
            bind_group,
            point_count: vertices.len() as u32,
            point_size: 5.0,
        })
    }

    /// Update camera uniforms
    pub fn update_camera(&self, context: &RenderContext, camera: &OrbitalCamera) {
        let uniforms = CameraUniforms::new(
            camera.view_projection_matrix(),
            camera.position(),
        );

        context
            .queue
            .write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));
    }

    /// Render the scatter plot
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.point_count, 0..1);
    }

    /// Get number of points
    pub fn point_count(&self) -> u32 {
        self.point_count
    }

    /// Get/set point size
    pub fn point_size(&self) -> f32 {
        self.point_size
    }

    pub fn set_point_size(&mut self, size: f32) {
        self.point_size = size;
    }
}
