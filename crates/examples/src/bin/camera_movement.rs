use anyhow::Result;
use glam::Vec3;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use viz_core::{OrbitalCamera, RenderContext};
use winit::{
    event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

/// Input state for camera controls
struct InputState {
    mouse_pressed: bool,
    last_mouse_pos: (f32, f32),
    shift_pressed: bool,
}

impl InputState {
    fn new() -> Self {
        Self {
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
            shift_pressed: false,
        }
    }
}

fn main() -> Result<()> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Camera Movement Example");
    info!("Controls:");
    info!("  - Left Mouse Drag: Rotate camera");
    info!("  - Shift + Left Mouse Drag: Pan camera");
    info!("  - Mouse Wheel: Zoom in/out");
    info!("  - R: Reset camera");
    info!("  - ESC: Exit");

    // Create event loop and window
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Scientific Visualizer - Camera Movement")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)?,
    );

    // Initialize GPU context
    let mut render_context = pollster::block_on(RenderContext::new(window.clone()))?;

    info!(
        "GPU initialized: {} ({:?})",
        render_context.adapter_info.name, render_context.adapter_info.backend
    );

    // Create camera
    let mut camera = OrbitalCamera::new(
        Vec3::ZERO,                        // Target
        15.0,                              // Distance
        render_context.aspect_ratio(),     // Aspect ratio
    );

    info!("Camera initialized at distance {} from origin", camera.distance);

    // Input state
    let mut input_state = InputState::new();

    // Performance tracking
    let mut frame_count = 0u32;
    let mut fps_timer = Instant::now();
    let mut last_fps = 0.0f32;

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
                        camera.set_aspect(render_context.aspect_ratio());
                    }

                    // Mouse button events
                    WindowEvent::MouseInput { state, button, .. } => {
                        if button == MouseButton::Left {
                            input_state.mouse_pressed = state == ElementState::Pressed;
                        }
                    }

                    // Mouse movement
                    WindowEvent::CursorMoved { position, .. } => {
                        let current_pos = (position.x as f32, position.y as f32);

                        if input_state.mouse_pressed {
                            let delta_x = current_pos.0 - input_state.last_mouse_pos.0;
                            let delta_y = current_pos.1 - input_state.last_mouse_pos.1;

                            if input_state.shift_pressed {
                                // Pan mode
                                camera.pan(delta_x, -delta_y);
                            } else {
                                // Rotate mode
                                let sensitivity = 0.005;
                                camera.rotate(delta_x * sensitivity, -delta_y * sensitivity);
                            }
                        }

                        input_state.last_mouse_pos = current_pos;
                    }

                    // Mouse wheel for zoom
                    WindowEvent::MouseWheel { delta, .. } => {
                        let zoom_delta = match delta {
                            MouseScrollDelta::LineDelta(_, y) => y * 0.1,
                            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.001,
                        };

                        camera.zoom(zoom_delta);
                    }

                    // Keyboard
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                            if let PhysicalKey::Code(keycode) = event.physical_key {
                                match keycode {
                                    KeyCode::Escape => {
                                        info!("ESC pressed, exiting");
                                        elwt.exit();
                                    }
                                    KeyCode::KeyR => {
                                        info!("Resetting camera");
                                        camera.reset();
                                    }
                                    KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                                        input_state.shift_pressed = true;
                                    }
                                    _ => {}
                                }
                            }
                        } else if event.state == ElementState::Released {
                            if let PhysicalKey::Code(keycode) = event.physical_key {
                                match keycode {
                                    KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                                        input_state.shift_pressed = false;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        // Get view projection matrix from camera
                        let _view_proj = camera.view_projection_matrix();

                        // Render (just clear for now, will use view_proj in Phase 3)
                        match render_context.get_current_texture() {
                            Ok(output) => {
                                let view = output
                                    .texture
                                    .create_view(&wgpu::TextureViewDescriptor::default());

                                let mut encoder = render_context
                                    .device
                                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                        label: Some("Render Encoder"),
                                    });

                                {
                                    // Render pass with gradient based on camera position
                                    let pos = camera.position();
                                    let normalized = (pos.y / 20.0 + 0.5).clamp(0.0, 1.0);

                                    let _render_pass =
                                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                            label: Some("Clear Pass"),
                                            color_attachments: &[Some(
                                                wgpu::RenderPassColorAttachment {
                                                    view: &view,
                                                    resolve_target: None,
                                                    ops: wgpu::Operations {
                                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                                            r: 0.1 * normalized as f64,
                                                            g: 0.2 * normalized as f64,
                                                            b: 0.3 + 0.2 * normalized as f64,
                                                            a: 1.0,
                                                        }),
                                                        store: wgpu::StoreOp::Store,
                                                    },
                                                },
                                            )],
                                            depth_stencil_attachment: None,
                                            timestamp_writes: None,
                                            occlusion_query_set: None,
                                        });
                                }

                                render_context.queue.submit(std::iter::once(encoder.finish()));
                                output.present();

                                // Track FPS
                                frame_count += 1;
                                if fps_timer.elapsed().as_secs() >= 1 {
                                    last_fps = frame_count as f32 / fps_timer.elapsed().as_secs_f32();
                                    info!(
                                        "FPS: {:.1} | Camera: pos={:.2?}, distance={:.2}, yaw={:.2}, pitch={:.2}",
                                        last_fps,
                                        camera.position(),
                                        camera.distance,
                                        camera.yaw.to_degrees(),
                                        camera.pitch.to_degrees()
                                    );
                                    frame_count = 0;
                                    fps_timer = Instant::now();
                                }
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
