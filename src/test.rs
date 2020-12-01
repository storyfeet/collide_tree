use crate::*;
use rand::Rng;

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
    for id in 0..10 {
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

    let mut l_col = Vec::new();
    for a in 0..list.len() {
        for b in a..list.len() {
            if list[a].b.hits(&list[b].b) {
                l_col.push((a, b));
            }
        }
    }

    //time check tree
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
    assert_eq!(l_col, t_col);
}
