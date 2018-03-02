
use siege_math::{Vec3, Mat3};
use cie1931::{Cie1931, D65};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Srgb24(pub u8, pub u8, pub u8);

impl Srgb24 {
    pub fn new(r: u8, g: u8, b: u8) -> Srgb24 {
        Srgb24(r,g,b)
    }
}

impl From<Srgb> for Srgb24 {
    fn from(srgb: Srgb) -> Srgb24 {
        Srgb24(
            (srgb.v.x * 255.0).round() as u8,
            (srgb.v.y * 255.0).round() as u8,
            (srgb.v.z * 255.0).round() as u8
        )
    }
}

impl From<Srgb24> for Srgb {
    fn from(srgb24: Srgb24) -> Srgb {
        Srgb::new(
            srgb24.0 as f32 / 255.0,
            srgb24.1 as f32 / 255.0,
            srgb24.2 as f32 / 255.0
        )
    }
}

#[derive(Debug, Clone)]
pub struct Srgb {
    pub v: Vec3<f32>
}

impl Srgb {
    pub fn new(r: f32, g: f32, b: f32) -> Srgb {
        Srgb {
            v: Vec3::new(r, g, b)
        }
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.v.x
    }
    #[inline]
    pub fn g(&self) -> f32 {
        self.v.y
    }
    #[inline]
    pub fn b(&self) -> f32 {
        self.v.z
    }
}

#[derive(Debug, Clone)]
pub struct LinearSrgb {
    pub v: Vec3<f32>
}

impl LinearSrgb {
    pub fn new(r: f32, g: f32, b: f32) -> LinearSrgb {
        LinearSrgb {
            v: Vec3::new(r, g, b)
        }
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.v.x
    }
    #[inline]
    pub fn g(&self) -> f32 {
        self.v.y
    }
    #[inline]
    pub fn b(&self) -> f32 {
        self.v.z
    }

    pub fn get_luminance(&self) -> f32
    {
        // middle row of From<LinearSrgb> for Cie1931
        Vec3::<f32>::new(0.2126729,
                         0.7151522,
                         0.0721750).dot(self.v)
    }

    // For luminance values > 1.0, this saturates to white (e.g. each
    // channel scales linearly and then clamps at 1.0)
    pub fn set_luminance(&mut self, luminance: f32) {
        // no negative values
        let newlum = if luminance < 0.0 { 0.0 } else { luminance };

        let oldlum: f32 = self.get_luminance();
        if oldlum==0.0 { return; } // black is gonna stay black, and we cant divide by zero
        let scale: f32 = newlum/oldlum;
        self.v.x = (self.v.x * scale).min(1.0);
        self.v.y = (self.v.y * scale).min(1.0);
        self.v.z = (self.v.z * scale).min(1.0);
    }

    pub fn get_brightness(&self) -> f32
    {
        // source???
        0.299 * self.r()  +  0.587 * self.g()  +  0.114 * self.b()
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        let original = self.get_brightness();
        let scale = brightness / original;
        self.v *= scale;
        // WARNING: can go beyond 1.0
    }

    pub fn set_max_brightness(&mut self) {
        let scale = self.r().max(self.g()).max(self.b());
        self.v /= scale;
    }
}

// Perhaps use the method here instead:
impl From<Cie1931<D65>> for LinearSrgb {
    fn from(input: Cie1931<D65>) -> LinearSrgb {
        // Concepts from (but not data):
        //   From https://en.wikipedia.org/wiki/SRGB and
        //   https://www.image-engineering.de/library/technotes/958-how-to-convert-between-srgb-and-ciexyz
        // Exact figures from http://www.color.org/chardata/rgb/srgb.xalter
        // Assuming: a 2° standard colorimetric observer for CIE XYZ
        //           D65 White Point (x = 0.3127, y = 0.3290, z = 0.3583)
        //           White Point Luminance: 80 cd/m^2

        /*
        // Tristimulus value normalization (sRGB spec section 6)
        // PRESUMES Y=100 !!!!!
        let n: Vec3<f32> = Vec3::new(
            input.v.x - 0.1901,
            input.v.y - 0.2,
            input.v.z - 0.2178) * 0.0125313;
         */

        let m: Mat3<f32> = Mat3::new(
            3.2406255, -1.5372080, -0.49862860,
            -0.96893071, 1.8757561, 0.041517524,
            0.055710120, -0.20402105, 1.0569959
        );

        LinearSrgb {
            v: &m * &input.v
        }
    }
}

