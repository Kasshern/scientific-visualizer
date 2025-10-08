use anyhow::Result;
use glam::Vec3;
use std::f32::consts::PI;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use viz_core::{
    performance_panel, ControlPanel, Dataset, OrbitalCamera, PerformanceMetrics, PointCloud,
    RenderContext, UiContext,
};
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
    cloud.with_name("Spiral (1K points)")
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
    cloud.with_name("Random Cube (10K points)")
}

fn main() -> Result<()> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting 3D Scatter Plot with UI Example");
    info!("Controls:");
    info!("  - Left Mouse Drag: Rotate camera");
    info!("  - Shift + Left Mouse Drag: Pan camera");
    info!("  - Mouse Wheel: Zoom in/out");
    info!("  - R: Reset camera");
    info!("  - H: Toggle UI");
    info!("  - ESC: Exit");

    // Create event loop and window
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Scientific Visualizer - 3D Scatter Plot with UI")
            .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            .build(&event_loop)?,
    );

    // Initialize GPU context
    let mut render_context = pollster::block_on(RenderContext::new(window.clone()))?;

    info!(
        "GPU initialized: {} ({:?})",
        render_context.adapter_info.name, render_context.adapter_info.backend
    );

    // Initialize UI
    let mut ui_context = UiContext::new(
        &render_context.device,
        render_context.config.format,
        &window,
    );

    // Create camera
    let mut camera = OrbitalCamera::new(Vec3::ZERO, 20.0, render_context.aspect_ratio());

    // Generate datasets
    let datasets = vec![generate_spiral_points(1000), generate_cube_points(10000)];
    let dataset_names: Vec<String> = datasets.iter().map(|d| d.name().to_string()).collect();

    // Create scatter plot with first dataset
    let mut scatter = Scatter3D::new(&render_context, &datasets[0])?;
    info!(
        "Scatter plot initialized with {} points",
        scatter.point_count()
    );

    // Frame the camera to see all points
    let bounds = datasets[0].bounds();
    camera.frame_bounds(bounds.min, bounds.max, 0.2);

    // UI state
    let mut control_panel = ControlPanel::default();
    let mut performance_metrics = PerformanceMetrics::new(100);
    let mut show_ui = true;

    // Input state
    let mut input_state = InputState::new();

    // Main event loop
    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                event: ref window_event,
                window_id,
            } if window_id == window.id() => {
                // Let UI handle event first
                let ui_consumed = if show_ui {
                    ui_context.handle_event(&window, window_event)
                } else {
                    false
                };

                // Only process 3D controls if UI didn't consume the event
                if !ui_consumed {
                    match window_event {
                        WindowEvent::CloseRequested => {
                            info!("Close requested, exiting");
                            elwt.exit();
                        }

                        WindowEvent::Resized(physical_size) => {
                            render_context.resize(physical_size.width, physical_size.height);
                            camera.set_aspect(render_context.aspect_ratio());
                        }

                        WindowEvent::MouseInput { state, button, .. } => {
                            if *button == MouseButton::Left {
                                input_state.mouse_pressed = *state == ElementState::Pressed;
                            }
                        }

                        WindowEvent::CursorMoved { position, .. } => {
                            let current_pos = (position.x as f32, position.y as f32);

                            if input_state.mouse_pressed && !ui_context.wants_pointer_input() {
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
                            if !ui_context.wants_pointer_input() {
                                let zoom_delta = match delta {
                                    MouseScrollDelta::LineDelta(_, y) => *y * 0.1,
                                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 * 0.001,
                                };
                                camera.zoom(zoom_delta);
                            }
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
                                            let bounds = datasets[control_panel.dataset_index].bounds();
                                            camera.frame_bounds(bounds.min, bounds.max, 0.2);
                                        }
                                        KeyCode::KeyH => {
                                            show_ui = !show_ui;
                                            info!("UI {}", if show_ui { "shown" } else { "hidden" });
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
                            // Update performance metrics
                            performance_metrics.record_frame();

                            // Begin UI frame
                            if show_ui {
                                let ctx = ui_context.begin_frame(&window);

                                // Draw performance panel
                                performance_panel(&ctx, &performance_metrics);

                                // Draw control panel
                                let old_dataset = control_panel.dataset_index;
                                let old_point_size = control_panel.point_size;

                                let dataset_refs: Vec<&str> = dataset_names.iter().map(|s| s.as_str()).collect();
                                control_panel.show(&ctx, &dataset_refs);

                                // Handle dataset change
                                if control_panel.dataset_index != old_dataset {
                                    info!(
                                        "Switching to dataset: {}",
                                        datasets[control_panel.dataset_index].name()
                                    );
                                    scatter =
                                        Scatter3D::new(&render_context, &datasets[control_panel.dataset_index])
                                            .unwrap();
                                    let bounds = datasets[control_panel.dataset_index].bounds();
                                    camera.frame_bounds(bounds.min, bounds.max, 0.2);
                                }

                                // Handle point size change
                                if control_panel.point_size != old_point_size {
                                    scatter.set_point_size(control_panel.point_size);
                                }
                            }

                            // Update camera uniforms
                            scatter.update_camera(&render_context, &camera);

                            // Render
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

                                    // Render 3D scene
                                    {
                                        let mut render_pass =
                                            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                                label: Some("3D Render Pass"),
                                                color_attachments: &[Some(
                                                    wgpu::RenderPassColorAttachment {
                                                        view: &view,
                                                        resolve_target: None,
                                                        ops: wgpu::Operations {
                                                            load: wgpu::LoadOp::Clear(
                                                                control_panel.background_wgpu_color(),
                                                            ),
                                                            store: wgpu::StoreOp::Store,
                                                        },
                                                    },
                                                )],
                                                depth_stencil_attachment: None,
                                                timestamp_writes: None,
                                                occlusion_query_set: None,
                                            });

                                        scatter.render(&mut render_pass);
                                    }

                                    // Render UI
                                    if show_ui {
                                        let full_output = ui_context.end_frame(&window);
                                        ui_context
                                            .render(
                                                &render_context.device,
                                                &render_context.queue,
                                                &mut encoder,
                                                &window,
                                                &view,
                                                full_output,
                                            )
                                            .unwrap();
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
            }

            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => {}
        }
    })?;

    Ok(())
}
