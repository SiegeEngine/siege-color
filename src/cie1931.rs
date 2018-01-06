
use srgb::LinearSRGB;

#[derive(Debug, Clone)]
pub struct Cie1931 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Cie1931 {
    pub fn new(x: f32, y: f32, z: f32) -> Cie1931 {
        Cie1931 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn to_linear_srgb(&self) -> LinearSRGB {
        LinearSRGB {
            r:  3.2406 * self.x - 1.5372 * self.y - 0.4986 * self.z,
            g: -0.9689 * self.x + 1.8758 * self.y + 0.0415 * self.z,
            b:  0.0557 * self.x - 0.2040 * self.y + 1.0570 * self.z,
        }
    }
}

// CIE 1931 xyY colorspace
#[derive(Debug, Clone)]
pub struct Cie1931xy {
    pub x: f32,
    pub y: f32
}

impl Cie1931xy {
    pub fn from_cie1931(input: Cie1931) -> Cie1931xy
    {
        Cie1931xy {
            x: input.x / (input.x + input.y + input.z),
            y: input.y / (input.x + input.y + input.z),
        }
    }

    pub fn to_cie1931(&self) -> Cie1931
    {
        Cie1931 {
            x: self.x / self.y,
            y: 1.0,
            z: (1.0 - self.x - self.y) / self.y,
        }
    }

    pub fn z(&self) -> f32 {
        1.0 - self.x - self.y
    }
}
