extern crate collide_tree;
use collide_tree::boxes::Bounds;
use collide_tree::test_util::create_range_list;
use collide_tree::*;
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
        let mut tree = CollideTree::new(Bounds::new(0., 0., 1000., 1000.));

        let mut t_col = Vec::new();
        for a in &list {
            tree.add_item(a.clone(), &mut |a, b| t_col.push((a.id, b.id)));
        }
        t_col.len() > 0
    });
}
