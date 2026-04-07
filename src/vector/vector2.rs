use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;

#[derive(Copy, Clone, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Add<Output = T> + Copy> Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl<T: Sub<Output = T> + Copy> Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn sub(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn div(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Rem<Output = T> + Copy> Rem<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;
    fn rem(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}
