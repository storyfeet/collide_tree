use crate::*;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct IBound {
    id: usize,
    b: Bounds<f64>,
}

impl Located<f64> for IBound {
    type ID = usize;
    fn id(&self) -> Self::ID {
        self.id
    }

    fn bounds(&self) -> Bounds<f64> {
        self.b.clone()
    }
}

#[test]
fn can_it_be_found() {
    let mut rnd = rand::thread_rng();
    let mut list = Vec::new();
    for id in 0..50 {
        let x: f64 = rnd.gen_range(0., 1000.);
        let y: f64 = rnd.gen_range(0., 1000.);
        let w = rnd.gen_range(0., 200.);
        let h = rnd.gen_range(0., 200.);

        let v = IBound {
            id,
            b: Bounds { x, y, w, h },
        };
        list.push(v);
    }

    //time check list
    let square_time = Instant::now();

    let mut l_col = Vec::new();
    for a in 0..(list.len() - 1) {
        for b in (a + 1)..list.len() {
            if list[a].b.hits(&list[b].b) {
                l_col.push((a, b));
            }
        }
    }

    let square_elapsed = square_time.elapsed();

    //time check tree

    let tree_time = Instant::now();
    let mut tree: LocalTree<IBound, f64> = LocalTree::new(Bounds {
        x: 0.,
        y: 0.,
        w: 1000.,
        h: 1000.,
    });

    let mut t_col = Vec::new();
    for a in &list {
        tree.add_item(a.clone(), &mut t_col);
    }

    let tree_elapsed = tree_time.elapsed();

    t_col.sort_by(|(a1, a2), (b1, b2)| match a1.cmp(b1) {
        Ordering::Equal => a2.cmp(b2),
        v => v,
    });

    assert!(
        square_elapsed < tree_elapsed,
        "Tree took longer t{:?}, s{:?}",
        tree_elapsed,
        square_elapsed
    );
    assert_eq!(l_col.len(), t_col.len());
    assert_eq!(l_col, t_col);
}
