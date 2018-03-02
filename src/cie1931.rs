
use std::marker::PhantomData;
use siege_math::{Vec3, Vec2, Mat3};

#[derive(Debug, Clone, Copy)]
pub struct D50;
#[derive(Debug, Clone, Copy)]
pub struct D65;
pub trait Illuminant { }
impl Illuminant for D50 { }
impl Illuminant for D65 { }


/// CIE 1931 XYZ colorspace at the D65 whitepoint
/// Normalized to Y=1.0 (Not Y=100 !!!)
#[derive(Debug, Clone)]
pub struct Cie1931<I: Illuminant> {
    pub v: Vec3<f32>,
    _phantom: PhantomData<I>
}

impl<I: Illuminant> Cie1931<I> {
    pub fn new(x: f32, y: f32, z: f32) -> Cie1931<I> {
        Cie1931 {
            v: Vec3::new(x, y, z),
            _phantom: Default::default()
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

    pub fn get_luminance(&self) -> f32 {
        self.v.y
    }

    pub fn set_luminance(&mut self, luminance: f32) {
        let scale = luminance / self.v.y;
        self.v *= scale;
    }
}

impl From<Cie1931<D65>> for Cie1931<D50> {
    fn from(input: Cie1931<D65>) -> Cie1931<D50>
    {
        let m: Mat3<f32> = Mat3::new(
            1.047844353856414, 0.022898981050086, -0.050206647741605,
            0.029549007606644, 0.990508028941971, -0.017074711360960,
            -0.009250984365223, 0.015072338237051, 0.751717835079977
        );

        Cie1931 {
            v: &m * &input.v,
            _phantom: Default::default()
        }
    }
}

impl From<Cie1931<D50>> for Cie1931<D65> {
    fn from(input: Cie1931<D50>) -> Cie1931<D65>
    {
        let m: Mat3<f32> = Mat3::new(
            0.9555491471339036, -0.02305395902610921, 0.0632967285241842,
            -0.028293615880275732, 1.009916725621172, 0.021049798533869513,
            0.012326727844429515, -0.020533074478247797, 1.3306432822046876
        );

        Cie1931 {
            v: &m * &input.v,
            _phantom: Default::default()
        }
    }
}


// CIE 1931xy colorspace
// FIXME - is this type parameterized by an illuminant?
#[derive(Debug, Clone)]
pub struct Cie1931xy {
    pub v: Vec2<f32>
}

impl Cie1931xy {
    pub fn new(x: f32, y: f32) -> Cie1931xy {
        Cie1931xy {
            v: Vec2::new(x, y)
        }
    }

    pub fn x(&self) -> f32 {
        self.v.x
    }
    pub fn y(&self) -> f32 {
        self.v.y
    }
    pub fn z(&self) -> f32 {
        1.0 - self.v.x - self.v.y
    }
}

impl From<Cie1931<D65>> for Cie1931xy {
    fn from(input: Cie1931<D65>) -> Cie1931xy {
        Cie1931xy::new(
            input.v.x / (input.v.x + input.v.y + input.v.z),
            input.v.y / (input.v.x + input.v.y + input.v.z)
        )
    }
}

impl From<Cie1931xy> for Cie1931<D65> {
    fn from(input: Cie1931xy) -> Cie1931<D65> {
        Cie1931::new(
            input.v.x / input.v.y,
            1.0,
            (1.0 - input.v.x - input.v.y) / input.v.y
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cie1931_to_from() {
        let a = Cie1931::<D65>::new(0.123, 1.0, 0.234);
        let b: Cie1931xy = From::from(a.clone());
        let c: Cie1931<D65> = From::from(b);

        assert!(a.v.x - c.v.x < 0.000001);
        assert!(c.v.x - a.v.x < 0.000001);
        assert!(a.v.y - c.v.y < 0.000001);
        assert!(c.v.y - a.v.y < 0.000001);
        assert!(a.v.z - c.v.z < 0.000001);
        assert!(c.v.z - a.v.z < 0.000001);
    }

    #[test]
    fn invert_test() {
        let c1 = Cie1931::<D50>::new(0.998123, 0.24987234, 0.45287234);
        let c2: Cie1931<D65> = From::from(c1.clone());
        let c3: Cie1931<D50> = From::from(c2);

        use float_cmp::ApproxEq;
        assert!( c1.v.approx_eq(&c3.v, 10, 10.0 * ::std::f32::EPSILON) );
    }
}
