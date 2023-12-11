pub mod insert_sort;
pub mod swap_sort;

fn heapify<T>(v: &mut Vec<T>, end: usize) where T: Copy + PartialOrd{
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
}