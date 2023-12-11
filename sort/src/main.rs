pub mod libsort;
use libsort::*;
use libsort::insert_sort::*;
use libsort::swap_sort::{bubble_sort, quick_sort};
use rand::prelude::*;

fn main() {
    const N: usize = 20;
    let mut rng = rand::thread_rng();
    let mut v = vec![0; N];
    for i in 0..N {
        v[i] = rng.gen_range(0..100);
        print!("{} ", v[i]);
    }
    println!("");
    //insert_sort(&mut v);
    //bubble_sort(&mut v);
    //quick_sort(&mut v);
    //two_direction_insert_sort(&mut v);
    //shell_insert_sort(&mut v);
    //list_insert_sort(&mut v);
    heap_sort(&mut v);
    for i in 0..N {
        print!("{} ", v[i]);
    }
    println!("");
}
