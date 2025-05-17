use rand::Rng;
use ds_rust::mylib::libsort::*;
use ds_rust::mylib::libsort::insert_sort::*;
fn main() {
    let mut rng = rand::thread_rng();
    let mut v = Vec::<isize>::new(); // Creates an empty Vec<i32>
    for _ in 0..10 {
        let n: i32 = rng.gen_range(0..100); // Generates a random i32 in [10, 20)
        v.push(rng.gen_range(0..100));
    }
    
    println!("{:?}", v);

    //heap_sort(&mut v);
    bsearch_insert_sort(&mut v);
    println!("{:?}", v);
}