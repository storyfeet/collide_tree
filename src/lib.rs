use num_traits::{NumAssign, NumCast};
use std::fmt::Debug;
use std::ops::*;

#[cfg(test)]
mod test;

pub trait BNum: PartialOrd + NumAssign + NumCast + Copy + Clone + Debug {}

impl<T: PartialOrd + NumAssign + NumCast + Sized + Copy + Clone + Debug> BNum for T {}

#[derive(Clone, Debug)]
pub struct Bounds<T: BNum> {
    x: T,
    y: T,
    w: T,
    h: T,
}

pub fn qcast<A: NumCast, B: NumCast>(a: A) -> B {
    NumCast::from(a).unwrap()
}

impl<T: BNum> Bounds<T> {
    pub fn hits(&self, b: &Self) -> bool {
        if self.x > b.x + b.w || b.x > self.x + self.w {
            return false;
        }
        if self.y > b.y + b.h || b.y > self.y + self.h {
            return false;
        }
        true
    }

    pub fn split(&self) -> (Self, Self) {
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

pub trait Located<T: BNum> {
    type ID;
    fn id(&self) -> Self::ID;
    fn bounds(&self) -> Bounds<T>;
}

pub struct LocalTree<L: Located<T> + Debug, T: BNum> {
    bound: Bounds<T>,
    top: Vec<L>,
    children: Option<Box<(LocalTree<L, T>, LocalTree<L, T>)>>,
}

impl<L: Located<T> + Debug, T: BNum> LocalTree<L, T> {
    pub fn new(bound: Bounds<T>) -> Self {
        LocalTree {
            bound,
            top: Vec::new(),
            children: None,
        }
    }
    pub fn add_item(&mut self, item: L, v: &mut Vec<(L::ID, L::ID)>) {
        let ib = item.bounds();
        self.grow_children();
        for t in &self.top {
            if t.bounds().hits(&ib) {
                v.push((t.id(), item.id()));
            }
        }
        match &mut self.children {
            Some(b) => {
                let (l, r) = b.deref_mut();
                match (l.bound.hits(&ib), r.bound.hits(&ib)) {
                    (true, false) => l.add_item(item, v),
                    (false, true) => r.add_item(item, v),
                    _ => {
                        l.check_hits(&item, v);
                        r.check_hits(&item, v);
                        self.top.push(item);
                    }
                }
            }
            None => self.top.push(item),
        }
    }
    pub fn check_hits(&mut self, item: &L, v: &mut Vec<(L::ID, L::ID)>) {
        let ib = item.bounds();
        for t in &self.top {
            if t.bounds().hits(&ib) {
                v.push((t.id(), item.id()));
            }
        }
        if let Some(b) = &mut self.children {
            let (l, r) = b.deref_mut();
            if l.bound.hits(&ib) {
                l.check_hits(item, v);
            }
            if r.bound.hits(&ib) {
                r.check_hits(item, v);
            }
        }
    }

    pub fn grow_children(&mut self) {
        if let Some(_) = self.children {
            return;
        }
        if self.top.len() < 8 {
            return;
        }
        let (l, r) = self.bound.split();
        println!("l:{:?}   ,   r:{:?}", l, r);

        let (mut l, mut r) = (Self::new(l), Self::new(r));
        let mut newtop = Vec::new();
        std::mem::swap(&mut newtop, &mut self.top);
        for v in newtop {
            let ib = v.bounds();
            match (l.bound.hits(&ib), r.bound.hits(&ib)) {
                (true, false) => l.top.push(v),
                (false, true) => r.top.push(v),
                _ => self.top.push(v),
            }
        }
        println!("   ltop = {:?}", l.top);
        println!("   rtop = {:?}", r.top);
        println!("   top = {:?}", self.top);
        self.children = Some(Box::new((l, r)));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
