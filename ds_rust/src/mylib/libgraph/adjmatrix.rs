use std::sync::Arc;

use super::Edge;
#[derive(Debug,Default, Clone)]
pub struct ArcCell<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>{
    pub info: Option<E>
}

impl<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd> ArcCell<E>{
    pub fn new() -> ArcCell<E>{
        ArcCell{info: None}
    }
}

pub struct MGraph<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>{
    pub vexs: Vec<T>,
    pub arcs: Vec<Vec<ArcCell<E>>>,
    pub vexnum: usize,
    pub arcnum: usize,
    pub isdirected: bool
}   

impl <T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd> MGraph<T, E>{
    pub fn new() -> MGraph<T, E>{
        MGraph{vexs: Vec::<T>::new(), 
        arcs: Vec::<Vec<ArcCell<E>>>::new(), 
        vexnum: 0usize, 
        arcnum: 0usize, 
        isdirected: false}
    }

    pub fn create_graph(&mut self, vexs: Vec<T>, arcs: Vec<Edge<T, E>>, isdirected: bool){
        self.vexnum = vexs.len();
        self.arcnum = arcs.len();
        self.vexs = vexs;
        self.isdirected = isdirected;
        /*for i in 0..self.vexnum{
            self.arcs.push(vec![ArcCell::default(); self.vexnum]);
        }*/
        self.arcs = vec![vec![ArcCell::default(); self.vexnum]; self.vexnum];

        for arc in arcs{
            let i = self.locate_vex(arc.tailvex).expect("err: node does not exist");
            let j = self.locate_vex(arc.headvex).expect("err: node does not exist");
            self.arcs[i][j].info = arc.info;
            if !isdirected{ //undirected graph
                self.arcs[j][i].info = arc.info;
            }
        }
    }

    pub fn locate_vex(&self, vex: T) -> Option<usize>{
        for (i, v) in self.vexs.iter().enumerate(){
            if *v == vex{
                return Some(i);
            }
        }
        return None;
    }

    pub fn weight(&self, i: usize, j: usize) -> Option<E>{
        match self.arcs[i][j].info{
            Some(w) => Some(w),
            None => None
        }
    }
}   

