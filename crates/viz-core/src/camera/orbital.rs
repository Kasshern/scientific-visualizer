use glam::{Mat4, Vec3};
use std::f32::consts::{FRAC_PI_2, PI};

/// Orbital camera for 3D visualization with smooth controls
///
/// The camera orbits around a target point at a fixed distance,
/// controlled by yaw (horizontal) and pitch (vertical) angles.
///
/// # Examples
/// ```
/// use viz_core::camera::OrbitalCamera;
/// use glam::Vec3;
///
/// let mut camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
/// camera.rotate(0.1, 0.05); // Rotate by delta angles
/// let view_proj = camera.view_projection_matrix();
/// ```
#[derive(Debug, Clone)]
pub struct OrbitalCamera {
    /// Target point the camera looks at
    pub target: Vec3,

    /// Distance from target
    pub distance: f32,

    /// Horizontal rotation in radians (around Y axis)
    pub yaw: f32,

    /// Vertical rotation in radians
    pub pitch: f32,

    /// Field of view in radians
    pub fov: f32,

    /// Aspect ratio (width / height)
    pub aspect: f32,

    /// Near clipping plane
    pub near: f32,

    /// Far clipping plane
    pub far: f32,
}

impl OrbitalCamera {
    /// Create a new orbital camera
    ///
    /// # Arguments
    /// * `target` - Point to look at
    /// * `distance` - Distance from target
    /// * `aspect` - Aspect ratio (width / height)
    pub fn new(target: Vec3, distance: f32, aspect: f32) -> Self {
        Self {
            target,
            distance,
            yaw: 0.0,
            pitch: 0.0,
            fov: PI / 4.0, // 45 degrees
            aspect,
            near: 0.1,
            far: 1000.0,
        }
    }

    /// Compute the camera's position in world space
    pub fn position(&self) -> Vec3 {
        let x = self.distance * self.pitch.cos() * self.yaw.sin();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * self.pitch.cos() * self.yaw.cos();
        self.target + Vec3::new(x, y, z)
    }

    /// Get the forward direction (from camera to target)
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position()).normalize()
    }

    /// Get the right direction
    pub fn right(&self) -> Vec3 {
        self.forward().cross(Vec3::Y).normalize()
    }

    /// Get the up direction
    pub fn up(&self) -> Vec3 {
        self.right().cross(self.forward()).normalize()
    }

    /// Compute the view matrix (world to camera space)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position(), self.target, Vec3::Y)
    }

    /// Compute the projection matrix (camera to clip space)
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
    }

    /// Compute combined view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Rotate the camera by delta angles
    ///
    /// # Arguments
    /// * `delta_yaw` - Change in yaw (radians)
    /// * `delta_pitch` - Change in pitch (radians)
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw;

        // Normalize yaw to [0, 2π]
        self.yaw = self.yaw.rem_euclid(2.0 * PI);

        // Clamp pitch to prevent gimbal lock
        self.pitch = (self.pitch + delta_pitch)
            .clamp(-FRAC_PI_2 + 0.01, FRAC_PI_2 - 0.01);
    }

    /// Zoom the camera (change distance to target)
    ///
    /// # Arguments
    /// * `delta` - Relative change in distance (e.g., 0.1 = 10% closer)
    pub fn zoom(&mut self, delta: f32) {
        self.distance = (self.distance * (1.0 - delta)).clamp(0.1, 1000.0);
    }

    /// Pan the camera (move target point)
    ///
    /// # Arguments
    /// * `delta_x` - Horizontal movement in screen space
    /// * `delta_y` - Vertical movement in screen space
    pub fn pan(&mut self, delta_x: f32, delta_y: f32) {
        let right = self.right();
        let up = self.up();

        // Scale pan speed by distance
        let pan_speed = self.distance * 0.001;

        self.target += right * delta_x * pan_speed;
        self.target += up * delta_y * pan_speed;
    }

    /// Update aspect ratio (call when window resizes)
    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    /// Set field of view in degrees
    pub fn set_fov_degrees(&mut self, fov_degrees: f32) {
        self.fov = fov_degrees.to_radians();
    }

    /// Reset camera to default view
    pub fn reset(&mut self) {
        self.yaw = 0.0;
        self.pitch = 0.0;
        self.distance = 10.0;
        self.target = Vec3::ZERO;
    }

    /// Frame all points (adjust camera to see all points)
    ///
    /// # Arguments
    /// * `points` - Points to frame
    /// * `padding` - Extra space around bounds (e.g., 0.1 = 10% padding)
    pub fn frame_bounds(&mut self, min: Vec3, max: Vec3, padding: f32) {
        // Set target to center of bounds
        self.target = (min + max) * 0.5;

        // Calculate required distance to see all points
        let size = (max - min).length();
        self.distance = (size * (1.0 + padding)) / (2.0 * (self.fov / 2.0).tan());

        // Clamp to reasonable values
        self.distance = self.distance.clamp(0.1, 1000.0);
    }
}

