use anyhow::Result;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use viz_core::RenderContext;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<()> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Scientific Visualizer - Basic Window Example");

    // Create event loop and window
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Scientific Visualizer - GPU Test")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)?
    );

    // Initialize GPU context
    let mut render_context = pollster::block_on(RenderContext::new(window.clone()))?;

    info!(
        "GPU initialized: {} ({:?})",
        render_context.adapter_info.name, render_context.adapter_info.backend
    );

    // Main event loop
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        info!("Close requested, exiting");
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        info!("Window resized to {:?}", physical_size);
                        render_context.resize(physical_size.width, physical_size.height);
                    }
                    WindowEvent::RedrawRequested => {
                        // Get surface texture
                        match render_context.get_current_texture() {
                            Ok(output) => {
                                let view = output
                                    .texture
                                    .create_view(&wgpu::TextureViewDescriptor::default());

                                let mut encoder = render_context.device.create_command_encoder(
                                    &wgpu::CommandEncoderDescriptor {
                                        label: Some("Render Encoder"),
                                    },
                                );

                                {
                                    let _render_pass = encoder.begin_render_pass(
                                        &wgpu::RenderPassDescriptor {
                                            label: Some("Clear Pass"),
                                            color_attachments: &[Some(
                                                wgpu::RenderPassColorAttachment {
                                                    view: &view,
                                                    resolve_target: None,
                                                    ops: wgpu::Operations {
                                                        load: wgpu::LoadOp::Clear(
                                                            wgpu::Color {
                                                                r: 0.1,
                                                                g: 0.2,
                                                                b: 0.3,
                                                                a: 1.0,
                                                            },
                                                        ),
                                                        store: wgpu::StoreOp::Store,
                                                    },
                                                },
                                            )],
                                            depth_stencil_attachment: None,
                                            timestamp_writes: None,
                                            occlusion_query_set: None,
                                        },
                                    );
                                }

                                render_context.queue.submit(std::iter::once(encoder.finish()));
                                output.present();
                            }
                            Err(e) => {
                                eprintln!("Failed to get surface texture: {}", e);
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
