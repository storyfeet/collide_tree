use crate::boxes::Bounds;
use crate::*;
use rand::Rng;
use std::cmp::Ordering;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct IBound {
    id: usize,
    b: Bounds<f64>,
}

impl Located for IBound {
    type ID = usize;
    type Box = Bounds<f64>;
    fn id(&self) -> Self::ID {
        self.id
    }

    fn bounds(&self) -> Bounds<f64> {
        self.b.clone()
    }
}

fn create_range_list(n: usize) -> Vec<IBound> {
    let mut rnd = rand::thread_rng();
    let mut list = Vec::new();
    for id in 0..n {
        let x: f64 = rnd.gen_range(0., 1000.);
        let y: f64 = rnd.gen_range(0., 1000.);
        let w = rnd.gen_range(0., 200.);
        let h = rnd.gen_range(0., 200.);

        let v = IBound {
            id,
            b: Bounds::new(x, y, w, h),
        };
        list.push(v);
    }
    list
}

#[test]
fn can_it_be_found() {
    let list = create_range_list(1000);

    let mut l_col = Vec::new();
    for a in 0..(list.len() - 1) {
        for b in (a + 1)..list.len() {
            if list[a].b.hits(&list[b].b) {
                l_col.push((a, b));
            }
        }
    }

    let mut tree: LocalTree<IBound> = LocalTree::new(Bounds::new(0., 0., 1000., 1000.));

    let mut t_col = Vec::new();
    for a in &list {
        tree.add_item(a.clone(), &mut t_col);
    }

    t_col.sort_by(|(a1, a2), (b1, b2)| match a1.cmp(b1) {
        Ordering::Equal => a2.cmp(b2),
        v => v,
    });

    assert_eq!(l_col.len(), t_col.len());
    assert_eq!(l_col, t_col);
}

/* #[bench]
fn bench_square_list(b: &mut Bencher) {
    b.iter(|| {
        let list = create_range_list(1000);
        let mut l_col = Vec::new();
        for a in 0..(list.len() - 1) {
            for b in (a + 1)..list.len() {
                if list[a].b.hits(&list[b].b) {
                    l_col.push((a, b));
                }
            }
        }
    })
}
#[bench]
fn bench_tree_list(b: &mut Bencher) {
    b.iter(|| {
        let list = create_range_list(1000);
        let mut tree: LocalTree<IBound> = LocalTree::new(Bounds::new(0., 0., 1000., 1000.));

        let mut t_col = Vec::new();
        for a in &list {
            tree.add_item(a.clone(), &mut t_col);
        }
    })
}*/
