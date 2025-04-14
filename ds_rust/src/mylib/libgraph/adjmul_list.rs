use super::Edge;
use std::rc::Rc;
use std::cell::RefCell;
struct EBox<E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>{
    ivex: usize,
    jvex: usize,
    ilink: Option<Rc<RefCell<EBox<E>>>>,
    jlink: Option<Rc<RefCell<EBox<E>>>>,
    info: Option<E>
}

impl<E: std::default::Default + Copy + std::fmt::Display + PartialEq+ PartialOrd> EBox<E>{
    fn new()->EBox<E>{
        EBox{
            ivex: 0,
            jvex: 0,
            ilink: None,
            jlink: None,
            info: None
        }
    }
}

struct VexBox<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>{
    data: T,
    firstedge: Option<Rc<RefCell<EBox<E>>>>,
}
impl<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>VexBox<T, E>{
    fn new()->VexBox<T, E>{
        VexBox{
            data: T::default(),
            firstedge: None,
        }
    }
}


pub struct AMLGraph<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>{
    xlist: Vec<VexBox<T, E>>,
    vexnum: usize,
    edgenum: usize
}


impl<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq+ PartialOrd> AMLGraph<T, E>{
    pub fn new()->AMLGraph<T, E>{
        AMLGraph { xlist: Vec::new(), vexnum: 0, edgenum: 0}
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
    pub fn create_udg(&mut self, vex: Vec<T>, edge: Vec<Edge<T, E>>){
        self.vexnum = vex.len();
        for v in vex{
            let mut vertex = VexBox::new();
            vertex.data = v;
            self.xlist.push(vertex);
        }

        self.edgenum = edge.len();
        for e in edge{
            let i = self.locate_vex(e.tailvex).expect("err: node does not exist");
            let j = self.locate_vex(e.headvex).expect("err: node does not exist");
            let edge = Rc::new(RefCell::new(EBox::<E>::new()));
            edge.borrow_mut().ivex = i;
            edge.borrow_mut().jvex = j;
            edge.borrow_mut().ilink = self.xlist[i].firstedge.take();
            edge.borrow_mut().jlink = self.xlist[j].firstedge.take();
            edge.borrow_mut().info = e.info;
            self.xlist[j].firstedge = Some(Rc::clone(&edge)); 
            self.xlist[i].firstedge = Some(Rc::clone(&edge)); 
        }
    }

    pub fn firstarc(&self, i: usize)->Option<Rc<RefCell<EBox<E>>>>{
        match &self.xlist[i].firstedge{
            Some(e) => {
                return Some(Rc::clone(e));
            }
            None => return  None
        }
    }
    pub fn nextarc(&self, cur: Option<Rc<RefCell<EBox<E>>>>) -> Option<Rc<RefCell<EBox<E>>>>{
        if let Some(e) = cur{
            if let Some(arc) = &e.borrow().ilink{
                return Some(Rc::clone(&arc))
            }
            if let Some(arc) = &e.borrow().jlink{
                return Some(Rc::clone(&arc))
            }
        }
        None
    }

    pub fn dfs(&mut self, i: usize, visited: &mut  Vec<bool>){
        let mut s = Vec::<Rc<RefCell<EBox<E>>>>::new();
        visited[i] = true;
        println!("Visit {}", self.xlist[i].data);
        
        if let Some(e) = &self.xlist[i].firstedge{
            s.push(Rc::clone(&e));
        }

        while let Some(e) = s.clone().last(){
            let i = e.borrow().ivex;
            let mut j = e.borrow().jvex;
            /**************important, we don't know we get the edge from i or j*/
            if !visited[i]{//we get the edge from j
                j = i;
            }
            /**************important, we don't know we get the edge from i or j*/

            if !visited[j]{
                visited[j] = true;
                println!("Visit {}", self.xlist[j].data);
                if let Some(arc) = &self.xlist[j].firstedge{
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
        for index in 0..self.vexnum{
            print!("{}: ", self.xlist[index].data);
            
            let mut p = self.xlist[index].firstedge.as_ref().map(|arc| Rc::clone(arc));
            while let Some(ref e) = p{
                print!("-->({}, {})", e.borrow().ivex, e.borrow().jvex);
                if e.borrow().ivex == index{// follow ilink 
                    let q = e.borrow().ilink.as_ref().map(|arc| Rc::clone(arc));
                    p = q;
                }else{//follow jlink
                    let q = e.borrow().jlink.as_ref().map(|arc| Rc::clone(arc));
                    p = q;
                }
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod test{
    use super::AMLGraph;
    use super::super::Edge;
    #[test]
    fn test_aml(){
        let mut g = AMLGraph::<char, usize>::new();
        let v = ['a', 'b', 'c', 'd', 'e'];
        let e = [('a','b', 13), ('a', 'd', 8), ('a', 'e', 7), ('b', 'c', 1), ('b', 'd', 4), ('c', 'd', 2), ('d', 'e', 3)];
        let edges : Vec<_> = e.iter().map(|&(from, to, weight)| Edge::<char, usize>::new_with_ends(from, to)).collect();
    
        g.create_udg(v.to_vec(), edges);
        let mut visited = vec![false; 5];
        g.print();
        g.dfs(0, &mut visited);
    }
}