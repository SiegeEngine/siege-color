
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

pub struct SRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct LinearSRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl LinearSRGB {
    // This applies gamma correction
    // Assumes the 2-degree colorimetric observer
    pub fn to_srgb(&self) -> SRGB {
        let a = 0.055;

        let mut red = if self.r <= 0.0031308 {
            12.92 * self.r
        } else {
            (1.0 + a) * self.r.powf(1.0/2.4) - a
        };
        if red>1.0 { red = 1.0 };

        let mut green = if self.g < 0.0031308 {
            12.92 * self.g
        } else {
            (1.0 + a) * self.g.powf(1.0/2.4) - a
        };
        if green>1.0 { green = 1.0 };

        let mut blue = if self.b < 0.0031308 {
            12.92 * self.b
        } else {
            (1.0 + a) * self.b.powf(1.0/2.4) - a
        };
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
}
