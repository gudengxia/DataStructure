use super::super::super::libgraph::adjmatrix::MGraph;
use num_traits::Bounded;
#[derive(Debug, Clone)]
pub struct LowCostEdge<E> where E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded {
    pub head: usize,
    pub tail: usize,
    pub weight: E,
}
pub fn kruskal<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd + Bounded>(g: &MGraph<T, E>) -> Vec<LowCostEdge<E>>{
    let mut lowcostedges = Vec::<LowCostEdge<E>>::new();
    if g.isdirected{
        for i in 0..g.vexnum{
            for j in 0..g.vexnum{
                if g.weight(i, j).is_some(){
                    lowcostedges.push(LowCostEdge::<E> { head: i, tail: j, weight: g.weight(i, j).unwrap()});
                }
            }
        }
    }else{
        for i in 0..g.vexnum{
            for j in i+1..g.vexnum{
                if g.weight(i, j).is_some(){
                    lowcostedges.push(LowCostEdge::<E> { head: i, tail: j, weight: g.weight(i, j).unwrap()});
                }
            }
        }
    }

    lowcostedges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap());
    
    let mut vex_set = Vec::<usize>::new();
    for i in 0..g.vexnum{
        vex_set.push(i);
    }

    let mut r = Vec::<LowCostEdge<E>>::new();
    let mut cnt = 0usize;
    for k in 0..lowcostedges.len(){
        let i = lowcostedges[k].head;
        let j = lowcostedges[k].tail;

        if vex_set[i] != vex_set[j]{
            r. push(lowcostedges[k].clone());
            for l in 0..g.vexnum{
                if vex_set[l] == vex_set[j]{
                    vex_set[l] = vex_set[i];
                }   
            }
            cnt += 1;
            if cnt == g.vexnum-1{
                break;    
            }
        }
    }
    r
}

#[cfg(test)] 
mod test{
    use super::kruskal;
    use super::super::super::super::libgraph::adjmatrix::MGraph;
    use super::super::super::super::libgraph::Edge;
    #[test]
    fn test_kruskal(){
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
}