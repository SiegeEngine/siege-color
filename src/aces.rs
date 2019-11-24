
use siege_math::{Vec2, Vec3, Mat3};
use crate::cie1931::{Cie1931, D60};
use super::Chromaticity;

pub const ACES_AP0_CHROMATICITIES: Chromaticity = Chromaticity {
    red:   Vec2 { x: 0.73470, y: 0.26530 },
    green: Vec2 { x: 0.00000, y: 1.00000 },
    blue:  Vec2 { x: 0.00010, y: -0.07700 },
    white: Vec2 { x: 0.32168, y: 0.33767 }
};

pub const ACES_AP1_CHROMATICITIES: Chromaticity = Chromaticity {
    red:   Vec2 { x: 0.713,   y: 0.293 },
    green: Vec2 { x: 0.165,   y: 0.830 },
    blue:  Vec2 { x: 0.128,   y: 0.044 },
    white: Vec2 { x: 0.32168, y: 0.33767 }
};


// ACES also deals with these:
// AP1 color primaries (contained within gamut, but is a wide gamut, bent towards
//                      display-referred spaces)
//    Red is (0.713, 0.293)
//    Green is (0.165, 0.830)
//    Blue is (0.128, 0.044)


/// Academy Color Encoding System, ACES-2065-1 (smpte)
///   AP0 color primaries (covers entire CIE 1964 standard-observer spectral locus)
///      Red is (0.7347, 0.2653)
///      Green is (0.0000, 1.0000)
///      Blue is (0.0001, -0.0770)
///   White point is CIE Standard D60 (0.32168, 0.33767)
/// Photometrically linear transfer characteristics
/// Perfectly white diffuser is (1,1,1) and 18% grey is (0.18, 0.18, 0.18)
/// Values can go outside of the [0,1] range. Values are scene referred.
#[derive(Debug, Clone)]
pub struct Aces {
    pub v: Vec3<f32>
}

impl Aces {
    pub fn new(r: f32, g: f32, b: f32) -> Aces {
        Aces {
            v: Vec3::new(r, g, b)
        }
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.v[0]
    }
    #[inline]
    pub fn g(&self) -> f32 {
        self.v[1]
    }
    #[inline]
    pub fn b(&self) -> f32 {
        self.v[2]
    }
}

impl From<Cie1931<D60>> for Aces {
    fn from(input: Cie1931<D60>) -> Aces {
        let m: Mat3<f32> = Mat3::new(
            1.0498110175, 0.00000000000, -0.0000974845,
            -0.4959030231, 1.3733130458, 0.0982400361,
            0.0000000000, 0.00000000000, 0.9912520182
        );
        Aces {
            v: &m * &input.v
        }
    }
}

impl From<Aces> for Cie1931<D60> {
    fn from(input: Aces) -> Cie1931<D60> {
        let m: Mat3<f32> = Mat3::new(
            0.9525523959, 0.0000000000, 0.0000936786,
            0.3439664498, 0.7281660966, -0.0721325464,
            0.0000000000, 0.0000000000, 1.0088251844
        );
        let v = &m * &input.v;
        Cie1931::<D60>::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aces_to_from() {
        let a = Aces::new(0.123, 1.0, 0.234);
        let b: Cie1931<D60> = From::from(a.clone());
        let c: Aces = From::from(b);

        assert!(a.v[0] - c.v[0] < 0.000001);
        assert!(c.v[0] - a.v[0] < 0.000001);
        assert!(a.v[1] - c.v[1] < 0.000001);
        assert!(c.v[1] - a.v[1] < 0.000001);
        assert!(a.v[2] - c.v[2] < 0.000001);
        assert!(c.v[2] - a.v[2] < 0.000001);
    }
}
