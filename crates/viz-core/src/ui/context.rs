use anyhow::Result;
use egui_wgpu::ScreenDescriptor;
use winit::window::Window;

/// UI context managing egui rendering
///
/// This integrates egui with the wgpu rendering pipeline,
/// allowing immediate-mode UI to be rendered on top of 3D visualizations.
pub struct UiContext {
    /// egui context for UI state
    pub egui_ctx: egui::Context,

    /// egui-winit state for input handling
    egui_state: egui_winit::State,

    /// egui-wgpu renderer
    renderer: egui_wgpu::Renderer,
}

impl UiContext {
    /// Create a new UI context
    pub fn new(
        device: &wgpu::Device,
        surface_format: wgpu::TextureFormat,
        window: &Window,
    ) -> Self {
        let egui_ctx = egui::Context::default();

        // Create egui-winit state for handling window events
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            None,
            None,
        );

        // Create egui-wgpu renderer
        let renderer = egui_wgpu::Renderer::new(device, surface_format, None, 1);

        Self {
            egui_ctx,
            egui_state,
            renderer,
        }
    }

    /// Handle window event
    pub fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
        let response = self.egui_state.on_window_event(window, event);
        response.consumed
    }

    /// Begin a new frame
    pub fn begin_frame(&mut self, window: &Window) -> egui::Context {
        let raw_input = self.egui_state.take_egui_input(window);
        self.egui_ctx.begin_frame(raw_input);
        self.egui_ctx.clone()
    }

    /// End frame and prepare rendering
    pub fn end_frame(&mut self, _window: &Window) -> egui::FullOutput {
        self.egui_ctx.end_frame()
    }

    /// Render the UI
    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        window: &Window,
        view: &wgpu::TextureView,
        full_output: egui::FullOutput,
    ) -> Result<()> {
        let size = window.inner_size();
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: window.scale_factor() as f32,
        };

        // Upload all resources for the GPU
        let primitives = self.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(device, queue, *id, image_delta);
        }

        self.renderer
            .update_buffers(device, queue, encoder, &primitives, &screen_descriptor);

        // Render egui
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load, // Don't clear - render on top
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            self.renderer
                .render(&mut render_pass, &primitives, &screen_descriptor);
        }

        // Cleanup textures
        for id in &full_output.textures_delta.free {
            self.renderer.free_texture(id);
        }

        Ok(())
    }

    /// Get whether the UI is using the mouse
    pub fn wants_pointer_input(&self) -> bool {
        self.egui_ctx.wants_pointer_input()
    }

    /// Get whether the UI is using the keyboard
    pub fn wants_keyboard_input(&self) -> bool {
        self.egui_ctx.wants_keyboard_input()
    }
}
