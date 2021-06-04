use core::ops::{Add, Sub, Mul, Div};
use std::f64::consts::PI;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul for Vector {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl Div for Vector {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y
        }
    }
}

impl Vector {
    #[inline]
    pub fn new() -> Vector {
        Vector::default()
    }

    #[inline]
    pub fn from_polar(r: f64, theta: f64) -> Vector {
        Vector {
            x: r * theta.cos(),
            y: r * theta.sin()
        }
    }

    #[inline]
    /// Returns a normal vector from given angle
    pub fn from_angle(theta: f64) -> Vector {
        Vector {
            x: theta.cos(),
            y: theta.sin()
        }
    }

    #[inline]
    pub fn angle(&self) -> f64 {
        f64::atan2(self.y, self.x)
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.x.hypot(self.y)
    }

    #[inline]
    pub fn normalized(&self) -> Vector {
        self.div_by_float(self.length())
    }

    #[inline]
    pub fn rotated(&self, theta: f64) -> Vector {
        Vector {
            x: self.x * theta.cos() - self.y * theta.sin(),
            y: self.x * theta.sin() + self.y * theta.cos()
        }
    }

    #[inline]
    pub fn add_float(self, other: f64) -> Vector {
        Vector {
            x: self.x + other,
            y: self.y + other
        }
    }

    #[inline]
    pub fn mul_by_float(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other
        }
    }

    #[inline]
    pub fn div_by_float(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other
        }
    }
}

#[inline]
pub fn angle_diff(this: f64, other: f64) -> f64 {
    let mut angle = this - other;
    if angle > PI {
        angle = -2. * PI - angle;
    } else if angle < -PI {
        angle += 2. * PI;
    }
    angle
}
