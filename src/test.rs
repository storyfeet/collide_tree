use crate::boxes::{Bounds, IdBound};
use crate::*;
use rand::Rng;
use std::cmp::Ordering;

fn create_range_list(n: usize) -> Vec<IdBound<usize, Bounds<f64>>> {
    let mut rnd = rand::thread_rng();
    let mut list = Vec::new();
    for id in 0..n {
        let x: f64 = rnd.gen_range(0., 1000.);
        let y: f64 = rnd.gen_range(0., 1000.);
        let w = rnd.gen_range(0., 200.);
        let h = rnd.gen_range(0., 200.);

        let v = IdBound::new(id, Bounds::new(x, y, w, h));
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
            if list[a].bounds().hits(&list[b].bounds()) {
                l_col.push((a, b));
            }
        }
    }

    let mut tree = LocalTree::new(Bounds::new(0., 0., 1000., 1000.));

    let mut t_col: Vec<(usize, usize)> = Vec::new();
    for a in &list {
        tree.add_item(a.clone(), &mut |a, b| t_col.push((a.id, b.id)));
    }

    t_col.sort_by(|(a1, a2), (b1, b2)| match a1.cmp(b1) {
        Ordering::Equal => a2.cmp(b2),
        v => v,
    });

    assert_eq!(l_col.len(), t_col.len());
    assert_eq!(l_col, t_col);
}
