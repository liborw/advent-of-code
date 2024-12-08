pub mod vec2;
use num::Num;
pub use vec2::Vec2;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<T> {
    pub min: Vec2<T>,
    pub max: Vec2<T>
}

impl<T: Num + Copy> Rect<T> {
    pub fn zero() -> Self {
        Self{min: Vec2::zero(), max: Vec2::zero()}
    }

    pub fn offset(&self, o: T) -> Self {
        Self{min: self.min - o, max: self.max + o}
    }
}
