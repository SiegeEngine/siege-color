
pub struct StarMagnitude(pub f64);

impl StarMagnitude {
    pub fn to_brightness(&self) -> f64 {
        (10.0_f64).powf(-self.0 * 0.4)
    }

    pub fn from_brightness(brightness: f64) -> StarMagnitude {
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
