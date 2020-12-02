use num_traits::{NumAssign, NumCast};
use std::fmt::Debug;
pub trait BNum: PartialOrd + NumAssign + NumCast + Copy + Clone + Debug {}

impl<T: PartialOrd + NumAssign + NumCast + Sized + Copy + Clone + Debug> BNum for T {}
pub fn qcast<A: NumCast, B: NumCast>(a: A) -> B {
    NumCast::from(a).unwrap()
}

#[derive(Clone, Debug)]
pub struct Bounds<T: BNum> {
    x: T,
    y: T,
    w: T,
    h: T,
}

impl<T: BNum> Bounds<T> {
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Bounds { x, y, w, h }
    }
}

impl<T: BNum> crate::BoundBox for Bounds<T> {
    fn hits(&self, b: &Self) -> bool {
        if self.x > b.x + b.w || b.x > self.x + self.w {
            return false;
        }
        if self.y > b.y + b.h || b.y > self.y + self.h {
            return false;
        }
        true
    }

    fn split(&self) -> (Self, Self) {
        match self.w > self.h {
            true => (
                Bounds {
                    w: self.w / qcast(2),
                    ..*self
                },
                Bounds {
                    x: self.x + self.w / qcast(2),
                    w: self.w - (self.w / qcast(2)),
                    ..*self
                },
            ),
            false => (
                Bounds {
                    h: self.h / qcast(2),
                    ..*self
                },
                Bounds {
                    y: self.y + self.h / qcast(2),
                    h: self.h - (self.h / qcast(2)),
                    ..*self
                },
            ),
        }
    }
}
