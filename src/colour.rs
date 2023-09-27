use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl Mul<f64> for Colour {
    type Output = Colour;

    fn mul(self, scalar: f64) -> Colour {
        Colour {
            r: self.r * scalar,
            g: self.g * scalar,
            b: self.b * scalar,
        }
    }
}

impl Mul<Colour> for f64 {
    type Output = Colour;

    fn mul(self, colour: Colour) -> Colour {
        Colour {
            r: self * colour.r,
            g: self * colour.g,
            b: self * colour.b,
        }
    }
}

// Implement the Add trait for Vector
impl Add<Colour> for Colour {
    type Output = Colour;

    fn add(self, other: Colour) -> Colour {
        Colour {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Div<f64> for Colour {
    type Output = Colour;

    fn div(self, scalar: f64) -> Colour {
        Colour {
            r: self.r / scalar,
            g: self.g / scalar,
            b: self.b / scalar,
        }
    }
}
