use super::Edge;
use std::rc::Rc;
use core::cell::RefCell;
struct ArcBox<E: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    tailvex: usize,
    headvex: usize,
    hlink: Option<Rc<RefCell<ArcBox<E>>>>,
    tlink: Option<Rc<RefCell<ArcBox<E>>>>,
    info: Option<E>
}

impl<E: std::default::Default + Copy + std::fmt::Display + PartialEq> ArcBox<E>{
    fn new()->ArcBox<E>{
        ArcBox{
            tailvex: 0,
            headvex: 0,
            hlink: None,
            tlink: None,
            info: None
        }
    }
}

struct VexNode<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    data: T,
    firstin: Option<Rc<RefCell<ArcBox<E>>>>,
    firstout: Option<Rc<RefCell<ArcBox<E>>>>
}
impl<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq>VexNode<T, E>{
    fn new()->VexNode<T, E>{
        VexNode{
            data: T::default(),
            firstin: None,
            firstout: None
        }
    }
}


pub struct OLGraph<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    xlist: Vec<VexNode<T, E>>,
    vexnum: usize,
    arcnum: usize
}


impl<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd> OLGraph<T, E>{
    pub fn new()->OLGraph<T, E>{
        OLGraph { xlist: Vec::new(), vexnum: 0, arcnum: 0}
    }

    fn locate_vex(&self, v: T) ->Result<usize, &'static str>{
        let n = self.xlist.len();
        for i in 0..n{
            if self.xlist[i].data == v{
                return Ok(i);
            }
        }
        Err("locate_vex error: the vertex does not exist")
    }
    pub fn create_dg(&mut self, vex: Vec<T>, edge: Vec<Edge<T, E>>){
        self.vexnum = vex.len();
        for v in vex{
            let mut vertex = VexNode::new();
            vertex.data = v;
            self.xlist.push(vertex);
        }

        self.arcnum = edge.len();
        for e in edge{
            let i = self.locate_vex(e.tailvex).expect("err: node does not exist");
            let j = self.locate_vex(e.headvex).expect("err: node does not exist");
            let arc = Rc::new(RefCell::new(ArcBox::<E>::new()));
            arc.borrow_mut().tailvex = i;
            arc.borrow_mut().headvex = j;
            arc.borrow_mut().hlink = self.xlist[j].firstin.take();
            arc.borrow_mut().tlink = self.xlist[i].firstout.take();
            arc.borrow_mut().info = e.info;
            self.xlist[j].firstin = Some(Rc::clone(&arc)); 
            self.xlist[i].firstout = Some(Rc::clone(&arc)); 
        }
    }

    pub fn firstarc(&self, i: usize)->Option<Rc<RefCell<ArcBox<E>>>>{
        match &self.xlist[i].firstout{
            Some(e) => {
                return Some(Rc::clone(e));
            }
            None => return  None
        }
    }
    pub fn nextarc(&self, cur: Option<Rc<RefCell<ArcBox<E>>>>) -> Option<Rc<RefCell<ArcBox<E>>>>{
        if let Some(e) = cur{
            if let Some(arc) = &e.borrow().tlink{
                return Some(Rc::clone(&arc))
            } 
        }
        None
    }

    pub fn dfs(&mut self, i: usize, visited: &mut  Vec<bool>){
        let mut s = Vec::<Rc<RefCell<ArcBox<E>>>>::new();
        visited[i] = true;
        println!("Visit {}", self.xlist[i].data);
        
        if let Some(e) = &self.xlist[i].firstout{
            s.push(Rc::clone(&e));
        }

        while let Some(e) = s.clone().last(){
            //let i = e.borrow().tailvex;
            let j = e.borrow().headvex;

            if !visited[j]{
                visited[j] = true;
                println!("Visit {}", self.xlist[j].data);
                if let Some(arc) = &self.xlist[j].firstout{
                    s.push(Rc::clone(arc));
                }    
            }
            else{
                let _ = s.pop();
                if let Some(arc) = self.nextarc(Some(Rc::clone(e))){
                    s.push(arc);
                }
            }
        }
    }

    pub fn print(&self){
        for v in & self.xlist{
            print!("{}: ", v.data);
            let mut p = v.firstout.as_ref().map(|arc| Rc::clone(arc));
            while let Some(ref e) = p{
                print!("-->({}, {})", e.borrow().tailvex, e.borrow().headvex);
                let q = e.borrow().tlink.as_ref().map(|arc| Rc::clone(arc));
                p = q;
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod test{
    use super::OLGraph;
    use super::super::Edge;
    #[test]
    fn test_olgraph(){
        let mut g = OLGraph::<char, usize>::new();
        let v = ['a', 'b', 'c', 'd'];
        let e = [('a','b', 1), ('a', 'c', 1), ('c', 'a', 1), ('c', 'd', 1), ('d', 'a', 1), ('d', 'b', 1)];
        let edges : Vec<_> = e.iter().map(|&(from, to, _)| Edge::<char, usize>::new_with_ends(from, to)).collect();
    
        g.create_dg(v.to_vec(), edges);
        let mut visited = vec![false; 4];
        g.print();
        g.dfs(0, &mut visited);
    }
}