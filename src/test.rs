use crate::boxes::Bounds;
use crate::*;
use std::cmp::Ordering;

#[test]
fn can_it_be_found() {
    let list = crate::test_util::create_range_list(1000);

    let mut l_col = Vec::new();
    for a in 0..(list.len() - 1) {
        for b in (a + 1)..list.len() {
            if list[a].bounds().hits(&list[b].bounds()) {
                l_col.push((a, b));
            }
        }
    }

    let mut tree = CollideTree::new(Bounds::new(0., 0., 1000., 1000.));

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
