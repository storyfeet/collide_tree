use std::fmt::Debug;
use std::ops::*;

pub mod boxes;
#[cfg(test)]
mod test;

pub trait BoundBox: Sized + Clone {
    ///Split the box in half somehow, normally this should vary in direction
    fn split(&self) -> (Self, Self);
    ///Test if one box collides with another.
    fn hits(&self, b: &Self) -> bool;
}

pub trait Located {
    type Box: BoundBox;
    fn bounds(&self) -> Self::Box;
}

pub struct LocalTree<L: Located + Debug> {
    bound: L::Box,
    top: Vec<L>,
    children: Option<Box<(LocalTree<L>, LocalTree<L>)>>,
}

impl<L: Located + Debug> LocalTree<L> {
    pub fn new(bound: L::Box) -> Self {
        LocalTree {
            bound,
            top: Vec::new(),
            children: None,
        }
    }
    pub fn add_item<F: FnMut(&L, &L)>(&mut self, item: L, f: &mut F) {
        self.grow_children();

        let ib = item.bounds();
        for t in &self.top {
            if t.bounds().hits(&ib) {
                f(&t, &item);
            }
        }
        match &mut self.children {
            Some(b) => {
                let (l, r) = b.deref_mut();
                match (l.bound.hits(&ib), r.bound.hits(&ib)) {
                    (true, false) => l.add_item(item, f),
                    (false, true) => r.add_item(item, f),
                    _ => {
                        l.check_hits(&item, f);
                        r.check_hits(&item, f);
                        self.top.push(item);
                    }
                }
            }
            None => self.top.push(item),
        }
    }
    pub fn check_hits<F: FnMut(&L, &L)>(&self, item: &L, f: &mut F) {
        let ib = item.bounds();
        for t in &self.top {
            if t.bounds().hits(&ib) {
                f(&t, &item);
            }
        }
        if let Some(b) = &self.children {
            let (l, r) = b.deref();
            if l.bound.hits(&ib) {
                l.check_hits(item, f);
            }
            if r.bound.hits(&ib) {
                r.check_hits(item, f);
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
        self.children = Some(Box::new((l, r)));
    }

    pub fn for_each_collision<F: FnMut(&L, &L)>(&self, f: &mut F) {
        if self.top.len() > 0 {
            for a in 0..self.top.len() - 1 {
                for b in (a + 1)..self.top.len() {
                    if self.top[a].bounds().hits(&self.top[b].bounds()) {
                        f(&self.top[a], &self.top[b])
                    }
                }
            }
        }

        if let Some(b) = &self.children {
            let (l, r) = b.deref();
            for a in &self.top {
                l.check_hits(a, f);
                r.check_hits(a, f);
            }
            l.for_each_collision(f);
            r.for_each_collision(f);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
