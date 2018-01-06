
#[derive(Debug, Clone)]
pub struct SRGBu(u8, u8, u8);

impl From<SRGB> for SRGBu {
    fn from(srgb: SRGB) -> SRGBu {
        SRGBu(
            (srgb.r * 256.0).floor() as u8,
            (srgb.g * 256.0).floor() as u8,
            (srgb.b * 256.0).floor() as u8
        )
    }
}

#[derive(Debug, Clone)]
pub struct SRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Debug, Clone)]
pub struct LinearSRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl LinearSRGB {
    pub fn from_srgb(srgb: SRGB) -> LinearSRGB {
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

        LinearSRGB {
            r: red,
            g: green,
            b: blue
        }
    }

    // This applies gamma correction
    // Assumes the 2-degree colorimetric observer
    pub fn to_srgb(&self) -> SRGB {

        let a = 0.055;
        let f = |x: f32| -> f32 {
            if x <= 0.0031308 { 12.92 * x }
            else { (1.0 + a) * x.powf(1.0/2.4) - a }
        };

        let mut red = f(self.r);
        if red>1.0 { red = 1.0 };

        let mut green = f(self.g);
        if green>1.0 { green = 1.0 };

        let mut blue = f(self.b);
        if blue>1.0 { blue = 1.0 };

        SRGB {
            r: red,
            g: green,
            b: blue
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
}


#[cfg(test)]
mod tests {
    use super::*;
    use cie1931::Cie1931;

    #[test]
    fn test_scale_brightness() {

        let mut x = LinearSRGB {
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
    fn test_xyz_to_srgb() {
        let xyz = Cie1931::new(0.25, 0.40, 0.10);
        let lsrgb = xyz.to_linear_srgb();
        let srgb = lsrgb.to_srgb();
        let srgbu: SRGBu = From::from(srgb);
        assert_eq!(srgbu.0, 106);
        assert_eq!(srgbu.1, 190);
        assert_eq!(srgbu.2, 55);
    }

    #[test]
    fn test_to_and_from_linear() {
        let srgb = SRGB {
            r: 0.1245,
            g: 0.0924,
            b: 0.9812,
        };

        let l = LinearSRGB::from_srgb(srgb.clone());
        let srgb2 = l.to_srgb();

        assert!(srgb.r - srgb2.r < 0.000001);
        assert!(srgb2.r - srgb.r < 0.000001);
        assert!(srgb.g - srgb2.g < 0.000001);
        assert!(srgb2.g - srgb.g < 0.000001);
        assert!(srgb.b - srgb2.b < 0.000001);
        assert!(srgb2.b - srgb.b < 0.000001);
    }
}
