pub fn bubble_sort<T>(v: &mut Vec<T> )
where T: Copy + PartialOrd,{
    let mut e: T;
    let n = v.len();
    for i in 1..n{
        let mut cnt: usize = 0;
        for j in 1..n-i{
            if v[j-1] > v[j]{
                e = v[j-1];
                v[j-1] = v[j];
                v[j] = e;
                cnt = cnt + 1;
            }
        }
        if cnt == 0{
            break;
        }
    }
}

fn partition<T>(v: &mut Vec<T>, mut low: usize, mut high:usize) -> usize
where T: Copy + PartialOrd, {
    let pivotkey = v[low];

    while low < high {
        while low < high && v[high] >= pivotkey{
            high -= 1;
        }
        v[low] = v[high];

        while low < high && v[low] <= pivotkey{
            low = low + 1;
        }
        v[high] = v[low];
    }
    v[low] = pivotkey;
    low
}

fn qsort<T>(v: &mut Vec<T>, low: usize, high: usize)
where T: Copy + PartialOrd, {
    if low < high {
        let pivotloc = partition(v, low, high);
        qsort(v, low, pivotloc-1);
        qsort(v, pivotloc+1, high);
    }
}

pub fn quick_sort<T>(v: &mut Vec<T>)
where T: Copy + PartialOrd,
{
    let n = v.len();
    qsort(v, 0, n-1);
}