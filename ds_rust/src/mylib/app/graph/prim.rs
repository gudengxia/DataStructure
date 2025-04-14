use crate::mylib::libgraph::adjmul_list::AMLGraph;

use super::super::super::libgraph::adjmatrix::MGraph;


#[derive(Clone)]
pub struct CloseEdge<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>{
    pub adjvex: usize, // from whom the i-th vertex in the graph is connected to the minumum spanning tree 
    pub weight: Option<E>,
    pub visited: bool
}

impl<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd> CloseEdge<E>{
    pub fn new() -> CloseEdge<E>{
        CloseEdge{
            adjvex: 0,
            weight: None,
            visited: false
        }
    }
}

fn min_closest_edge<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>(close_edgesedges: & Vec<CloseEdge<E>>) -> Option<usize>{
    let mut min = E::default();
    let mut index = -1;
    for i in 0..close_edgesedges.len(){
        if !close_edgesedges[i].visited && close_edgesedges[i].weight.is_some() && close_edgesedges[i].weight.unwrap() < min {
            min = close_edgesedges[i].weight.unwrap();
            index = i as isize;
        }
    }

    if index == -1{
        return None;
    }
    Some(index as usize)
}
pub fn minimum_span_tree_prim<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>(g: &MGraph<T, E>, u: usize) -> Vec<CloseEdge<E>>{
    let mut close_edges = vec![CloseEdge::<E>::new(); g.vexnum];

    for j in 0..g.vexnum{      
        close_edges[j].visited = false;
        close_edges[j].adjvex = j;
        if let Some(w) = g.arcs[u][j].info{
            close_edges[j].weight = g.arcs[u][j].info.clone();
        }else{
            close_edges[j].weight = None;
        }    
    }

    close_edges[u].visited = true;
    for i in 1 ..close_edges.len(){
        if let Some(index) = min_closest_edge(&close_edges){
            close_edges[index].visited = true;
            for j in 0..g.vexnum{
                if !close_edges[j].visited && g.arcs[index][j].info.is_some() && g.arcs[index][j].info.unwrap() < close_edges[j].weight.unwrap_or(E::default()){
                    close_edges[j].weight = g.arcs[index][j].info.clone();
                    close_edges[j].adjvex = index;
                }
            }   
        }        
    }
    close_edges
}