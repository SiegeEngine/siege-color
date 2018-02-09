
pub struct StarMagnitude(pub f32);

impl StarMagnitude {
    pub fn to_brightness(&self) -> f32 {
        (10.0_f32).powf(-self.0 * 0.4)
    }

    pub fn from_brightness(brightness: f32) -> StarMagnitude {
        StarMagnitude(-2.5 * brightness.log10())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude_brightness() {
        let m = StarMagnitude(4.234);
        let b = m.to_brightness();
        let m2 = StarMagnitude::from_brightness(b);
        assert!(m2.0 - 4.234 < 0.0000001);
    }
}
