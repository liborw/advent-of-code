pub mod vec2;
use num::Num;
pub use vec2::Vec2;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect<T> {
    pub min: Vec2<T>,
    pub max: Vec2<T>
}

impl<T: Num + Copy> Rect<T> {

    pub fn new(min: Vec2<T>, max: Vec2<T>) -> Self {
        Self{min, max}
    }

    pub fn zero() -> Self {
        Self{min: Vec2::zero(), max: Vec2::zero()}
    }

    pub fn offset(&self, o: T) -> Self {
        Self{min: self.min - o, max: self.max + o}
    }

    pub fn is_inside(&self, vec: Vec2<T>) -> bool
    where T: PartialOrd
    {
        self.min < vec && vec < self.max
    }
}

impl<T> From<(T, T, T, T)> for Rect<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self{min: Vec2::new(value.0, value.1), max: Vec2::new(value.2, value.3)}
    }
}
