use super::super::super::libgraph::adjmatrix::MGraph;
use num_traits::Bounded;

#[derive(Clone, Debug)]
pub struct CloseEdge<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded>{
    pub adjvex: usize, // from whom the i-th vertex in the graph is connected to the minumum spanning tree 
    pub weight: E,
    pub visited: bool
}

impl<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded> CloseEdge<E>{
    pub fn new() -> CloseEdge<E>{
        CloseEdge{
            adjvex: 0,
            weight: E::max_value(),
            visited: false
        }
    }
}

fn min_closest_edge<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded>(close_edgesedges: & Vec<CloseEdge<E>>) -> Option<usize>{
    let mut min = E::max_value();
    let mut index = -1;
    for i in 0..close_edgesedges.len(){
        if !close_edgesedges[i].visited && close_edgesedges[i].weight < min {
            min = close_edgesedges[i].weight;
            index = i as isize;
        }
    }

    if index == -1{
        return None;
    }
    Some(index as usize)
}
pub fn minimum_span_tree_prim<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded>(g: &MGraph<T, E>, u: usize) -> Vec<CloseEdge<E>>{
    let mut close_edges = vec![CloseEdge::<E>::new(); g.vexnum];

    for j in 0..g.vexnum{      
        close_edges[j].visited = false;
        close_edges[j].adjvex = u;
        close_edges[j].weight = g.arcs[u][j].info.unwrap_or(E::max_value());    
    }

    close_edges[u].visited = true;
    close_edges[u].weight = E::min_value();

    for _ in 1 ..close_edges.len(){
        if let Some(k) = min_closest_edge(&close_edges){
            close_edges[k].visited = true;
            for j in 0..g.vexnum{
                if !close_edges[j].visited && g.arcs[k][j].info.is_some() && g.arcs[k][j].info.unwrap() < close_edges[j].weight{
                    close_edges[j].weight = g.arcs[k][j].info.unwrap();
                    close_edges[j].adjvex = k;
                }
            }   
        }else{
            break;
        }        
    }
    close_edges
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::super::super::libgraph::adjmatrix::MGraph;
    use super::super::super::super::libgraph::Edge;
    #[test]
    fn test_prim() {
        let mut g = MGraph::<char, usize>::new();
    let v = ['a', 'b', 'c', 'd', 'e', 'f'];
    let e = [('a','b', 6), ('a', 'c', 1), ('a', 'd', 5), ('b', 'c', 5), ('b', 'e', 3), ('c', 'd', 5), ('c', 'e', 6), ('c', 'f', 4), ('d', 'f', 2), ('e', 'f', 6)];
    let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends_weight(from, to, weight)).collect();

    g.create_graph(v.to_vec(), edges, false);
    let mut visited = vec![false; 6];
    //g.print();
    g.dfs(0, &mut visited);
    let r = minimum_span_tree_prim(&g, 0); // (vexnum, u)
    for i in 0 ..r.len(){
        println!("{} -> {} : {}", i, r[i].adjvex, r[i].weight);
    }
    }         
}   