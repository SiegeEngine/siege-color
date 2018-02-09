
pub struct StarMagnitude(pub f32);

impl StarMagnitude {
    pub fn to_brightness(&self) -> f32 {
        (10.0_f32).powf(-self.0 * 0.4)
    }

    pub fn from_brightness(brightness: f32) -> StarMagnitude {
        StarMagnitude(-2.5 * brightness.log10())
    }

    // Irradiance in watts per square meter
    pub fn irradiance(&self) -> f32 {
        /*
        https://archive.is/20121204144725/http://www.astro.utoronto.ca/~patton/astro/mags.html
        https://www.cfa.harvard.edu/~dfabricant/huchra/ay145/mags.html

        This magnitude system is defined such that, when monochromatic flux f
        is measured in erg sec^-1 cm^-2 Hz^-1,

           m(AB) = -2.5 log(f) - 48.60

        1 erg = 10^-7 Joules

        green (550 nm) has frequency of c/550nm = 545,077,196,363,636.3 hz

        constant factor for unit conversions is then:
        100 (cm/m) * 100 (cm/m) * 10^-7 (J/erg) / 545,077,196,363,636.3 hz
         = 5.450771963636363e+16
         */
        (5.45077e+16) * (10.0_f32).powf((self.0 + 48.60)/(-2.5))
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
