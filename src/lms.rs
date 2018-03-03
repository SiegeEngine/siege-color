
use siege_math::{Vec3, Mat3};
use cie1931::{Cie1931, D65};

/// This is the colorspace to use when converting white points
#[derive(Debug, Clone)]
pub struct Lms {
    pub v: Vec3<f32>
}

impl Lms {
    pub fn new(x: f32, y: f32, z: f32) -> Lms {
        Lms {
            v: Vec3::new(x, y, z)
        }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.v.x
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.v.y
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.v.z
    }
}

impl From<Cie1931<D65>> for Lms {
    fn from(input: Cie1931<D65>) -> Lms {
        // CIECAM02
        let m: Mat3<f32> = Mat3::new(
            0.7328, 0.4296, -0.1624,
            -0.7036, 1.6975, 0.0061,
            0.0030, 0.0136, 0.9834
        );
        Lms {
            v: &m * &input.v
        }
    }
}

impl From<Lms> for Cie1931<D65> {
    fn from(input: Lms) -> Cie1931<D65> {
        // Inverse CIECAM02
        let m: Mat3<f32> = Mat3::new(
            1.0961238, -0.278869, 0.18274519,
            0.45436904,  0.47353318, 0.07209781,
            -0.0096276095, -0.0056980313, 1.0153257
        );
        let v = &m * &input.v;
        Cie1931::<D65>::new(v.x, v.y, v.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lms_to_from() {
        let a = Lms::new(0.123, 1.0, 0.234);
        let b: Cie1931<D65> = From::from(a.clone());
        let c: Lms = From::from(b);

        assert!(a.v.x - c.v.x < 0.000001);
        assert!(c.v.x - a.v.x < 0.000001);
        assert!(a.v.y - c.v.y < 0.000001);
        assert!(c.v.y - a.v.y < 0.000001);
        assert!(a.v.z - c.v.z < 0.000001);
        assert!(c.v.z - a.v.z < 0.000001);
    }
}
