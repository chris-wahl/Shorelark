use crate::*;
use std::f32::consts::{FRAC_PI_4, PI};

/// FOV_RANGE determines how far the bird can see
/// - 0.1 = 10% of the map
/// - 0.5 = 50% of the map
/// - 1.0 = 100% of the map
const FOV_RANGE: f32 = 0.25;

/// FOV_ANGLE determines the range of vision the bird has
const FOV_ANGLE: f32 = PI * FRAC_PI_4;

// CELLS ar the number of photoreceptors in a single eye
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    pub(crate) fov_range: f32,
    pub(crate) fov_angle: f32,
    pub(crate) cells: usize,
}

impl Eye {
    pub fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        return Self { fov_range, fov_angle, cells };
    }

    pub fn cells(&self) -> usize {
        return self.cells;
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food]
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();

            if dist >= self.fov_range {
                // 1. If the distance between the food and the bird is outside the bird's field of
                //    view RANGE, this food is not seen.
                continue;
            }

            let angle = na::Rotation2::rotation_between(
                &na::Vector2::x(),
                &vec
            ).angle() // The vector direction relative to the X axis
                - rotation.angle(); // AND include the bird's rotation

            let angle = na::wrap(angle, -PI, PI); // And wrap around PI

            // And finally, check if this angle is within the bird's FOV
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                // Outside the bird's fov angle.  Move on to the next food.
                continue;
            }

            // At this point, the food is within the bird's vision FOV.
            // Determine which photoreceptor sees it by looking at the angle between the food and
            // the bird's eye.

            // Know that the angle is within the fov, so transform this into the range <0, 1>,
            let angle = angle + self.fov_angle / 2.0;
            // This can be treated as a percentage such that
            // - 0.2 = the food is seen by the 20%-th eye (to the left)
            // - 0.5 = directly in front of the bird
            // - 0.8 = to the right
            // Use this to get the photoreceptor number.
            // - 0.2 * 8 photoreceptors = 1.6 ~= 1 (2nd cell - 0-indexed)
            // - 0.5 * 8 photoreceptors = 4.0 ~= 4 (5th cell)
            // - 0.8 * 8 photoreceptors = 6.4 ~= 6 (seventh cell)
            let cell = angle / self.fov_angle * (self.cells as f32);

            // Convert `cell` to `usize` so it can be used as an index.
            // Also calling `min` to cover an extreme edge case: for cell=1.0 (food all the way
            // to the far right of the bird's vision), would get a `cell` value of the length of the
            // cells array - which is out of bounds.  Using `min` to force the "final" photoreceptor
            // to cover the 1.0 value.
            let cell = (cell as usize).min(cells.len() - 1);

            // The "intensity" of the food in the bird's vision.
            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;

        }

        return cells;
    }
}

impl Default for Eye {
    fn default() -> Self {
        return Self::new(FOV_RANGE, FOV_ANGLE, CELLS);
    }
}