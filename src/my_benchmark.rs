extern crate collide_tree;
use collide_tree::boxes::{Bounds, IdBound};
use collide_tree::*;
use rand::Rng;
use std::time::{Duration, Instant};

fn b_mark<F: Fn() -> bool>(s: &str, f: F) {
    let mut t_time = Duration::from_millis(0);
    for _ in 0..5 {
        let start = Instant::now();
        match f() {
            true => t_time += start.elapsed(),
            _ => {
                println!("test: failed {}", s);
                return;
            }
        }
    }
    println!(
        "Ran test : \"{}\" 5 times in {:?}, Avg time = {:?}",
        s,
        t_time,
        t_time.div_f64(5.),
    );
}

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

fn main() {
    b_mark("square", || {
        let list = create_range_list(1000);
        let mut l_col = Vec::new();
        for a in 0..(list.len() - 1) {
            for b in (a + 1)..list.len() {
                if list[a].bounds().hits(&list[b].bounds()) {
                    l_col.push((a, b));
                }
            }
        }
        l_col.len() > 0
    });
    b_mark("tree", || {
        let list = create_range_list(1000);
        let mut tree = LocalTree::new(Bounds::new(0., 0., 1000., 1000.));

        let mut t_col = Vec::new();
        for a in &list {
            tree.add_item(a.clone(), &mut t_col);
        }
        t_col.len() > 0
    });
}
