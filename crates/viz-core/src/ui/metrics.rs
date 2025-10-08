use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Performance metrics tracker for visualization
///
/// Tracks FPS, frame times, and provides smoothed averages
/// for display in the UI.
#[derive(Debug)]
pub struct PerformanceMetrics {
    /// Recent frame times (in milliseconds)
    frame_times: VecDeque<f32>,

    /// Maximum number of frames to track
    max_samples: usize,

    /// Last frame timestamp
    last_frame: Instant,

    /// Total frames rendered
    total_frames: u64,

    /// Time of first frame
    start_time: Instant,
}

impl PerformanceMetrics {
    /// Create a new performance metrics tracker
    ///
    /// # Arguments
    /// * `max_samples` - Maximum number of frame times to track (default: 100)
    pub fn new(max_samples: usize) -> Self {
        let now = Instant::now();
        Self {
            frame_times: VecDeque::with_capacity(max_samples),
            max_samples,
            last_frame: now,
            total_frames: 0,
            start_time: now,
        }
    }

    /// Record a new frame
    pub fn record_frame(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame).as_secs_f32() * 1000.0; // ms

        self.frame_times.push_back(frame_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.pop_front();
        }

        self.last_frame = now;
        self.total_frames += 1;
    }

    /// Get current FPS (based on last frame time)
    pub fn current_fps(&self) -> f32 {
        if let Some(&last_frame_time) = self.frame_times.back() {
            if last_frame_time > 0.0 {
                1000.0 / last_frame_time
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Get average FPS over recent frames
    pub fn average_fps(&self) -> f32 {
        let avg_frame_time = self.average_frame_time();
        if avg_frame_time > 0.0 {
            1000.0 / avg_frame_time
        } else {
            0.0
        }
    }

    /// Get average frame time in milliseconds
    pub fn average_frame_time(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let sum: f32 = self.frame_times.iter().sum();
        sum / self.frame_times.len() as f32
    }

    /// Get minimum frame time (best frame)
    pub fn min_frame_time(&self) -> f32 {
        self.frame_times
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    /// Get maximum frame time (worst frame)
    pub fn max_frame_time(&self) -> f32 {
        self.frame_times
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0)
    }

    /// Get frame times for plotting
    pub fn frame_times(&self) -> &VecDeque<f32> {
        &self.frame_times
    }

    /// Get total frames rendered
    pub fn total_frames(&self) -> u64 {
        self.total_frames
    }

    /// Get total elapsed time
    pub fn elapsed_time(&self) -> Duration {
        Instant::now().duration_since(self.start_time)
    }

    /// Get overall average FPS since start
    pub fn overall_average_fps(&self) -> f32 {
        let elapsed_secs = self.elapsed_time().as_secs_f32();
        if elapsed_secs > 0.0 {
            self.total_frames as f32 / elapsed_secs
        } else {
            0.0
        }
    }

    /// Reset metrics
    pub fn reset(&mut self) {
        self.frame_times.clear();
        self.total_frames = 0;
        self.start_time = Instant::now();
        self.last_frame = Instant::now();
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self::new(100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_new() {
        let metrics = PerformanceMetrics::new(50);
        assert_eq!(metrics.total_frames(), 0);
        assert_eq!(metrics.current_fps(), 0.0);
    }

    #[test]
    fn test_record_frame() {
        let mut metrics = PerformanceMetrics::new(10);

        // Sleep a bit to ensure measurable time passes
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
        metrics.record_frame();

        assert_eq!(metrics.total_frames(), 1);
        assert!(metrics.current_fps() > 0.0);
    }

    #[test]
    fn test_max_samples() {
        let mut metrics = PerformanceMetrics::new(5);

        for _ in 0..10 {
            thread::sleep(Duration::from_millis(1));
            metrics.record_frame();
        }

        assert_eq!(metrics.frame_times().len(), 5); // Should cap at max_samples
        assert_eq!(metrics.total_frames(), 10); // But total frames keeps counting
    }

    #[test]
    fn test_reset() {
        let mut metrics = PerformanceMetrics::new(10);

        metrics.record_frame();
        metrics.record_frame();

        assert_eq!(metrics.total_frames(), 2);

        metrics.reset();

        assert_eq!(metrics.total_frames(), 0);
        assert_eq!(metrics.frame_times().len(), 0);
    }
}
