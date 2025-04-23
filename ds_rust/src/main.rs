use ds_rust::mylib::libgraph::Edge;
//use ds_rust::mylib::libgraph::orthogonal_list::{OLGraph};
use ds_rust::mylib::libgraph::adjmatrix::MGraph;
//use ds_rust::mylib::app::graph::prim::{minimum_span_tree_prim, CloseEdge};
use ds_rust::mylib::app::graph::kruskal::{kruskal, LowCostEdge};
fn main(){
    let mut g = MGraph::<char, usize>::new();
    let v = ['a', 'b', 'c', 'd', 'e', 'f'];
    let e = [('a','b', 6), ('a', 'c', 1), ('a', 'd', 5), ('b', 'c', 5), ('b', 'e', 3), ('c', 'd', 5), ('c', 'e', 6), ('c', 'f', 4), ('d', 'f', 2), ('e', 'f', 6)];
    let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends_weight(from, to, weight)).collect();

    g.create_graph(v.to_vec(), edges, false);
    //let mut visited = vec![false; 6];
    //g.print();
    //g.dfs(0, &mut visited);
    let r = kruskal(&g); // (vexnum, u)
    for i in 0 ..r.len(){
        println!("{} -> {} : {}", r[i].head, r[i].tail, r[i].weight);
    }
}