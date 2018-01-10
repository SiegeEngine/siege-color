
use cie1931::Cie1931;

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
            (srgb.r * 255.0).round() as u8,
            (srgb.g * 255.0).round() as u8,
            (srgb.b * 255.0).round() as u8
        )
    }
}

impl From<Srgb24> for Srgb {
    fn from(srgb24: Srgb24) -> Srgb {
        Srgb {
            r: srgb24.0 as f32 / 255.0,
            g: srgb24.1 as f32 / 255.0,
            b: srgb24.2 as f32 / 255.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Srgb {
    pub fn new(r: f32, g: f32, b: f32) -> Srgb {
        Srgb {
            r: r,
            g: g,
            b: b
        }
    }
}

#[derive(Debug, Clone)]
pub struct LinearSrgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl LinearSrgb {
    pub fn new(r: f32, g: f32, b: f32) -> LinearSrgb {
        LinearSrgb {
            r: r,
            g: g,
            b: b
        }
    }

    pub fn get_brightness(&self) -> f32
    {
        0.299 * self.r  +  0.587 * self.g  +  0.114 * self.b
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        let original = self.get_brightness();
        let scale = brightness / original;
        self.r *= scale;
        self.g *= scale;
        self.b *= scale;
        // WARNING: can go beyond 1.0
    }

    pub fn set_max_brightness(&mut self) {
        let scale = self.r.max(self.g).max(self.b);
        self.r /= scale;
        self.g /= scale;
        self.b /= scale;
    }
}

// Perhaps use the method here instead:
impl From<Cie1931> for LinearSrgb {
    fn from(input: Cie1931) -> LinearSrgb {
        LinearSrgb {
            // From https://en.wikipedia.org/wiki/SRGB and
            // https://www.image-engineering.de/library/technotes/958-how-to-convert-between-srgb-and-ciexyz
            r:  3.2404542 * input.x - 1.5371385 * input.y - 0.4985314 * input.z,
            g: -0.9692660 * input.x + 1.8760108 * input.y + 0.0415560 * input.z,
            b:  0.0556434 * input.x - 0.2040259 * input.y + 1.0572252 * input.z,
        }
    }
}

impl From<LinearSrgb> for Cie1931 {
    fn from(input: LinearSrgb) -> Cie1931 {
        Cie1931 {
            // From https://en.wikipedia.org/wiki/SRGB and
            // https://www.image-engineering.de/library/technotes/958-how-to-convert-between-srgb-and-ciexyz
            // Reference point of D65 (as defined by sRGB) -- be warned, ICC profiles use D50.
            x: 0.4124564 * input.r + 0.3575761 * input.g + 0.1804375 * input.b,
            y: 0.2126729 * input.r + 0.7151522 * input.g + 0.0721750 * input.b,
            z: 0.0193339 * input.r + 0.1191920 * input.g + 0.9503041 * input.b,
        }
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

        let mut red = f(srgb.r);
        if red>1.0 { red = 1.0 };

        let mut green = f(srgb.g);
        if green>1.0 { green = 1.0 };

        let mut blue = f(srgb.b);
        if blue>1.0 { blue = 1.0 };

        LinearSrgb {
            r: red,
            g: green,
            b: blue
        }
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

        let mut red = f(s.r);
        if red>1.0 { red = 1.0 };

        let mut green = f(s.g);
        if green>1.0 { green = 1.0 };

        let mut blue = f(s.b);
        if blue>1.0 { blue = 1.0 };

        Srgb {
            r: red,
            g: green,
            b: blue
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cie1931::Cie1931;

    #[test]
    fn test_scale_brightness() {

        let mut x = LinearSrgb {
            r: 0.5,
            g: 0.1,
            b: 0.4,
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
        let lsrgb = LinearSrgb {
            r: 0.5,
            g: 0.2,
            b: 0.7
        };
        let xyz: Cie1931 = From::from(lsrgb.clone());
        let lsrgb2: LinearSrgb = From::from(xyz);
        assert!(lsrgb.r - lsrgb2.r < 0.00005);
        assert!(lsrgb2.r - lsrgb.r < 0.00005);
        assert!(lsrgb.g - lsrgb2.g < 0.00005);
        assert!(lsrgb2.g - lsrgb.g < 0.00005);
        assert!(lsrgb.b - lsrgb2.b < 0.00005);
        assert!(lsrgb2.b - lsrgb.b < 0.00005);
    }

    #[test]
    fn test_xyz_to_srgb() {
        // sample from https://au.mathworks.com/help/images/ref/xyz2rgb.html?s_tid=gn_loc_drop
        let xyz = Cie1931::new(0.25, 0.40, 0.10);
        let lsrgb: LinearSrgb = From::from(xyz);
        let srgb: Srgb = From::from(lsrgb);
        let srgbu: Srgb24 = From::from(srgb);
        assert_eq!(srgbu.0, 106);
        assert_eq!(srgbu.1, 190);
        assert_eq!(srgbu.2, 55);
    }

    #[test]
    fn test_to_and_from_linear() {
        let srgb = Srgb {
            r: 0.1245,
            g: 0.0924,
            b: 0.9812,
        };

        let l: LinearSrgb = From::from(srgb.clone());
        let srgb2: Srgb = From::from(l);

        assert!(srgb.r - srgb2.r < 0.000001);
        assert!(srgb2.r - srgb.r < 0.000001);
        assert!(srgb.g - srgb2.g < 0.000001);
        assert!(srgb2.g - srgb.g < 0.000001);
        assert!(srgb.b - srgb2.b < 0.000001);
        assert!(srgb2.b - srgb.b < 0.000001);
    }

    #[test]
    fn test_set_max_brightness() {
        let mut lsrgb = LinearSrgb::new(0.1, 0.25, 0.5);
        lsrgb.set_max_brightness();
        assert!(lsrgb.r - 0.2 < 0.0000001);
        assert!(0.2 - lsrgb.r < 0.0000001);
        assert!(lsrgb.g - 0.5 < 0.0000001);
        assert!(0.5 - lsrgb.g < 0.0000001);
        assert!(lsrgb.b - 1.0 < 0.0000001);
        assert!(1.0 - lsrgb.b < 0.0000001);

        let mut lsrgb = LinearSrgb::new(1.5, 0.8234, 0.24);
        lsrgb.set_max_brightness();
        assert!(lsrgb.r - 1.0 < 0.0000001);
        assert!(1.0 - lsrgb.r < 0.0000001);
    }
}
