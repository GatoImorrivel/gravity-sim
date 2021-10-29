use std::ops::{Sub, AddAssign, Add, Mul, Neg};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2F {
    pub x: f32,
    pub y: f32,
}

impl Vector2F {
    pub fn magnituded(&self) -> f32 {
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2))
    }
    
    pub fn normalized(&self) -> Result<Vector2F, ()> {
        let mag = self.magnituded();
        if mag != 0.0 {
            Ok(
                Vector2F {
                    x: self.x / mag,
                    y: self.y / mag,
                }
            )
        } else {
            Err(())
        }
    }
    
    pub fn multiplied_by(&self, multiplier: f32) -> Vector2F {
        Vector2F {
            x: self.x * multiplier,
            y: self.y * multiplier,
        }
    }

    pub fn added_by(&self, adder: f32) -> Vector2F {
        Vector2F {
            x: self.x * adder,
            y: self.y * adder,
        }
    }

    pub fn powered_by(&self, power: f32) -> Vector2F {
        Vector2F {
            x: f32::powf(self.x, power),
            y: f32::powf(self.y, power),
        }
    }

    pub fn divided_by(&self, divisor: f32) -> Vector2F {
        Vector2F {
            x: self.x / divisor,
            y: self.y / divisor,
        }
    }
}

impl Sub for Vector2F {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Vector2F {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Add for Vector2F {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Vector2F {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Neg for Vector2F {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}