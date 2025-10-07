use super::Dataset;
use crate::math::Bounds3D;
use glam::{Vec3, Vec4};
use std::collections::HashMap;

/// 3D point cloud dataset with optional colors and metadata
///
/// # Examples
/// ```
/// use viz_core::data::PointCloud;
/// use glam::{Vec3, Vec4};
///
/// let positions = vec![
///     Vec3::new(0.0, 0.0, 0.0),
///     Vec3::new(1.0, 1.0, 1.0),
///     Vec3::new(-1.0, 0.5, 0.2),
/// ];
///
/// let mut cloud = PointCloud::new(positions);
/// cloud = cloud.with_colors(vec![
///     Vec4::new(1.0, 0.0, 0.0, 1.0),
///     Vec4::new(0.0, 1.0, 0.0, 1.0),
///     Vec4::new(0.0, 0.0, 1.0, 1.0),
/// ]);
///
/// assert_eq!(cloud.len(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct PointCloud {
    /// Point positions in 3D space
    positions: Vec<Vec3>,

    /// Optional colors (RGBA) for each point
    colors: Option<Vec<Vec4>>,

    /// Optional per-point sizes
    sizes: Option<Vec<f32>>,

    /// Additional metadata fields (for color mapping, filtering, etc.)
    metadata: HashMap<String, Vec<f32>>,

    /// Cached bounding box
    bounds: Option<Bounds3D>,

    /// Dataset name
    name: String,
}

impl PointCloud {
    /// Create a new point cloud from positions
    pub fn new(positions: Vec<Vec3>) -> Self {
        Self {
            positions,
            colors: None,
            sizes: None,
            metadata: HashMap::new(),
            bounds: None,
            name: String::from("Point Cloud"),
        }
    }

    /// Set colors for all points
    ///
    /// # Panics
    /// Panics if colors.len() != positions.len()
    pub fn with_colors(mut self, colors: Vec<Vec4>) -> Self {
        assert_eq!(
            colors.len(),
            self.positions.len(),
            "Colors length must match positions length"
        );
        self.colors = Some(colors);
        self
    }

    /// Set sizes for all points
    ///
    /// # Panics
    /// Panics if sizes.len() != positions.len()
    pub fn with_sizes(mut self, sizes: Vec<f32>) -> Self {
        assert_eq!(
            sizes.len(),
            self.positions.len(),
            "Sizes length must match positions length"
        );
        self.sizes = Some(sizes);
        self
    }

    /// Add metadata field
    ///
    /// # Panics
    /// Panics if values.len() != positions.len()
    pub fn with_metadata(mut self, key: String, values: Vec<f32>) -> Self {
        assert_eq!(
            values.len(),
            self.positions.len(),
            "Metadata length must match positions length"
        );
        self.metadata.insert(key, values);
        self
    }

    /// Set the dataset name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Get point positions
    pub fn positions(&self) -> &[Vec3] {
        &self.positions
    }

    /// Get point colors (or None if not set)
    pub fn colors(&self) -> Option<&[Vec4]> {
        self.colors.as_deref()
    }

    /// Get point sizes (or None if not set)
    pub fn sizes(&self) -> Option<&[f32]> {
        self.sizes.as_deref()
    }

    /// Get metadata field by name
    pub fn metadata(&self, key: &str) -> Option<&[f32]> {
        self.metadata.get(key).map(|v| v.as_slice())
    }

    /// Get all metadata keys
    pub fn metadata_keys(&self) -> Vec<&String> {
        self.metadata.keys().collect()
    }

    /// Generate default colors (white) for all points
    pub fn generate_default_colors(&mut self) {
        if self.colors.is_none() {
            self.colors = Some(vec![Vec4::ONE; self.positions.len()]);
        }
    }

    /// Generate colors from height (Y coordinate)
    ///
    /// Maps Y values to a rainbow gradient
    pub fn generate_height_colors(&mut self) {
        let bounds = self.compute_bounds();
        let min_y = bounds.min.y;
        let max_y = bounds.max.y;
        let range = max_y - min_y;

        let colors: Vec<Vec4> = self
            .positions
            .iter()
            .map(|pos| {
                let t = if range > 0.0 {
                    (pos.y - min_y) / range
                } else {
                    0.5
                };

                // Rainbow gradient: red -> green -> blue
                let r = (1.0 - t).max(0.0);
                let g = (1.0 - (t - 0.5).abs() * 2.0).max(0.0);
                let b = t.max(0.0);

                Vec4::new(r, g, b, 1.0)
            })
            .collect();

        self.colors = Some(colors);
    }