impl From<LinearSrgb> for Cie1931<D65> {
    fn from(input: LinearSrgb) -> Cie1931<D65> {
        // From https://en.wikipedia.org/wiki/SRGB and
        // https://www.image-engineering.de/library/technotes/958-how-to-convert-between-srgb-and-ciexyz
        // Reference point of D65 (as defined by sRGB) -- be warned, ICC profiles use D50.
        // (These values are exact. The above matrix is an inverse of this.)
        let m: Mat3<f32> = Mat3::new(
            0.4124, 0.3576, 0.1805,
            0.2126, 0.7152, 0.0722,
            0.0193, 0.1192, 0.9505
        );

        let cv = &m * &input.v;

        Cie1931::new(cv.x, cv.y, cv.z)
    }

}

// Perhaps use the method here instead:
//   https://en.wikipedia.org/w/index.php?title=CIE_1931_color_space&action=edit&section=13
impl From<Srgb> for LinearSrgb {
    fn from(srgb: Srgb) -> LinearSrgb {
        let f = |x: f32| -> f32 {
            if x <= 0.04045 { x / 12.92 }
            else { ((x + 0.055)/1.055).powf(2.4) }
        };

        let mut red = f(srgb.r());
        if red>1.0 { red = 1.0 };

        let mut green = f(srgb.g());
        if green>1.0 { green = 1.0 };

        let mut blue = f(srgb.b());
        if blue>1.0 { blue = 1.0 };

        LinearSrgb::new(red, green, blue)
    }
}

impl From<LinearSrgb> for Srgb {
    // This applies gamma correction
    // Assumes the 2-degree colorimetric observer
    fn from(s: LinearSrgb) -> Srgb {
        let a = 0.055;
        let f = |x: f32| -> f32 {
            if x <= 0.0031308 { 12.92 * x }
            else { (1.0 + a) * x.powf(1.0/2.4) - a }
        };

        let mut red = f(s.r());
        if red>1.0 { red = 1.0 };

        let mut green = f(s.g());
        if green>1.0 { green = 1.0 };

        let mut blue = f(s.b());
        if blue>1.0 { blue = 1.0 };

        Srgb::new(red, green, blue)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cie1931::{Cie1931, D65};
    use float_cmp::ApproxEqUlps;

    #[test]
    fn test_scale_brightness() {

        let mut x = LinearSrgb {
            v: Vec3::new( 0.5, 0.1, 0.4 )
        };

        x.set_brightness(0.8);
        assert!(x.get_brightness() > 0.799999);
        assert!(x.get_brightness() < 0.800001);

        x.set_brightness(0.2);
        assert!(x.get_brightness() > 0.199999);
        assert!(x.get_brightness() < 0.200001);
    }

    #[test]
    fn test_to_and_from_24() {
        let srgb24 = Srgb24(1,64,255);
        let srgb: Srgb = From::from(srgb24.clone());
        let srgb24_2: Srgb24 = From::from(srgb);
        assert_eq!(srgb24, srgb24_2);
    }

    #[test]
    fn test_to_and_from_cie1931() {
        let lsrgb = LinearSrgb::new(0.5, 0.2, 0.7);
        let xyz: Cie1931<D65> = From::from(lsrgb.clone());
        let lsrgb2: LinearSrgb = From::from(xyz);

        assert!(lsrgb.v.approx_eq_ulps(&lsrgb2.v, 10));
    }

    #[test]
    fn test_xyz_to_srgb() {
        // sample from https://au.mathworks.com/help/images/ref/xyz2rgb.html?s_tid=gn_loc_drop
        let xyz = Cie1931::<D65>::new(0.25, 0.40, 0.10);
        let lsrgb: LinearSrgb = From::from(xyz);
        let srgb: Srgb = From::from(lsrgb);
        let srgbu: Srgb24 = From::from(srgb);
        assert_eq!(srgbu.0, 106);
        assert_eq!(srgbu.1, 190);
        assert_eq!(srgbu.2, 55);
    }

    #[test]
    fn test_to_and_from_linear() {
        let srgb = Srgb::new(0.1245, 0.0924, 0.9812);

        let l: LinearSrgb = From::from(srgb.clone());
        let srgb2: Srgb = From::from(l);

        assert!(srgb.v.approx_eq_ulps(&srgb2.v, 10));
    }

    #[test]
    fn test_set_max_brightness() {
        let mut lsrgb = LinearSrgb::new(0.1, 0.25, 0.5);
        lsrgb.set_max_brightness();

        assert!(lsrgb.v.approx_eq_ulps(&Vec3::<f32>::new(0.2, 0.5, 1.0), 10));

        let mut lsrgb = LinearSrgb::new(1.5, 0.8234, 0.24);
        lsrgb.set_max_brightness();
        assert!(lsrgb.r().approx_eq_ulps(&1.0, 10));
    }
}
