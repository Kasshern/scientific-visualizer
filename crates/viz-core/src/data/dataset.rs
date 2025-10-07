use crate::math::Bounds3D;
use wgpu::{Buffer, Device};

/// Generic trait for all visualization datasets
pub trait Dataset: Send + Sync {
    /// Get the bounding box encompassing all data
    fn bounds(&self) -> Bounds3D;

    /// Number of elements in the dataset
    fn len(&self) -> usize;

    /// Check if dataset is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get a human-readable name for this dataset
    fn name(&self) -> &str {
        "Unnamed Dataset"
    }
}
