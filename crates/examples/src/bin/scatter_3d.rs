use anyhow::Result;
use glam::Vec3;
use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use viz_core::{Dataset, OrbitalCamera, PointCloud, RenderContext};
use viz_plots::Scatter3D;
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

/// Generate a spiral of points for visualization
fn generate_spiral_points(num_points: usize) -> PointCloud {
    let mut positions = Vec::with_capacity(num_points);

    for i in 0..num_points {
        let t = (i as f32 / num_points as f32) * 4.0 * PI;
        let radius = 5.0 * (i as f32 / num_points as f32);

        let x = radius * t.cos();
        let y = t * 0.5 - 5.0; // Vertical component
        let z = radius * t.sin();

        positions.push(Vec3::new(x, y, z));
    }

    let mut cloud = PointCloud::new(positions);
    cloud.generate_height_colors(); // Color by height
    cloud.with_name("Spiral")
}

/// Generate a cube of random points
fn generate_cube_points(num_points: usize) -> PointCloud {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut positions = Vec::with_capacity(num_points);

    for _ in 0..num_points {
        let x = rng.gen_range(-5.0..5.0);
        let y = rng.gen_range(-5.0..5.0);
        let z = rng.gen_range(-5.0..5.0);
        positions.push(Vec3::new(x, y, z));
    }

    let mut cloud = PointCloud::new(positions);
    cloud.generate_height_colors();
    cloud.with_name("Random Cube")
}

fn main() -> Result<()> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting 3D Scatter Plot Example");
    info!("Controls:");
    info!("  - Left Mouse Drag: Rotate camera");
    info!("  - Shift + Left Mouse Drag: Pan camera");
    info!("  - Mouse Wheel: Zoom in/out");
    info!("  - R: Reset camera");
    info!("  - 1: Spiral (1000 points)");
    info!("  - 2: Cube (10000 points)");
    info!("  - ESC: Exit");

    // Create event loop and window
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Scientific Visualizer - 3D Scatter Plot")
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
        Vec3::ZERO,
        20.0,
        render_context.aspect_ratio(),
    );

    // Generate initial point cloud (spiral)
    let point_cloud = generate_spiral_points(1000);
    info!("Generated {} points in spiral pattern", point_cloud.len());

    // Create scatter plot
    let mut scatter = Scatter3D::new(&render_context, &point_cloud)?;
    info!("Scatter plot initialized with {} points", scatter.point_count());

    // Frame the camera to see all points
    let bounds = point_cloud.bounds();
    camera.frame_bounds(bounds.min, bounds.max, 0.2);

    // Input state
    let mut input_state = InputState::new();

    // Performance tracking
    let mut frame_count = 0u32;
    let mut fps_timer = Instant::now();

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
                        render_context.resize(physical_size.width, physical_size.height);
                        camera.set_aspect(render_context.aspect_ratio());
                    }

                    WindowEvent::MouseInput { state, button, .. } => {
                        if button == MouseButton::Left {
                            input_state.mouse_pressed = state == ElementState::Pressed;
                        }
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        let current_pos = (position.x as f32, position.y as f32);

                        if input_state.mouse_pressed {
                            let delta_x = current_pos.0 - input_state.last_mouse_pos.0;
                            let delta_y = current_pos.1 - input_state.last_mouse_pos.1;

                            if input_state.shift_pressed {
                                camera.pan(delta_x, -delta_y);
                            } else {
                                let sensitivity = 0.005;
                                camera.rotate(delta_x * sensitivity, -delta_y * sensitivity);
                            }
                        }

                        input_state.last_mouse_pos = current_pos;
                    }

                    WindowEvent::MouseWheel { delta, .. } => {
                        let zoom_delta = match delta {
                            MouseScrollDelta::LineDelta(_, y) => y * 0.1,
                            MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.001,
                        };
                        camera.zoom(zoom_delta);
                    }

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
                                        camera.distance = 20.0;
                                    }
                                    KeyCode::Digit1 => {
                                        info!("Switching to spiral (1000 points)");
                                        let point_cloud = generate_spiral_points(1000);
                                        scatter = Scatter3D::new(&render_context, &point_cloud).unwrap();
                                        let bounds = point_cloud.bounds();
                                        camera.frame_bounds(bounds.min, bounds.max, 0.2);
                                    }
                                    KeyCode::Digit2 => {
                                        info!("Switching to cube (10000 points)");
                                        let point_cloud = generate_cube_points(10000);
                                        scatter = Scatter3D::new(&render_context, &point_cloud).unwrap();
                                        let bounds = point_cloud.bounds();
                                        camera.frame_bounds(bounds.min, bounds.max, 0.2);
                                    }
                                    KeyCode::ShiftLeft | KeyCode::ShiftRight => {
                                        input_state.shift_pressed = true;
                                    }
                                    _ => {}
                                }
                            }
                        } else if event.state == ElementState::Released {
                            if let PhysicalKey::Code(keycode) = event.physical_key {
                                if matches!(keycode, KeyCode::ShiftLeft | KeyCode::ShiftRight) {
                                    input_state.shift_pressed = false;
                                }
                            }
                        }
                    }

                    WindowEvent::RedrawRequested => {
                        // Update camera uniforms
                        scatter.update_camera(&render_context, &camera);

                        // Render
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
                                    let mut render_pass =
                                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                            label: Some("Main Render Pass"),
                                            color_attachments: &[Some(
                                                wgpu::RenderPassColorAttachment {
                                                    view: &view,
                                                    resolve_target: None,
                                                    ops: wgpu::Operations {
                                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                                            r: 0.05,
                                                            g: 0.05,
                                                            b: 0.08,
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

                                    // Render scatter plot!
                                    scatter.render(&mut render_pass);
                                }

                                render_context.queue.submit(std::iter::once(encoder.finish()));
                                output.present();

                                // Track FPS
                                frame_count += 1;
                                if fps_timer.elapsed().as_secs() >= 1 {
                                    let fps = frame_count as f32 / fps_timer.elapsed().as_secs_f32();
                                    info!(
                                        "FPS: {:.1} | Points: {} | Camera distance: {:.1}",
                                        fps,
                                        scatter.point_count(),
                                        camera.distance
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