    /// Compute bounding box (cached)
    fn compute_bounds(&mut self) -> Bounds3D {
        if let Some(bounds) = self.bounds {
            return bounds;
        }

        let bounds = Bounds3D::from_points(&self.positions);
        self.bounds = Some(bounds);
        bounds
    }
}

impl Dataset for PointCloud {
    fn bounds(&self) -> Bounds3D {
        self.bounds
            .unwrap_or_else(|| Bounds3D::from_points(&self.positions))
    }

    fn len(&self) -> usize {
        self.positions.len()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let positions = vec![Vec3::ZERO, Vec3::ONE];
        let cloud = PointCloud::new(positions);
        assert_eq!(cloud.len(), 2);
        assert!(!cloud.is_empty());
    }

    #[test]
    fn test_with_colors() {
        let positions = vec![Vec3::ZERO, Vec3::ONE];
        let colors = vec![Vec4::new(1.0, 0.0, 0.0, 1.0), Vec4::new(0.0, 1.0, 0.0, 1.0)];

        let cloud = PointCloud::new(positions).with_colors(colors.clone());

        assert_eq!(cloud.colors().unwrap().len(), 2);
        assert_eq!(cloud.colors().unwrap()[0], colors[0]);
    }

    #[test]
    #[should_panic(expected = "Colors length must match positions length")]
    fn test_colors_length_mismatch() {
        let positions = vec![Vec3::ZERO, Vec3::ONE];
        let colors = vec![Vec4::ONE]; // Wrong length

        let _cloud = PointCloud::new(positions).with_colors(colors);
    }

    #[test]
    fn test_with_metadata() {
        let positions = vec![Vec3::ZERO, Vec3::ONE, Vec3::X];
        let temperature = vec![20.0, 25.0, 30.0];

        let cloud = PointCloud::new(positions)
            .with_metadata("temperature".to_string(), temperature.clone());

        assert_eq!(cloud.metadata("temperature").unwrap().len(), 3);
        assert_eq!(cloud.metadata("temperature").unwrap()[1], 25.0);
        assert!(cloud.metadata("pressure").is_none());
    }

    #[test]
    fn test_bounds() {
        let positions = vec![
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 2.0, 0.0),
        ];

        let cloud = PointCloud::new(positions);
        let bounds = cloud.bounds();

        assert_eq!(bounds.min, Vec3::new(-1.0, -1.0, -1.0));
        assert_eq!(bounds.max, Vec3::new(1.0, 2.0, 1.0));
    }

    #[test]
    fn test_generate_default_colors() {
        let positions = vec![Vec3::ZERO, Vec3::ONE];
        let mut cloud = PointCloud::new(positions);

        assert!(cloud.colors().is_none());

        cloud.generate_default_colors();

        assert!(cloud.colors().is_some());
        assert_eq!(cloud.colors().unwrap().len(), 2);
        assert_eq!(cloud.colors().unwrap()[0], Vec4::ONE);
    }

    #[test]
    fn test_generate_height_colors() {
        let positions = vec![
            Vec3::new(0.0, 0.0, 0.0),  // Min height
            Vec3::new(0.0, 10.0, 0.0), // Max height
            Vec3::new(0.0, 5.0, 0.0),  // Middle
        ];

        let mut cloud = PointCloud::new(positions);
        cloud.generate_height_colors();

        let colors = cloud.colors().unwrap();
        assert_eq!(colors.len(), 3);

        // Min height should be reddish (high R, low B)
        assert!(colors[0].x > colors[0].z);

        // Max height should be bluish (low R, high B)
        assert!(colors[1].z > colors[1].x);
    }

    #[test]
    fn test_name() {
        let cloud = PointCloud::new(vec![Vec3::ZERO])
            .with_name("My Data");

        assert_eq!(cloud.name(), "My Data");
    }
}
