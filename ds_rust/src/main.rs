use ds_rust::mylib::libgraph::Edge;
//use ds_rust::mylib::libgraph::orthogonal_list::{OLGraph};
use ds_rust::mylib::libgraph::adjmul_list::AMLGraph;
fn main(){
    let mut g = AMLGraph::<char, usize>::new();
    let v = ['a', 'b', 'c', 'd', 'e'];
    let e = [('a','b', 13), ('a', 'd', 8), ('a', 'e', 7), ('b', 'c', 1), ('b', 'd', 4), ('c', 'd', 2), ('d', 'e', 3)];
    let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends(from, to)).collect();

    g.create_udg(v.to_vec(), edges);
    let mut visited = vec![false; 5];
    g.print();
    g.dfs(0, &mut visited);
}