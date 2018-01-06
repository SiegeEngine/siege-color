
use cie1931::Cie1931xy;

// Color Temperature in Kelvin
pub struct ColorTemp(pub u16);

impl ColorTemp {
    pub fn new(k: u16) -> ColorTemp {
        ColorTemp(k)
    }
}

impl ColorTemp {
    pub fn to_cie1931xy(&self) -> Option<Cie1931xy>
    {
        // We use the Planckian locus to compute this, using the approximation
        // given in https://en.wikipedia.org/wiki/Planckian_locus

        // Only works for the given range:
        if self.0 < 1667 { return None; }
        if self.0 > 25000 { return None; }

        let ct = self.0 as f32;

        let x = if ct < 4000.0 {
            -0.2661239 * (10.0_f32).powi(9) / ct.powi(3)
                - 0.2343580 * (10.0_f32).powi(6) / ct.powi(2)
                + 0.8776956 * (10.0_f32).powi(3) / ct
                + 0.179910
        } else {
            -3.0258469 * (10.0_f32).powi(9) / ct.powi(3)
                + 2.1070379 * (10.0_f32).powi(6) / ct.powi(2)
                + 0.2226347 * (10.0_f32).powi(3) / ct
                + 0.240390
        };
        let y = if ct < 2222.0 {
            -1.1063814 * x.powi(3)
                - 1.34811020 * x.powi(2)
                + 2.18444832 * x
                - 0.20219683
        } else if ct < 4000.0 {
            -0.9549476 * x.powi(3)
                - 1.37418593 * x.powi(2)
                + 2.09137015 * x
                - 0.16748867
        } else {
            3.0817580 * x.powi(3)
                - 5.87338670 * x.powi(2)
                + 3.75112997 * x
                - 0.37001483
        };

        Some(Cie1931xy {
            x: x,
            y: y
        })
    }
}
