// Scatter plot shaders for point rendering

// Camera uniforms passed from CPU
struct CameraUniforms {
    view_proj: mat4x4<f32>,  // Combined view-projection matrix
    view_pos: vec3<f32>,      // Camera position in world space
    _padding: f32,            // Alignment padding
}

@group(0) @binding(0)
var<uniform> camera: CameraUniforms;

// Vertex input from vertex buffer
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

// Output from vertex shader to fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_pos: vec3<f32>,
    @location(2) distance: f32,
}

// Vertex shader
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position to clip space
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 1.0);

    // Pass through color
    out.color = in.color;

    // Store world position for fragment shader
    out.world_pos = in.position;

    // Calculate distance from camera for depth-based effects
    out.distance = length(in.position - camera.view_pos);

    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Distance-based fading (optional, can be toggled)
    let fade_start = 50.0;
    let fade_end = 100.0;
    let fade = 1.0 - smoothstep(fade_start, fade_end, in.distance);

    // Apply fade to alpha
    var color = in.color;
    color.a *= fade;

    // Discard fully transparent fragments
    if (color.a < 0.01) {
        discard;
    }

    return color;
}
