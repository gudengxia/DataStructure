use std::rc::Rc;
use core::cell::RefCell;
struct BTNode<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    data: T,
    lchild: Option<Rc<RefCell<BTNode<T>>>>,
    rchild: Option<Rc<RefCell<BTNode<T>>>>
}

impl<T:  std::default::Default + Copy + std::fmt::Display + PartialEq> BTNode<T> {
    pub fn new() -> BTNode<T> {
        BTNode {
            data: T::default(),
            lchild: None,
            rchild: None
        }
    }
}

pub struct BiTree<T:  std::default::Default + Copy + std::fmt::Display + PartialEq>{
    root: Option<Rc<RefCell<BTNode<T>>>>
}

impl <T:  std::default::Default + Copy + std::fmt::Display + PartialEq> BiTree<T> {
    pub fn new() -> BiTree<T> {
        BiTree { root: None }
    }

    pub fn build_tree(&mut self, val: Vec<Option<T>>) -> Result<(), &'static str> {
        let mut s = Vec::<Rc<RefCell<BTNode<T>>>>::new();
        if val[0].is_none(){
            return Err("invalid input");
        }
        println!("create node {}", val[0].unwrap());
        let e = val[0].unwrap();
        let mut node = Rc::new(RefCell::new(BTNode::new()));
        node.borrow_mut().data = e;
        s.push(Rc::clone(&node));
        let mut p = Rc::clone(&node);
        self.root = Some(node);

        let mut backwards = false; 
        //0: go next or backwards from left, 1: come back from right
        let mut i = 1usize;
        let n = val.len();
        while !s.is_empty() {
            //1. the left child is not null, and go the left until the left child is null
            while !backwards && i < n && val[i].is_some() {
                println!("create node {}", val[i].unwrap());
                node = Rc::new(RefCell::new(BTNode::new()));
                node.borrow_mut().data = val[i].unwrap();
                i += 1;
                p.borrow_mut().lchild = Some(Rc::clone(&node));
                s.push(Rc::clone(&node));
                p = Rc::clone(&node);
            }
            //2. the left child is null, pop the top of the stack,and go its right branch
            backwards = false;
            // imply i < n and the stack is not empty
            // create the left subtree
            i += 1; //leap the construction of left subtree
            p = s.pop().unwrap();
            // create rchild
            if let Some(e) = val[i]{
                println!("create node {}", e);
                node = Rc::new(RefCell::new(BTNode::new()));
                node.borrow_mut().data = e;
                i += 1;
                p.borrow_mut().rchild = Some(Rc::clone(&node));
                s.push(Rc::clone(&node));
                p = Rc::clone(&node);
            }else{
                backwards = true;
            }
        }
        if !s.is_empty(){
            return Err("invalid input");
        }
        Ok(())
    }

    pub fn preorder(&self)
    {
        let mut s = Vec::<Rc<RefCell<BTNode<T>>>>::new();

        if self.root.is_none(){
            return;
        }
        let mut p = Rc::clone(self.root.as_ref().unwrap());
        println!("Visit {}", p.borrow().data);
        s.push(p);
        
        let mut backwards = false;
        while let Some(q) = s.clone().last(){
            if !backwards && q.borrow().lchild.is_some() {
                p = Rc::clone(q.borrow().lchild.as_ref().unwrap());
                println!("Visit {}", p.borrow().data);
                s.push(Rc::clone(&p));
            }else{
                s.pop();
                // if rchild is not null
                if let Some(rchild) = &q.borrow().rchild{
                    p = Rc::clone(&rchild);
                    println!("Visit {}", p.borrow().data);
                    s.push(Rc::clone(&p));
                    backwards = false;
                }else{
                    backwards = true;
                }
            }
        }
    }

    pub fn inorder(&self)
    {
        let mut s = Vec::<Rc<RefCell<BTNode<T>>>>::new();

        if self.root.is_none(){
            return;
        }
        let mut p = Rc::clone(self.root.as_ref().unwrap());
        s.push(p);
        
        let mut backwards = false;
        while let Some(q) = s.clone().last(){
            if !backwards && q.borrow().lchild.is_some() {
                p = Rc::clone(q.borrow().lchild.as_ref().unwrap());
                //println!("Visit {}", p.borrow().data);
                s.push(Rc::clone(&p));
            }else{
                s.pop();
                println!("Visit {}", q.borrow().data);
                // if rchild is not null
                if let Some(rchild) = &q.borrow().rchild{
                    p = Rc::clone(&rchild);
                    //println!("Visit {}", p.borrow().data);
                    s.push(Rc::clone(&p));
                    backwards = false;
                }else{
                    backwards = true;
                }
            }
        }
    }

    pub fn postorder(&self)
    {
        #[derive(Clone)]
        struct SElem<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
            e: Rc<RefCell<BTNode<T>>>,
            dir: usize
        }
        let mut s = Vec::<SElem<T>>::new();

        if self.root.is_none(){
            return;
        }
        let mut p = Rc::clone(self.root.as_ref().unwrap());
        s.push(SElem{e: p, dir: 0usize});
        
        let mut backwards = false;
        while let Some(q) = s.clone().last(){
            if !backwards && q.e.borrow().lchild.is_some() && q.dir == 0{
                p = Rc::clone(q.e.borrow().lchild.as_ref().unwrap());
                //println!("Visit {}", p.borrow().data);
                s.push(SElem{e: Rc::clone(&p), dir: 0});
            }else{
                s.pop();
                if q.dir == 0{
                    let e = SElem{e: Rc::clone(&q.e), dir: 1};
                    s.push(e);
                    if let Some(rchild) = &q.e.borrow().rchild{
                        p = Rc::clone(&rchild);
                        //println!("Visit {}", p.borrow().data);
                        s.push(SElem{e:Rc::clone(&p), dir: 0});
                        backwards = false;
                    }else{
                        backwards = true;
                    }
                }
                else{
                    println!("Visit {}", q.e.borrow().data);
                }      
                // if rchild is not null
            }
        }
    }
    
}

#[cfg(test)]
mod test{
    use super::BiTree;
    #[test]
    fn test_binarytree_traverse(){
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
}