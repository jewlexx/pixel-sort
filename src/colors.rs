use image::Rgba;

pub trait SingleColor {
    fn get_rgba(&self) -> Rgba<u8>;

    /// Calculate luminance using standard formula
    ///
    /// Formula: (0.2126*R + 0.7152*G + 0.0722*B)
    fn get_luminance(&self) -> u32 {
        let [r, g, b, _] = self.get_rgba().0;

        (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u32
    }

    fn get_absolute(&self) -> u32 {
        let [r, g, b, _] = self.get_rgba().0;

        r as u32 * g as u32 * b as u32
    }

    fn get_hsla(&self) -> (u32, f32, f32, u8) {
        let [r, g, b, alpha] = self.get_rgba().0;

        let normalised = [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
        let [dr, dg, db] = normalised;

        let max = normalised
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let min = normalised
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let c = max - min;

        let hue = {
            let hue = if c == 0.0 {
                0.0
            } else {
                match c {
                    x if x > 0.5 => (max - dr) / c,
                    x if x > 0.25 => (max - dg) / c + 2.0,
                    x if x > 0.75 => (max - db) / c + 4.0,
                    _ => (max - dr) / c + 6.0,
                }
            };

            hue * 60.0
        };

        let lightness = (max + min) / 2.0;

        let saturation = if c == 0.0 {
            0.0
        } else if lightness < 0.5 {
            c / (max + min)
        } else {
            c / (2.0 - max - min)
        };

        (hue as u32, saturation, lightness, alpha)
    }

    fn get_hue(&self) -> u32 {
        self.get_hsla().0
    }

    fn get_saturation(&self) -> f32 {
        self.get_hsla().1
    }
}

impl SingleColor for (u32, u32, Rgba<u8>) {
    fn get_rgba(&self) -> Rgba<u8> {
        self.2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl SingleColor for [u8; 4] {
        fn get_rgba(&self) -> Rgba<u8> {
            Rgba(*self)
        }
    }

    #[test]
    fn test_get_hsla() {
        const WHITE: [u8; 4] = [255; 4];
        const BLACK: [u8; 4] = [0, 0, 0, 255];
        const RED: [u8; 4] = [255, 0, 0, 255];
        const GREEN: [u8; 4] = [0, 255, 0, 255];
        const BLUE: [u8; 4] = [0, 0, 255, 255];
        const CYAN: [u8; 4] = [0, 255, 255, 255];

        assert_eq!(WHITE.get_hsla(), (0, 0.0, 1.0, 255));
        assert_eq!(BLACK.get_hsla(), (0, 0.0, 0.0, 255));
        assert_eq!(RED.get_hsla(), (0, 1.0, 0.5, 255));
        assert_eq!(GREEN.get_hsla(), (120, 1.0, 0.5, 255));
        assert_eq!(BLUE.get_hsla(), (240, 1.0, 0.5, 255));
        assert_eq!(CYAN.get_hsla(), (180, 1.0, 0.5, 255));
    }
}
