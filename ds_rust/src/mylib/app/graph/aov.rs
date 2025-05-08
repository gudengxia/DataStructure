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
            let q = e.borrow().hlink.as_ref().map(|arc| Rc::clone(arc));
            p = q;
        }
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
        let mut p = g.firstarc(u);
        while let Some(ref e) = p{
            let v = e.borrow().headvex;
            indegree[v] -= 1;
            if indegree[v] == 0{
                let _ = q.enqueue(v);
                topo.push(v);
            }
            let q = e.borrow().tlink.as_ref().map(|arc| Rc::clone(arc));
            p = q;
        }
    }
    topo
}