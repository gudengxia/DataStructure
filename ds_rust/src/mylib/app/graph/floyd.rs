use super::super::super::libgraph::orthogonal_list::OLGraph;
use num_traits::{Bounded, Zero};
/*1. The graph can have negative-weight edges 
    2. the running time is O(n^3)
    3. can not handle negative cycle*/

pub fn floyd<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero +std::ops::Add<Output = E>>(g: &OLGraph<T, E>) -> (Vec<Vec<E>>, Vec<Vec<usize>>){
    let mut d = Vec::<Vec<E>>::new();
    let mut path = Vec::<Vec<usize>>::new();
    let n = g.vexnum;
    for _ in 0..n{
        let di = vec![E::default(); n];
        let pi = vec![0; n];
        d.push(di);
        path.push(pi);
    }

    
    for i in 0..n{
        for j in 0..n{
            if j == i{
                d[i][j] = E::zero();
                path[i][j] = i;
            }else{
                let w = g.weight(i, j);
                if let Some(e) = w{
                    d[i][j] = e;
                    path[i][j] = i;
                }else{
                    d[i][j] = E::max_value();
                    path[i][j] = n;
                }
            }
        } 
    }
    
    for k in 0..n{
        for i in 0..n{
            for j in 0..n{
                if d[i][k] != E::max_value() && d[k][j] != E::max_value() && d[i][k] + d[k][j] < d[i][j]{
                    d[i][j] = d[i][k] + d[k][j];
                    path[i][j] = path[k][j];
                }
            }
        }
    }
    (d, path)
}

#[cfg(test)]
mod test{
    use super::*;
    use crate::mylib::libgraph::Edge;
    use crate::mylib::libgraph::orthogonal_list::OLGraph;
    #[test]
    fn test_floyd(){
        let mut g = OLGraph::<char, usize>::new();
        let v = ['a', 'b', 'c', 'd'];
        let e = [('a','b', 1), ('a', 'd', 4), ('b', 'c', 9), ('b', 'd', 2), ('c', 'a', 3), ('c', 'b', 5), ('c', 'd', 8), ('d', 'c', 6)];
        let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends_weight(from, to, weight)).collect();

        g.create_dg(v.to_vec(), edges);
        
        g.print();
        
        let (r, p) = floyd(&g); // (vexnum, u)
        println!("{:?}", r);
        println!("{:?}", p);
    }
}