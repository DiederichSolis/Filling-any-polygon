#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Color { r, g, b, a: 1.0 }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r * 255.0) as u32) << 16 | ((self.g * 255.0) as u32) << 8 | (self.b * 255.0) as u32
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,        
            b: self.b * other,
            a: self.a * other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    fn color_approx_eq(c1: Color, c2: Color) -> bool {
        approx_eq(c1.r, c2.r) && approx_eq(c1.g, c2.g) && approx_eq(c1.b, c2.b) && approx_eq(c1.a, c2.a)
    }

    #[test]
    fn test_multiply_by_negative() {
        let color = Color::new(0.5, 0.5, 0.5, 0.5);
        let result = color * -1.0;
        assert!(color_approx_eq(result, Color::new(-0.5, -0.5, -0.5, -0.5)));
    }

    #[test]
    fn test_multiply_by_larger_number() {
        let color = Color::new(0.5, 0.5, 0.5, 0.5);
        let result = color * 2.0;
        assert!(color_approx_eq(result, Color::new(1.0, 1.0, 1.0, 1.0)));
    }
}
