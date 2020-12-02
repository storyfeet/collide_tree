use crate::boxes::{Bounds, IdBound};
use rand::Rng;

pub fn create_range_list(n: usize) -> Vec<IdBound<usize, Bounds<f64>>> {
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
