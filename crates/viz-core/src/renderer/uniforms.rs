use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};

/// Camera uniforms for GPU (matches WGSL struct)
///
/// This structure is passed to shaders via a uniform buffer.
/// Layout must match the WGSL CameraUniforms struct exactly.
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniforms {
    /// Combined view-projection matrix (64 bytes)
    pub view_proj: [[f32; 4]; 4],

    /// Camera position in world space (12 bytes)
    pub view_pos: [f32; 3],

    /// Padding for alignment (4 bytes)
    pub _padding: f32,
}

impl CameraUniforms {
    /// Create camera uniforms from view-projection matrix and position
    pub fn new(view_proj: Mat4, view_pos: Vec3) -> Self {
        Self {
            view_proj: view_proj.to_cols_array_2d(),
            view_pos: view_pos.to_array(),
            _padding: 0.0,
        }
    }

    /// Update from camera
    pub fn update(&mut self, view_proj: Mat4, view_pos: Vec3) {
        self.view_proj = view_proj.to_cols_array_2d();
        self.view_pos = view_pos.to_array();
    }
}

impl Default for CameraUniforms {
    fn default() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
            view_pos: [0.0, 0.0, 0.0],
            _padding: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        // Must be 80 bytes (64 for mat4x4 + 12 for vec3 + 4 for padding)
        assert_eq!(std::mem::size_of::<CameraUniforms>(), 80);
    }

    #[test]
    fn test_alignment() {
        // Alignment is determined by largest field (f32 array = 4 bytes)
        // This is fine for WGPU uniform buffers
        assert!(std::mem::align_of::<CameraUniforms>() >= 4);
    }

    #[test]
    fn test_new() {
        let view_proj = Mat4::IDENTITY;
        let view_pos = Vec3::new(1.0, 2.0, 3.0);

        let uniforms = CameraUniforms::new(view_proj, view_pos);

        assert_eq!(uniforms.view_pos, [1.0, 2.0, 3.0]);
        assert_eq!(uniforms.view_proj, Mat4::IDENTITY.to_cols_array_2d());
    }

    #[test]
    fn test_update() {
        let mut uniforms = CameraUniforms::default();

        let new_view_proj = Mat4::from_scale(Vec3::splat(2.0));
        let new_view_pos = Vec3::new(5.0, 10.0, 15.0);

        uniforms.update(new_view_proj, new_view_pos);

        assert_eq!(uniforms.view_pos, [5.0, 10.0, 15.0]);
    }
}
