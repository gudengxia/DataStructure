
pub fn insert_sort<T>(v: &mut Vec<T>)
    where T: PartialOrd + Copy + std::fmt::Debug,{
        let n:usize = v.len();
        for i in 1..n
        {
            if v[i] < v[i-1]{
                let e = v[i];
                let mut j: isize = (i-1) as isize;
                while j >= 0 && v[j as usize] > e{
                    v[(j+1) as usize] = v[j as usize];
                    j = j - 1;
                }
            
                v[(j+1) as usize] = e;
            }
        }
}

pub fn two_direction_insert_sort<T>(v: &mut Vec<T>) 
where T: Copy + PartialOrd + std::fmt::Debug,{
    let n: usize = v.len();
    let mut t: Vec<T> = v.clone();

    let mut front: usize = 0;
    let mut end: usize = 0;

    for i in 1..n{
        let mut j: usize;
        if v[i] < t[0]{
            j = front;
            while t[j] < v[i]{
                t[(j + n - 1) % n] = t[j];
                j = (j + 1) % n;
            }
            j = (j + n - 1) % n;
            t[j] = v[i];
            front = (front + n - 1) % n;
        }
        else {
            j = end;
            while t[j] > v[i] {
                t[j + 1] = t[j];
                j = j -1;
            } 
            j = j + 1;
            t[j] = v[i];
            end = (end + 1) % n; 
        }
    }

    for i in 0..n{
        v[i] = t[(front + i) % n];
    }
}

pub fn list_insert_sort<T>(v: &mut Vec<T>) where T: Copy + PartialOrd{
    let n = v.len();
    let mut h: usize = 0;
    let mut next: Vec<isize> = vec![-1; n];

    for i in 1..n{
        if v[i] < v[h]{
            next[i] = h as isize;
            h = i;
        }//if the value is smaller than the head, the value is the new head
        else {
            let mut j = h;
            while next[j] != -1 && v[next[j] as usize] < v[i]{
                j = next[j] as usize;
            }// find the last value that is smaller than the value in the sorted static link list
            next[i] = next[j];  //insert a new node
            next[j] = i as isize;
        }
    }
    println!("h={}", h);
    println!("{:?}", next);

    let mut p = h;

    for i in 0..n{
        if p != i{
            while p < i{
                p = next[p] as usize;
            }//the value should be in the i-th position is now in the p-th position
            (v[i], v[p]) = (v[p], v[i]);
            let q = next[p] as usize;
            next[p] = next[i]; //update the moved value's next pointer to be the original value
            next[i] = p as isize; //the original value's moved to the p-th position
            p = q;//update p
        }
    }
}

pub fn shell_insert_sort<T>(v: &mut Vec<T>) where T: Copy + PartialOrd{
    let steps = [7, 3, 1];
    let n = v.len();
    for step in steps{
        for grp in 0..step{
            let mut i = grp + step;
            while i < n{
                if v[i] < v[i - step] {
                    let e = v[i];
                    let mut j = (i - step) as isize;
                    while j >= 0 && v[j as usize] > e{
                        v[(j + step as isize) as usize] = v[j as usize];
                        j -= step as isize;
                    }
                    v[(j + step as isize) as usize] = e;
                }
                i += step;
            }
        } 
    }
}

pub fn bsearch_insert_sort<T>(v: &mut Vec<T>) where T: Copy + PartialOrd{
    let n = v.len();
    for i in 1..n{
        let e = v[i];
        let mut low = 0isize;
        let mut high = (i - 1) as isize;
        while low <= high{
            let mid = (low + high) / 2;
            if e < v[mid as usize]{
                high = mid - 1;
            }
            else{
                low = mid + 1;
            }
        }
        let mut j = i - 1;
        while j >= low as usize{
            v[j + 1] = v[j];
            j -= 1;
        }
        v[low as usize] = e;
    }
}