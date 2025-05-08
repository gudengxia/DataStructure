use crate::mylib::libgraph::orthogonal_list::OLGraph;
use num_traits::{Bounded, Zero};
use std::rc::Rc;

/*1. The graph can have negative-weight edges 
    2. the running time is O(n*m)
    3. can not handle negative cycle*/

pub fn bellman_ford<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Zero + Bounded + std::ops::Add<Output = E>>(g: &OLGraph<T, E>, u: usize)-> (Vec<E>, Vec<usize>, bool){
    let n = g.vexnum;
    let mut d = vec![E::max_value(); n];
    let mut path = vec![n; n];
    let mut isrelaxed = false;
    d[u] = E::zero();
    path[u] = u;
    for _ in 1..n{
        isrelaxed = false;
        for i in 0..n{
            if d[i] == E::max_value() {
                break;}
            let mut p = g.firstarc(i);
            while let Some(ref e) = p{
                
                let j = e.borrow().headvex;
                let w = e.borrow().info.unwrap_or(E::max_value());    
                if w != E::max_value() && d[j] > d[i] + w{
                    d[j] = d[i] + w;
                    path[j] = i;
                    isrelaxed = true;
                }
                let q = e.borrow().tlink.as_ref().map(|arc| Rc::clone(arc));
                p = q;
            }
        }
    }  
    return (d, path, isrelaxed);
}

/*pub fn johnson<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Zero + Bounded + std::ops::Add<Output = E>>(g: &OLGraph<T, E>, u: usize)-> (Vec<E>, Vec<usize>){
    1. add a new vex s to the graph, set the weight of (s, v) = 0 for all vex v in G
    2. run bellman_ford from s and get dist[s, .], and if there is a negative cycle, return -1 gracefully
    3. pi[u, v] = dist[s, u] - dist[s, u] + w[u, v]
    4. for every vertex u in G' with weight pi[., .], run dijkstra to get pi[u, .]
    5 dist[u, v] = pi[u, v] - dist[s, u] + dist[s, v]
}*/