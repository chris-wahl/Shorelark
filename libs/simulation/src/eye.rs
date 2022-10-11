use std::f32::consts::{FRAC_PI_4, PI};

use crate::*;

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
        foods: &[Food],
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
                &vec,
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

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::*;

    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        foods: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(self.fov_range, self.fov_angle, TEST_EYE_CELLS);

            let actual_vision = eye.process_vision(
                na::Point2::new(self.x, self.y),
                na::Rotation2::new(self.rot),
                &self.foods,
            );
            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|cell| {
                    // As a reminder, the higher cell's value, the closer
                    // the food is:

                    if cell >= 0.7 {
                        // <0.7, 1.0>
                        // food is right in front of us
                        "#"
                    } else if cell >= 0.3 {
                        // <0.3, 0.7)
                        // food is somewhat further
                        "+"
                    } else if cell > 0.0 {
                        // <0.0, 0.3)
                        // food is pretty far away
                        "."
                    } else {
                        // 0.0
                        // no food in sight, this cell sees empty space
                        " "
                    }
                })
                .collect();
            // There's nothing special about the cell values (`0.7`, `0.3`, `0.0`) or the
            // characters (`#`, `+`, `.`).
            let actual_vision = actual_vision.join("");

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        return Food {
            position: na::Point2::new(x, y),
        };
    }

    mod different_fov_ranges {
        use test_case::test_case;

        use super::*;

        /// During tests in this module, we're using a world that looks
        /// like this:
        ///
        /// ------------
        /// |          |
        /// |          |
        /// |    @>   %|
        /// |          |
        /// |          |
        /// ------------
        ///
        /// Each test gradually reduces our birdie's field of view and
        /// compares what the birdie sees:
        ///
        /// ------------
        /// |        /.|
        /// |      /...|
        /// |    @>...%|
        /// |      \...|
        /// |        \.|
        /// ------------
        ///
        /// ------------
        /// |          |
        /// |      /.| |
        /// |    @>..|%|
        /// |      \.| |
        /// |          |
        /// ------------
        ///
        /// ------------
        /// |          |
        /// |          |
        /// |    @>.| %|
        /// |          |
        /// |          |
        /// ------------
        ///
        /// Over time, what we see is the food gradually disappearing
        /// into an emptiness:
        ///
        /// (well, technically the food and bird remain stationary - it's
        /// only the birdie's own field of view that gets reduced.)
        #[test_case(1.0, "      +      ")] // Food is inside the FOV
        #[test_case(0.9, "      +      ")] // ditto
        #[test_case(0.8, "      +      ")] // ditto
        #[test_case(0.7, "      .      ")] // Food slowly disappears
        #[test_case(0.6, "      .      ")] // ditto
        #[test_case(0.5, "             ")] // Food disappeared!
        #[test_case(0.4, "             ")]
        #[test_case(0.3, "             ")]
        #[test_case(0.2, "             ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.5)],
                fov_angle: FRAC_PI_2,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                expected_vision,
                fov_range,
            }.run();
        }
    }

    mod different_rotations {
        use test_case::test_case;

        use super::*;

        /// World:
        ///
        /// ------------
        /// |          |
        /// |          |
        /// |    @>    |
        /// |          |
        /// |         %|
        /// ------------
        ///
        /// Test cases:
        ///
        /// ------------
        /// |..........|
        /// |..........|
        /// |....@>....|
        /// |..........|
        /// |.........%|
        /// ------------
        ///
        /// ------------
        /// |..........|
        /// |..........|
        /// |....@.....|
        /// |....v.....|
        /// |.........%|
        /// ------------
        ///
        /// ------------
        /// |..........|
        /// |..........|
        /// |...<@.....|
        /// |..........|
        /// |.........%|
        /// ------------
        ///
        /// ... and so on, until we do a full circle, 360° rotation:
        #[test_case(0.00 * PI, "         +   ")] // Food is to our right
        #[test_case(0.25 * PI, "        +    ")]
        #[test_case(0.50 * PI, "      +      ")]
        #[test_case(0.75 * PI, "    +        ")]
        #[test_case(1.00 * PI, "   +         ")] // Food is behind us
        #[test_case(1.25 * PI, " +           ")] // (we continue to see it
        #[test_case(1.50 * PI, "            +")] // due to 360° fov_angle.)
        #[test_case(1.75 * PI, "           + ")]
        #[test_case(2.00 * PI, "         +   ")] // Here we've done 360°
        #[test_case(2.25 * PI, "        +    ")] // (and a bit more, to
        #[test_case(2.50 * PI, "      +      ")] // prove the numbers wrap.)
        fn test(rot: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(0.5, 1.0)],
                fov_range: 1.0,
                fov_angle: 2.0 * PI,
                x: 0.5,
                y: 0.5,
                rot,
                expected_vision,
            }.run()
        }
    }

    mod different_positions {
        use test_case::test_case;

        use super::*;

        /// World:
        ///
        /// ------------
        /// |          |
        /// |         %|
        /// |          |
        /// |         %|
        /// |          |
        /// ------------
        ///
        /// Test cases for the X axis:
        ///
        /// ------------
        /// |          |
        /// |        /%|
        /// |       @>.|
        /// |        \%|
        /// |          |
        /// ------------
        ///
        /// ------------
        /// |        /.|
        /// |      /..%|
        /// |     @>...|
        /// |      \..%|
        /// |        \.|
        /// ------------
        ///
        /// ... and so on, going further left
        ///     (or, from the bird's point of view - going _back_)
        ///
        /// Test cases for the Y axis:
        ///
        /// ------------
        /// |     @>...|
        /// |       \.%|
        /// |         \|
        /// |         %|
        /// |          |
        /// ------------
        ///
        /// ------------
        /// |      /...|
        /// |     @>..%|
        /// |      \...|
        /// |        \%|
        /// |          |
        /// ------------
        ///
        /// ... and so on, going further down
        ///     (or, from the bird's point of view - going _right_)

        // Checking the X axis:
        // (you can see the bird is "flying away" from the foods)
        #[test_case(0.9, 0.5, "#           #")]
        #[test_case(0.8, 0.5, "  #       #  ")]
        #[test_case(0.7, 0.5, "   +     +   ")]
        #[test_case(0.6, 0.5, "    +   +    ")]
        #[test_case(0.5, 0.5, "    +   +    ")]
        #[test_case(0.4, 0.5, "     + +     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        //
        // Checking the Y axis:
        // (you can see the bird is "flying alongside" the foods)
        #[test_case(0.5, 0.0, "            +")]
        #[test_case(0.5, 0.1, "          + .")]
        #[test_case(0.5, 0.2, "         +  +")]
        #[test_case(0.5, 0.3, "        + +  ")]
        #[test_case(0.5, 0.4, "      +  +   ")]
        #[test_case(0.5, 0.6, "   +  +      ")]
        #[test_case(0.5, 0.7, "  + +        ")]
        #[test_case(0.5, 0.8, "+  +         ")]
        #[test_case(0.5, 0.9, ". +          ")]
        #[test_case(0.5, 1.0, "+            ")]
        fn test(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![food(1.0, 0.4), food(1.0, 0.6)],
                fov_range: 1.0,
                fov_angle: FRAC_PI_2,
                rot: 0.0,
                x,
                y,
                expected_vision,
            }.run()
        }
    }

    mod different_fov_angles {
        use test_case::test_case;

        use super::*;

        /// World:
        ///
        /// ------------
        /// |%  %  %  %|
        /// |          |
        /// |    @>    |
        /// |          |
        /// |%  %  %  %|
        /// ------------
        ///
        /// Test cases:
        ///
        /// ------------
        /// |%  %  %/.%|
        /// |      /...|
        /// |    @>....|
        /// |      \...|
        /// |%  %  %\.%|
        /// ------------
        ///
        /// ------------
        /// |%  %|.%..%|
        /// |    |.....|
        /// |    @>....|
        /// |    |.....|
        /// |%  %|.%..%|
        /// ------------
        ///
        /// ... and so on, until we reach the full, 360° FOV
        #[test_case(0.25 * PI, " +         + ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI, ".  +     +  .")]
        #[test_case(0.75 * PI, "  . +   + .  ")] // FOV gets progressively
        #[test_case(1.00 * PI, "   . + + .   ")] // wider and wider...
        #[test_case(1.25 * PI, "   . + + .   ")]
        #[test_case(1.50 * PI, ".   .+ +.   .")]
        #[test_case(1.75 * PI, ".   .+ +.   .")]
        #[test_case(2.00 * PI, "+.  .+ +.  .+")] // FOV is wide = 8 foods
        fn test(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                foods: vec![
                    food(0.0, 0.0),
                    food(0.0, 0.33),
                    food(0.0, 0.66),
                    food(0.0, 1.0),
                    food(1.0, 0.0),
                    food(1.0, 0.33),
                    food(1.0, 0.66),
                    food(1.0, 1.0),
                ],
                fov_range: 1.0,
                x: 0.5,
                y: 0.5,
                rot: 0.0,
                fov_angle,
                expected_vision,
            }.run()
        }
    }
}