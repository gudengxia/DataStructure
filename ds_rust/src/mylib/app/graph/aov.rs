use crate::mylib::libgraph::orthogonal_list::OLGraph;
use num_traits::{Bounded, Zero};
use crate::mylib::liblist::queue::Queue;
use crate::mylib::liblist::queue::LinkQueue;
use std::rc::Rc;
//use std::cell::RefCell;

pub struct AOV<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero + std::ops::Add<Output = E>>{
    pub i: usize,
    pub j: usize,
    pub weight: E
}

impl <E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero + std::ops::Add<Output = E>> AOV<E> {
    pub fn new(i: usize, j: usize, weight: E) -> Self {
        Self { i, j, weight }
    }
}

pub fn topological<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero +std::ops::Add<Output = E>>(g: &OLGraph<T, E>) -> Vec<usize>{
    let n = g.vexnum;
    let mut topo = Vec::<usize>::new();
    let mut indegree = vec![0; g.vexnum];

    for i in 0..g.vexnum{
        let mut p = g.firstin(i);
        while let Some(ref e) = p{
            indegree[i] += 1;
            let q = e.borrow().hlink.as_ref().map(|arc| Rc::clone(arc));//netxin
            p = q;
        }//count the indegree of each vertex
    }
    
    let mut q = LinkQueue::<usize>::new();
    for i in 0..n{
        if indegree[i] == 0{
            let _ = q.enqueue(i);
            topo.push(i);
        }
    }

    while q.length() > 0{
        let u = q.dequeue().unwrap();
        let mut p = g.firstarc(u);//firstout
        while let Some(ref e) = p{
            let v = e.borrow().headvex;
            indegree[v] -= 1;
            if indegree[v] == 0{
                let _ = q.enqueue(v);
                topo.push(v);
            }
            let q = e.borrow().tlink.as_ref().map(|arc| Rc::clone(arc));//nextout
            p = q;
        }
    }

    topo
}

pub fn inverse_topological<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero +std::ops::Add<Output = E>>(g: &OLGraph<T, E>) -> Vec<usize>{
    let n = g.vexnum;
    let mut itopo = Vec::<usize>::new();
    let mut outdegree = vec![0; g.vexnum];

    for i in 0..g.vexnum{
        let mut p = g.firstarc(i);//firstout
        while let Some(ref e) = p{
            outdegree[i] += 1;
            let q = e.borrow().tlink.as_ref().map(|arc| Rc::clone(arc));//nextout
            p = q;
        }
    }
    
    let mut q = LinkQueue::<usize>::new();
    for i in 0..n{
        if outdegree[i] == 0{
            let _ = q.enqueue(i);
            itopo.push(i);
        }
    }

    while q.length() > 0{
        let u = q.dequeue().unwrap();
        let mut p = g.firstin(u);
        while let Some(ref e) = p{
            let v = e.borrow().tailvex;
            outdegree[v] -= 1;
            if outdegree[v] == 0{
                let _ = q.enqueue(v);
                itopo.push(v);
            }
            let q = e.borrow().hlink.as_ref().map(|arc| Rc::clone(arc));//netxin
            p = q;
        }
    }
    itopo
}

