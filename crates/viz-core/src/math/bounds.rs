use glam::Vec3;

/// Axis-aligned bounding box (AABB) in 3D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds3D {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds3D {
    /// Create a new bounding box from min and max points
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Create a bounding box that encompasses all given points
    ///
    /// # Examples
    /// ```
    /// use glam::Vec3;
    /// use viz_core::math::Bounds3D;
    ///
    /// let points = vec![
    ///     Vec3::new(0.0, 0.0, 0.0),
    ///     Vec3::new(1.0, 2.0, 3.0),
    ///     Vec3::new(-1.0, 1.0, 0.5),
    /// ];
    /// let bounds = Bounds3D::from_points(&points);
    /// assert_eq!(bounds.min, Vec3::new(-1.0, 0.0, 0.0));
    /// assert_eq!(bounds.max, Vec3::new(1.0, 2.0, 3.0));
    /// ```
    pub fn from_points(points: &[Vec3]) -> Self {
        if points.is_empty() {
            return Self::zero();
        }

        let mut min = points[0];
        let mut max = points[0];

        for &point in points.iter().skip(1) {
            min = min.min(point);
            max = max.max(point);
        }

        Self { min, max }
    }

    /// Create a zero-size bounding box at the origin
    pub fn zero() -> Self {
        Self {
            min: Vec3::ZERO,
            max: Vec3::ZERO,
        }
    }

    /// Create a bounding box centered at origin with given size
    pub fn centered(size: f32) -> Self {
        let half = size / 2.0;
        Self {
            min: Vec3::splat(-half),
            max: Vec3::splat(half),
        }
    }

    /// Get the center point of the bounding box
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Get the size (dimensions) of the bounding box
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Get the extents (half-size) of the bounding box
    pub fn extents(&self) -> Vec3 {
        self.size() * 0.5
    }

    /// Get the diagonal length of the bounding box
    pub fn diagonal(&self) -> f32 {
        self.size().length()
    }

    /// Check if the bounding box contains a point
    pub fn contains(&self, point: Vec3) -> bool {
        point.cmpge(self.min).all() && point.cmple(self.max).all()
    }

    /// Check if this bounding box intersects another
    pub fn intersects(&self, other: &Bounds3D) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }

    /// Expand the bounding box to include a point
    pub fn expand_to_include(&mut self, point: Vec3) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    /// Expand the bounding box to include another bounding box
    pub fn expand_to_include_bounds(&mut self, other: &Bounds3D) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
    }

    /// Create a new bounding box that includes both this and another
    pub fn union(&self, other: &Bounds3D) -> Bounds3D {
        Bounds3D {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Create a padded version of this bounding box
    pub fn padded(&self, padding: f32) -> Bounds3D {
        Bounds3D {
            min: self.min - Vec3::splat(padding),
            max: self.max + Vec3::splat(padding),
        }
    }

    /// Get the 8 corner points of the bounding box
    pub fn corners(&self) -> [Vec3; 8] {
        [
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
        ]
    }
}

impl Default for Bounds3D {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_points() {
        let points = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(-1.0, 1.0, 0.5),
        ];
        let bounds = Bounds3D::from_points(&points);
        assert_eq!(bounds.min, Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(bounds.max, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_center() {
        let bounds = Bounds3D::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(bounds.center(), Vec3::ZERO);
    }

    #[test]
    fn test_size() {
        let bounds = Bounds3D::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(bounds.size(), Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_contains() {
        let bounds = Bounds3D::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        assert!(bounds.contains(Vec3::ZERO));
        assert!(bounds.contains(Vec3::new(0.5, 0.5, 0.5)));
        assert!(!bounds.contains(Vec3::new(2.0, 0.0, 0.0)));
    }

    #[test]
    fn test_intersects() {
        let bounds1 = Bounds3D::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 2.0, 2.0));
        let bounds2 = Bounds3D::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(3.0, 3.0, 3.0));
        let bounds3 = Bounds3D::new(Vec3::new(3.0, 3.0, 3.0), Vec3::new(4.0, 4.0, 4.0));

        assert!(bounds1.intersects(&bounds2));
        assert!(!bounds1.intersects(&bounds3));
    }

    #[test]
    fn test_diagonal() {
        let bounds = Bounds3D::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let expected = (3.0_f32).sqrt();
        assert!((bounds.diagonal() - expected).abs() < 1e-6);
    }

    #[test]
    fn test_corners() {
        let bounds = Bounds3D::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
        let corners = bounds.corners();
        assert_eq!(corners.len(), 8);
        assert!(corners.contains(&Vec3::new(0.0, 0.0, 0.0)));
        assert!(corners.contains(&Vec3::new(1.0, 1.0, 1.0)));
    }
}
