Collide Tree
=============

A test for colliding many objects that runs at `O(n*lg(n))` and can be used at any number of dimensions

This runs faster than the naive, "compare everything with everything else" system, with over 25 elements.

With 1000 Elements, it runs in a 5th of the time. 

* Naive       : ~= 210.3ms
* Collide Tree: ~=  38.ms

How to use it.
------------

The CollideTree object is dependant on the following two traits 

```rust
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
```

When you create the  CollideTree, you must provide it with a \<T:BoundBox>. This will represent the area the tree will put the items into.
Items you add will all be of type \<L:Located+Debug> and the bounds method must return a T.

to get a list of collisions from a list of 'L', simply add them to the tree, and use the closure to mark what you want to do with the collisions.

```rust

    let list = create_range_list(1000);
    let mut tree = CollideTree::new(Bounds::new(0., 0., 1000., 1000.));

    //the vec for building the result 
    let mut t_col = Vec::new();


    for a in &list {
        //in the closure add the collision ids to the result. or do something else with them
        tree.add_item(a.clone(), &mut |a, b| t_col.push((a.id, b.id)));
    }
```



