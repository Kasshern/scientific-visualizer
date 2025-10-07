use glam::{Mat4, Quat, Vec3};

/// 3D transformation with translation, rotation, and scale
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    /// Create a new identity transform
    pub fn identity() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Create a transform with only translation
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Self::identity()
        }
    }

    /// Create a transform with only rotation
    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            rotation,
            ..Self::identity()
        }
    }

    /// Create a transform with only scale
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Self::identity()
        }
    }

    /// Create a transform with uniform scale
    pub fn from_uniform_scale(scale: f32) -> Self {
        Self {
            scale: Vec3::splat(scale),
            ..Self::identity()
        }
    }

    /// Convert the transform to a 4x4 matrix
    ///
    /// # Examples
    /// ```
    /// use glam::{Vec3, Quat};
    /// use viz_core::math::Transform;
    ///
    /// let transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
    /// let matrix = transform.to_matrix();
    /// // Matrix can be used directly with GPU
    /// ```
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Apply this transform to a point
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.to_matrix().transform_point3(point)
    }

    /// Apply this transform to a vector (no translation)
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        self.to_matrix().transform_vector3(vector)
    }

    /// Combine two transforms (self * other)
    pub fn mul_transform(&self, other: &Transform) -> Transform {
        Transform {
            translation: self.transform_point(other.translation),
            rotation: self.rotation * other.rotation,
            scale: self.scale * other.scale,
        }
    }

    /// Get the inverse of this transform
    pub fn inverse(&self) -> Transform {
        let inv_rotation = self.rotation.inverse();
        let inv_scale = Vec3::ONE / self.scale;
        let inv_translation = -(inv_rotation * (self.translation / self.scale));

        Transform {
            translation: inv_translation,
            rotation: inv_rotation,
            scale: inv_scale,
        }
    }

    /// Translate the transform by a vector
    pub fn translate(&mut self, delta: Vec3) {
        self.translation += delta;
    }

    /// Rotate the transform by a quaternion
    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }

    /// Scale the transform
    pub fn apply_scale(&mut self, scale: Vec3) {
        self.scale *= scale;
    }

    /// Look at a target position from a given position
    ///
    /// # Examples
    /// ```
    /// use glam::Vec3;
    /// use viz_core::math::Transform;
    ///
    /// let transform = Transform::look_at(
    ///     Vec3::new(0.0, 0.0, 10.0), // eye position
    ///     Vec3::ZERO,                 // target
    ///     Vec3::Y                     // up
    /// );
    /// ```
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - eye).normalize();
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        let rotation = Quat::from_mat3(&glam::Mat3::from_cols(right, up, forward));

        Self {
            translation: eye,
            rotation,
            scale: Vec3::ONE,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: Vec3, b: Vec3, epsilon: f32) -> bool {
        (a - b).length() < epsilon
    }

    #[test]
    fn test_identity() {
        let transform = Transform::identity();
        assert_eq!(transform.translation, Vec3::ZERO);
        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(transform.scale, Vec3::ONE);
    }

    #[test]
    fn test_to_matrix() {
        let transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let matrix = transform.to_matrix();
        let point = matrix.transform_point3(Vec3::ZERO);
        assert!(approx_eq(point, Vec3::new(1.0, 2.0, 3.0), 1e-6));
    }

    #[test]
    fn test_transform_point() {
        let transform = Transform::from_translation(Vec3::new(1.0, 0.0, 0.0));
        let point = transform.transform_point(Vec3::new(1.0, 0.0, 0.0));
        assert!(approx_eq(point, Vec3::new(2.0, 0.0, 0.0), 1e-6));
    }

    #[test]
    fn test_inverse() {
        let transform = Transform::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let inverse = transform.inverse();
        let combined = transform.mul_transform(&inverse);

        assert!(approx_eq(combined.translation, Vec3::ZERO, 1e-5));

        // Check rotation is close to identity by checking dot product
        let dot = combined.rotation.dot(Quat::IDENTITY).abs();
        assert!((dot - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_uniform_scale() {
        let transform = Transform::from_uniform_scale(2.0);
        assert_eq!(transform.scale, Vec3::splat(2.0));
    }
}
