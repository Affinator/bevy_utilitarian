use bevy::render::color::Color;
use std::{
    iter::Sum,
    ops::{Add, Deref, Mul, Sub},
};

#[derive(Default, std::fmt::Debug, Clone, PartialEq, Copy)]
pub struct ColorPoint {
    pub color: Color,
}

impl ColorPoint {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            color: Color::rgba(r, g, b, a),
        }
    }
}

impl Mul<f32> for ColorPoint {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            color: self.color * rhs,
        }
    }
}

impl Add<Self> for ColorPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            color: self.color + rhs.color,
        }
    }
}

impl Sub<Self> for ColorPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_color = Color::rgba(
            self.color.r() - rhs.color.r(),
            self.color.g() - rhs.color.g(),
            self.color.b() - rhs.color.b(),
            self.color.a() - rhs.color.a(),
        );

        Self { color: new_color }
    }
}

impl Add<f32> for ColorPoint {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        let new_color = Color::rgba(
            self.color.r() + rhs,
            self.color.g() + rhs,
            self.color.b() + rhs,
            self.color.a() + rhs,
        );
        Self { color: new_color }
    }
}

impl Sum for ColorPoint {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut c = Color::BLACK;

        for i in iter {
            c += i.color;
        }

        Self { color: c }
    }
}

impl Deref for ColorPoint {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.color
    }
}
