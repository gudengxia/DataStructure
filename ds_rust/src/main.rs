use ds_rust::mylib::libgraph::Edge;
use ds_rust::mylib::libgraph::orthogonal_list::OLGraph;
use ds_rust::mylib::app::graph::aov::{inverse_topological, topological, critical_path};
use ds_rust::mylib::libtree::binarytree::BiTree;

fn main(){
    let mut g = OLGraph::<char, usize>::new();
    let v = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];
    let e = [('a','b', 6), ('a', 'c', 4), ('a', 'd', 5), ('b', 'e', 1), ('c', 'e', 1), ('d', 'f', 2), ('e', 'g', 9), ('e', 'h', 7), ('f', 'h', 4), ('g', 'i', 2), ('h', 'i', 4)];
    
    let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends_weight(from, to, weight)).collect();

    g.create_dg(v.to_vec(), edges);
    let topo = topological(&g);  
    println!("{:?}", topo);
    let itopo = inverse_topological(&g);
    println!("{:?}", itopo);
    let cpath = critical_path(&g).unwrap();
    for e in cpath{
        println!("{:?}", e);
    }
}

