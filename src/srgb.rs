
pub struct LinearSRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub struct SRGB {
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
}
