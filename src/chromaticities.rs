
use siege_math::Vec2;

/// Chromaticity points specified in CIE 1931 XYZ space
pub struct Chromaticity {
    pub red: Vec2<f32>,
    pub green: Vec2<f32>,
    pub blue: Vec2<f32>,
    pub white: Vec2<f32>,
}
