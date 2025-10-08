/// Type of scaling to apply when mapping values to colormap domain [0, 1]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleType {
    /// Linear scaling: t = (value - min) / (max - min)
    Linear,
    /// Logarithmic scaling: t = log(value - min + 1) / log(max - min + 1)
    Log,
}

/// Utilities for mapping data values to colormap domain [0, 1]
pub struct ColorScale;

impl ColorScale {
    /// Map a value to [0, 1] using linear scaling
    ///
    /// # Arguments
    /// * `value` - The value to map
    /// * `min` - Minimum value of the data range
    /// * `max` - Maximum value of the data range
    ///
    /// # Returns
    /// Normalized value in [0, 1], clamped to bounds
    pub fn map_linear(value: f32, min: f32, max: f32) -> f32 {
        if max <= min {
            return 0.5; // Fallback for degenerate range
        }
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    }

    /// Map a value to [0, 1] using logarithmic scaling
    ///
    /// This is useful for data spanning multiple orders of magnitude.
    /// Uses log(value - min + 1) to handle zero values gracefully.
    ///
    /// # Arguments
    /// * `value` - The value to map
    /// * `min` - Minimum value of the data range
    /// * `max` - Maximum value of the data range
    ///
    /// # Returns
    /// Normalized value in [0, 1], clamped to bounds
    pub fn map_log(value: f32, min: f32, max: f32) -> f32 {
        if max <= min {
            return 0.5; // Fallback for degenerate range
        }

        let range = max - min;
        let normalized = value - min;

        // Use log(x + 1) to handle zero values
        let log_value = (normalized + 1.0).ln();
        let log_max = (range + 1.0).ln();

        (log_value / log_max).clamp(0.0, 1.0)
    }

    /// Map a value to [0, 1] using the specified scale type
    pub fn map(value: f32, min: f32, max: f32, scale_type: ScaleType) -> f32 {
        match scale_type {
            ScaleType::Linear => Self::map_linear(value, min, max),
            ScaleType::Log => Self::map_log(value, min, max),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_scale_boundaries() {
        assert_eq!(ColorScale::map_linear(0.0, 0.0, 1.0), 0.0);
        assert_eq!(ColorScale::map_linear(1.0, 0.0, 1.0), 1.0);
        assert_eq!(ColorScale::map_linear(0.5, 0.0, 1.0), 0.5);
    }

    #[test]
    fn test_linear_scale_range() {
        assert_eq!(ColorScale::map_linear(5.0, 0.0, 10.0), 0.5);
        assert_eq!(ColorScale::map_linear(7.5, 5.0, 10.0), 0.5);
        assert_eq!(ColorScale::map_linear(-5.0, -10.0, 0.0), 0.5);
    }

    #[test]
    fn test_linear_scale_clamping() {
        assert_eq!(ColorScale::map_linear(-1.0, 0.0, 1.0), 0.0);
        assert_eq!(ColorScale::map_linear(2.0, 0.0, 1.0), 1.0);
    }

    #[test]
    fn test_linear_scale_degenerate() {
        // When min == max, return 0.5
        assert_eq!(ColorScale::map_linear(5.0, 5.0, 5.0), 0.5);
        assert_eq!(ColorScale::map_linear(10.0, 5.0, 5.0), 0.5);
    }

    #[test]
    fn test_log_scale_boundaries() {
        let t0 = ColorScale::map_log(0.0, 0.0, 1.0);
        assert_eq!(t0, 0.0);

        let t1 = ColorScale::map_log(1.0, 0.0, 1.0);
        assert_eq!(t1, 1.0);
    }

    #[test]
    fn test_log_scale_monotonic() {
        // Log scale should be monotonically increasing
        let t1 = ColorScale::map_log(1.0, 0.0, 10.0);
        let t5 = ColorScale::map_log(5.0, 0.0, 10.0);
        let t9 = ColorScale::map_log(9.0, 0.0, 10.0);

        assert!(t1 < t5);
        assert!(t5 < t9);
    }

    #[test]
    fn test_log_scale_expansion() {
        // Log scale expands lower values (gives them more color range)
        // and compresses higher values
        let linear_1 = ColorScale::map_linear(1.0, 0.0, 10.0);  // 0.1
        let log_1 = ColorScale::map_log(1.0, 0.0, 10.0);

        // Lower values get expanded - mapped to higher t values than linear
        assert!(log_1 > linear_1);

        // Verify the expansion is significant
        assert!(log_1 > 0.2);  // Should be around 0.29
    }

    #[test]
    fn test_log_scale_degenerate() {
        // When min == max, return 0.5
        assert_eq!(ColorScale::map_log(5.0, 5.0, 5.0), 0.5);
    }

    #[test]
    fn test_scale_type_dispatch() {
        let value = 5.0;
        let min = 0.0;
        let max = 10.0;

        let linear = ColorScale::map(value, min, max, ScaleType::Linear);
        let log = ColorScale::map(value, min, max, ScaleType::Log);

        assert_eq!(linear, ColorScale::map_linear(value, min, max));
        assert_eq!(log, ColorScale::map_log(value, min, max));
    }
}
