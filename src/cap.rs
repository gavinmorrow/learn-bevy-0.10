/// A struct that represents the minimum and maximum values for a value.
#[derive(Clone, Copy, Debug)]
pub struct Cap {
    pub min: f32,
    pub max: f32,
}

impl Cap {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

impl Cap {
    /// Caps a value between the min and max values.
    pub fn apply(&self, value: f32) -> f32 {
        value.min(self.max).max(self.min)
    }

    /// Tests a value against the min and max values.
    ///
    /// # Returns
    ///
    /// * `std::cmp::Ordering::Less` if the value is less than the min value.
    /// * `std::cmp::Ordering::Greater` if the value is greater than the max value.
    /// * `std::cmp::Ordering::Equal` if the value is between the min and max values (inclusive).
    pub fn test(&self, value: f32) -> std::cmp::Ordering {
        if value < self.min {
            std::cmp::Ordering::Less
        } else if value > self.max {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

/// Calculates the bounds for a sprite in a window.
///
/// # Arguments
///
/// * `window` - The dimensions of the window.
/// * `sprite` - The dimensions of the sprite.
///
/// # Returns
///
/// An array of [`Cap`]s that represent the minimum and maximum values
/// for each dimension (in the same order as the input arrays).
pub fn calc_cap<const D: usize>(window: [f32; D], sprite: [f32; D]) -> [Cap; D] {
    let mut caps = [Cap::new(0.0, 0.0); D];

    for (i, (window, sprite)) in window.iter().zip(sprite.iter()).enumerate() {
        let half_width = sprite / 2.0;
        let min = 0.0 + half_width;
        let max = window - half_width;

        caps[i] = Cap::new(min, max);
    }

    caps
}
