use ds_rust::mylib::libgraph::Edge;
use ds_rust::mylib::libgraph::orthogonal_list::OLGraph;
use ds_rust::mylib::app::graph::aov::topological;
use ds_rust::mylib::libtree::binarytree::BiTree;
/*fn main(){
    let mut g = OLGraph::<char, usize>::new();
    let v = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
    let e = [('a','b', 1), ('a', 'c', 4), ('a', 'd', 1), ('a', 'l', 1), ('b', 'c', 9), ('c', 'e', 2), ('c', 'g', 3), ('c', 'h', 5), ('d', 'e', 8), ('e', 'g', 6), ('f', 'h', 1), ('g', 'l', 1), ('i', 'g', 1), ('i', 'k', 1), ('i', 'l', 1), ('j', 'l', 1), ('k', 'f', 1)];
    let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends_weight(from, to, weight)).collect();

    g.create_dg(v.to_vec(), edges);
    let topo = topological(&g);  
    println!("{:?}", topo)
}*/

fn main(){
    let mut bt = BiTree::<char>::new();
    let v = [Some('a'), Some('b'), Some('c'), None, None, Some('d'), Some('e'), None, Some('g'), None, None, Some('f'), None, None, Some('h'), None, None];
    let _ = bt.build_tree(v.to_vec());
    println!("preorder:");
    bt.preorder();
    println!("inorder:");
    bt.inorder();
    println!("postorder:");
    bt.postorder();
    println!("traverse:");
}