pub fn critical_path<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded + Zero +std::ops::Add<Output = E> + std::ops::Sub<Output = E>>(g: &OLGraph<T, E>)->Result<Vec<(usize, usize, E, E)>, &'static str>{
    let n = g.vexnum;
    let mut ve = vec![E::zero(); n];
    let mut vl = vec![E::zero(); n];
    let mut topo = Vec::<usize>::new();
    let mut indegree = vec![0; g.vexnum];
    //1. comuter every the earliest time of each vertex, which indicates the opening ceremony of an event.
    for i in 0..g.vexnum{
        let mut p = g.firstin(i);
        while let Some(ref e) = p{
            indegree[i] += 1;
            let q = e.borrow().hlink.as_ref().map(|arc| Rc::clone(arc));//netxin
            p = q;
        }//count the indegree of each vertex
    }
    
    let mut q = LinkQueue::<usize>::new();
    for i in 0..n{
        if indegree[i] == 0{
            let _ = q.enqueue(i);
            topo.push(i);
            ve[i] = E::zero(); // Initialize the earliest time of the source vertice
        }
    }

    while q.length() > 0{
        let u = q.dequeue().unwrap();
        let mut p = g.firstarc(u);//firstout
        while let Some(ref e) = p{
            let v = e.borrow().headvex;
            let w = e.borrow().info.unwrap_or(E::max_value());
            //update the earliest time of the vertex
            if ve[u] + w > ve[v]{
                ve[v] = ve[u] + w; //max{ve[v]}
            }
            indegree[v] -= 1;
            if indegree[v] == 0{
                let _ = q.enqueue(v);
                topo.push(v);
            }
            let q = e.borrow().tlink.as_ref().map(|arc| Rc::clone(arc));//nextout
            p = q;
        }
    }
    if topo.len() != n{
        return Err("There might be a cycle in the graph.");
    }
    
    //2. computer every the latest time of each vertex, which indicates the closing ceremony of an event.
    for i in 0..n{
        vl[i] = ve[n-1]; // very important error that has been fixed
    } // Initialize the vl 
    let mut itopo = Vec::<usize>::new();
    let mut outdegree = vec![0; g.vexnum];

    for i in 0..g.vexnum{
        let mut p = g.firstarc(i);//firstout
        while let Some(ref e) = p{
            outdegree[i] += 1;
            let q = e.borrow().tlink.as_ref().map(|arc| Rc::clone(arc));//nextout
            p = q;
        }
    }
    
    let mut q = LinkQueue::<usize>::new();
    for i in 0..n{
        if outdegree[i] == 0{
            let _ = q.enqueue(i);
            itopo.push(i);
        }
    }

    while q.length() > 0{
        let u = q.dequeue().unwrap();
       
        let mut p = g.firstin(u);
        while let Some(ref e) = p{
            //v-->u
            let v = e.borrow().tailvex;
            let w = e.borrow().info.unwrap_or(E::max_value());
            if vl[v]  > vl[u] - w{
                vl[v] = vl[u] - w; // min{vl[v]} update the latest time of the vertex
            }
            outdegree[v] -= 1;
            if outdegree[v] == 0{
                let _ = q.enqueue(v);
                itopo.push(v);
            }
            let q = e.borrow().hlink.as_ref().map(|arc| Rc::clone(arc));//netxin
            p = q;
        }
    }
    if itopo.len() != n{
        return Err("There might be a cycle in the graph.");
    }
    //3. computer the critical path
     for i in 0..n{
        println!("{}: {}, {}", i, ve[i], vl[i]);
    }
    let mut cpath = Vec::<(usize, usize, E, E)>::new();
    for i in 0..n{
        let mut p = g.firstarc(i);
        while let Some(ref e) = p{
            let j = e.borrow().headvex;
            let w = e.borrow().info.unwrap_or(E::max_value());
            if ve[i] + w == vl[j]{
                let _ = cpath.push((i, j, ve[i], vl[j]));
            }
            let q = e.borrow().tlink.as_ref().map(|arc| Rc::clone(arc));//nextout
            p = q;
        }
    }
    Ok(cpath)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mylib::libgraph::Edge;
    use crate::mylib::libgraph::orthogonal_list::*;
    #[test]
    fn test_aov(){
        let mut g = OLGraph::<char, usize>::new();
        let v = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
        let e = [('a','b', 1), ('a', 'c', 4), ('a', 'd', 1), ('a', 'l', 1), ('b', 'c', 9), ('c', 'e', 2), ('c', 'g', 3), ('c', 'h', 5), ('d', 'e', 8), ('e', 'g', 6), ('f', 'h', 1), ('g', 'l', 1), ('i', 'g', 1), ('i', 'k', 1), ('i', 'l', 1), ('j', 'l', 1), ('k', 'f', 1)];
        let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends_weight(from, to, weight)).collect();
        g.create_dg(v.to_vec(), edges);
        let topo = topological(&g);  
        println!("{:?}", topo);
        let itopo = inverse_topological(&g);
        println!("{:?}", itopo);
    }}