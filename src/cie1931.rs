
// CIE 1931 XYZ colorspace
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

    pub fn get_luminance(&self) -> f32 {
        self.y
    }

    pub fn set_luminance(&mut self, luminance: f32) {
        let scale = luminance / self.y;
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}

// CIE 1931 xyY colorspace
#[derive(Debug, Clone)]
pub struct Cie1931xy {
    pub x: f32,
    pub y: f32
}

impl Cie1931xy {
    pub fn new(x: f32, y: f32) -> Cie1931xy {
        Cie1931xy {
            x: x,
            y: y,
        }
    }

    pub fn z(&self) -> f32 {
        1.0 - self.x - self.y
    }
}

impl From<Cie1931> for Cie1931xy {
    fn from(input: Cie1931) -> Cie1931xy {
        Cie1931xy {
            x: input.x / (input.x + input.y + input.z),
            y: input.y / (input.x + input.y + input.z),
        }
    }
}

impl From<Cie1931xy> for Cie1931 {
    fn from(input: Cie1931xy) -> Cie1931 {
        Cie1931 {
            x: input.x / input.y,
            y: 1.0,
            z: (1.0 - input.x - input.y) / input.y,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cie1931_to_from() {
        let a = Cie1931::new(0.123, 1.0, 0.234);
        let b: Cie1931xy = From::from(a.clone());
        let c: Cie1931 = From::from(b);

        assert!(a.x - c.x < 0.000001);
        assert!(c.x - a.x < 0.000001);
        assert!(a.y - c.y < 0.000001);
        assert!(c.y - a.y < 0.000001);
        assert!(a.z - c.z < 0.000001);
        assert!(c.z - a.z < 0.000001);
    }
}
