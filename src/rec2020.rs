
use siege_math::Vec2;
use super::Chromaticity;

pub const REC2020_CHROMATICITIES: Chromaticity = Chromaticity {
    red:   Vec2 { x: 0.708, y: 0.292 },
    green: Vec2 { x: 0.170, y: 0.797 },
    blue:  Vec2 { x: 0.131, y: 0.046 },
    white: Vec2 { x: 0.3217, y: 0.3290 }
};
