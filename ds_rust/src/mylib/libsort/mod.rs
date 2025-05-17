pub mod insert_sort;
pub mod swap_sort;

/*fn heapify<T>(v: &mut Vec<T>, end: usize) where T: Copy + PartialOrd{
    let mut last = end;
    if last & 0x1usize == 1{
        let parent = (last + 1) / 2 - 1;
        if v[parent] < v[last]{
            let e = v[parent];
            v[parent] = v[last];
            v[last] = e;
        } 
        last -= 1;
    }
    for i in (2..=last).rev().step_by(2){       
        let parent = i / 2 - 1;
        let pmax =  if v[i-1] <= v[i]{
            i
        } else{
            i - 1
        };
        if v[parent] < v[pmax]{
            let e = v[parent];
            v[parent] = v[pmax];
            v[pmax] = e;
        }
    }
}



pub fn heap_sort<T>(v: &mut Vec<T>) where T: Copy + PartialOrd{
    let n = v.len();
    for i in (1..n).rev(){
        heapify(v, i);
        (v[0], v[i]) = (v[i], v[0]);    
    }
}*/

pub fn heap_adjust<T: PartialOrd + Copy>(v: &mut Vec<T>,  start: usize, end: usize){
    let e = v[start];
    let mut s = start;
    let mut j = 2 * s + 1;
    while j <= end{
        if j + 1 <= end && v[j] < v[j + 1]{
            j = j + 1;
        }
        if e >= v[j]{
            break;
        }// if the root is larger than its larger child, break
        v[s] = v[j];
        s = j;
        j = 2 * s + 1;
    }
    v[s] = e;
}// build a max heap

pub fn heap_sort<T: PartialOrd + Copy>(v: &mut Vec<T>){
    let n = v.len();
    let last_non_leaf = (n + 1) / 2 - 1;
    for i in (0..=last_non_leaf).rev(){
        heap_adjust(v, i, n - 1);
    }//create heap

    for i in (1..n).rev(){
        (v[0], v[i]) = (v[i], v[0]);  //adjust the largest value to the end the unsorted list
        heap_adjust(v, 0, i-1);  
    }
}

#[cfg(test)]
mod test{
    use super::*;
    //use std::time::Instant;
    use rand::Rng;
    #[test]
    fn test_heap_sort(){
        let mut rng = rand::thread_rng();
        let mut v = Vec::<i32>::new(); // Creates an empty Vec<i32>
        for _ in 0..10 {
            let n: i32 = rng.gen_range(0..100); // Generates a random i32 in [10, 20)
            v.push(n);
        }
    
        println!("{:?}", v);

        heap_sort(&mut v);
        //bsearch_insert_sort(&mut v);
        println!("{:?}", v);
    }
}