
pub mod colortemp;
pub mod cie1931;
pub mod srgb;
pub mod star_magnitude;

pub use colortemp::*;
pub use cie1931::*;
pub use srgb::*;
pub use star_magnitude::*;

// Converts watts-or-lumens/sq-meter to an RGB value from 0.0 to 1.0
// using the given white_point, and presuming a 100,000:1 contrast ratio
pub fn color_level(irradiance: f32, white_point: f32) -> f32
{
    // We simulate the human eye with 100,000:1 contrast ratio,
    // even though it only has between 1000:1 or 16000:1 contrast ratio.
    // We do this because the eye has a rapid dynamic response closer to
    // 100,000:1 (some say 1,000,000:1, but we don't go so far).

    let black_point = white_point / 100_000_f32;

    // irradiances at the white point or higher should yield 1.0
    if irradiance >= white_point {
        1.0
    }
    else if irradiance <= black_point {
        0.0
    }
    else {
        // presuming a linear relationship
        (irradiance - black_point) / (white_point - black_point)
    }
}
