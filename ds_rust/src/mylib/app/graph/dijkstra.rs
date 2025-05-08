use std::vec;

use super::super::super::libgraph::orthogonal_list::OLGraph;
use num_traits::{Bounded, Zero};
/*1. Ensures that there is a path from u to v
  2. there is no negative-weight edge 
  3. the running time is O(n^2+m)*/
pub struct ShortestPath<E> where E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero + std::ops::Add<Output = E>{
    pub s: Vec<bool>, // s[i] is true if a shortest path from u to i has been found
    pub d: Vec<E>, // d[i] is the shortest path from u to i
    pub path: Vec<usize>
}

impl<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero + std::ops::Add<Output = E>>ShortestPath<E>{
    pub fn new() -> ShortestPath<E> {
        ShortestPath { s: Vec::new(), d: Vec::new() , path: Vec::new()}
    }
}
pub fn dijkstra<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero + std::ops::Add<Output = E>>(g: &OLGraph<T, E>, u: usize) -> (Vec<E>, Vec<usize>){
    //let mut sp = ShortestPath::<E>::new();
    let n = g.vexnum;
    let mut isfound = vec![false; n];
    let mut d = vec![E::max_value(); n];
    let mut path = vec![0; n];
    for i in 0..n{
        if g.weight(u, i).is_none(){
            d[i] = E::max_value();
        }else{
            d[i] = g.weight(u, i).unwrap();
        }
    }
    isfound[u] = true;
    d[u] = E::zero();
    path[u] = u;

    for _ in 1..n{
        let mut min = E::max_value();
        let mut index = -1isize;
        for w in 0 ..n{
            if !isfound[w] && d[w] < min{
                index = w as isize;
                min = d[w];
            }
        }
        if index == -1{
            break;
        }else{
            let v = index as usize;
            isfound[v] = true;
            for w in 0..n{
                if !isfound[w] && g.weight(v, w).is_some() && d[v] != E::max_value() && d[v] + g.weight(v, w).unwrap() < d[w]{
                    d[w] =  d[v] + g.weight(v, w).unwrap();
                    path[w] = v;
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
    fn test_dijkstra()
    {
        let mut g = OLGraph::<char, usize>::new();
        let v = ['a', 'b', 'c', 'd', 'e', 'f'];
        let e = [('a','c', 10), ('a', 'e', 30), ('a', 'f', 100), ('b', 'c', 5), ('c', 'd', 50), ('d', 'f', 10), ('e', 'd', 20), ('e', 'f', 60)];
        let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends_weight(from, to, weight)).collect();

        g.create_dg(v.to_vec(), edges);
        
        g.print();
    
        let r = dijkstra(&g, 0); // (vexnum, u)
        println!("{:?}", r);
    }
}
