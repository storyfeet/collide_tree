//! Collide Tree
//! ============
//!
//! The following Test Shows the naive method, and the tree method will get the same results.
//!
//! ```rust
//! use collide_tree::test_util::create_range_list;
//! use collide_tree::*;
//! use collide_tree::boxes::*;
//! use std::cmp::Ordering;
//!
//!
//! let list = create_range_list(1000);
//!
//! // NAIVE collision detection
//! let mut l_col = Vec::new();
//! for a in 0..(list.len() - 1) {
//!     for b in (a + 1)..list.len() {
//!         if list[a].bounds().hits(&list[b].bounds()) {
//!             l_col.push((a, b));
//!         }
//!     }
//! }
//! // COLLIDE_TREE collision detection
//! // The tree is largely expected to be created and filled every frame.
//! let mut tree = CollideTree::new(Bounds::new(0., 0., 1000., 1000.));
//!
//! let mut t_col: Vec<(usize, usize)> = Vec::new();
//! for a in &list {
//!     //Collisions are detected as you add items to the tree
//!     //The closure is called on every collision
//!     tree.add_item(a.clone(), &mut |a, b| t_col.push((a.id, b.id)));
//! }
//!
//! // Sort so results match on order
//! t_col.sort_by(|(a1, a2), (b1, b2)| match a1.cmp(b1) {
//!     Ordering::Equal => a2.cmp(b2),
//!     v => v,
//! });
//!
//! assert_eq!(l_col.len(), t_col.len());
//! assert_eq!(l_col, t_col);
//! ```

use std::fmt::Debug;
use std::ops::*;

pub mod boxes;
#[cfg(test)]
mod test;
pub mod test_util;

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

pub struct CollideTree<L: Located + Debug> {
    bound: L::Box,
    top: Vec<L>,
    children: Option<Box<(CollideTree<L>, CollideTree<L>)>>,
}

impl<L: Located + Debug> CollideTree<L> {
    pub fn new(bound: L::Box) -> Self {
        CollideTree {
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