impl Default for OrbitalCamera {
    fn default() -> Self {
        Self::new(Vec3::ZERO, 10.0, 16.0 / 9.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: Vec3, b: Vec3, epsilon: f32) -> bool {
        (a - b).length() < epsilon
    }

    #[test]
    fn test_new() {
        let camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
        assert_eq!(camera.target, Vec3::ZERO);
        assert_eq!(camera.distance, 10.0);
        assert_eq!(camera.aspect, 1.77);
    }

    #[test]
    fn test_position() {
        let camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
        let pos = camera.position();

        // At yaw=0, pitch=0, camera should be at (0, 0, 10)
        assert!(approx_eq(pos, Vec3::new(0.0, 0.0, 10.0), 1e-5));
    }

    #[test]
    fn test_rotate() {
        let mut camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
        camera.rotate(PI / 2.0, 0.0); // 90 degrees yaw

        let pos = camera.position();
        // Should now be at (10, 0, 0) approximately
        assert!(approx_eq(pos, Vec3::new(10.0, 0.0, 0.0), 1e-5));
    }

    #[test]
    fn test_pitch_clamping() {
        let mut camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);

        // Try to pitch beyond limit
        camera.rotate(0.0, 10.0); // Way beyond vertical

        // Should be clamped
        assert!(camera.pitch < FRAC_PI_2);
        assert!(camera.pitch > -FRAC_PI_2);
    }

    #[test]
    fn test_zoom() {
        let mut camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
        camera.zoom(0.5); // Zoom in 50%

        assert!((camera.distance - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_zoom_clamping() {
        let mut camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
        camera.zoom(1.5); // Try to zoom past minimum

        assert!(camera.distance >= 0.1);
    }

    #[test]
    fn test_view_matrix() {
        let camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
        let view = camera.view_matrix();

        // View matrix should have determinant of 1 (it's orthonormal with translation)
        assert!((view.determinant().abs() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_projection_matrix() {
        let camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
        let proj = camera.projection_matrix();

        // Projection matrix should not be zero
        assert!(proj.determinant().abs() > 1e-10);
    }

    #[test]
    fn test_frame_bounds() {
        let mut camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);
        let min = Vec3::new(-5.0, -5.0, -5.0);
        let max = Vec3::new(5.0, 5.0, 5.0);

        camera.frame_bounds(min, max, 0.1);

        // Target should be at center
        assert!(approx_eq(camera.target, Vec3::ZERO, 1e-5));

        // Distance should be adjusted
        assert!(camera.distance > 0.0);
    }

    #[test]
    fn test_reset() {
        let mut camera = OrbitalCamera::new(Vec3::new(5.0, 5.0, 5.0), 20.0, 1.77);
        camera.rotate(1.0, 0.5);
        camera.zoom(0.5);

        camera.reset();

        assert_eq!(camera.yaw, 0.0);
        assert_eq!(camera.pitch, 0.0);
        assert_eq!(camera.distance, 10.0);
        assert_eq!(camera.target, Vec3::ZERO);
    }

    #[test]
    fn test_directions() {
        let camera = OrbitalCamera::new(Vec3::ZERO, 10.0, 1.77);

        let forward = camera.forward();
        let right = camera.right();
        let up = camera.up();

        // All should be unit vectors
        assert!((forward.length() - 1.0).abs() < 1e-5);
        assert!((right.length() - 1.0).abs() < 1e-5);
        assert!((up.length() - 1.0).abs() < 1e-5);

        // Should be orthogonal
        assert!(forward.dot(right).abs() < 1e-5);
        assert!(forward.dot(up).abs() < 1e-5);
        assert!(right.dot(up).abs() < 1e-5);
    }
}
