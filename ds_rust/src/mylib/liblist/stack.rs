use std::rc::Rc;
use std::cell::RefCell;
pub trait Stack<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> Self;
    fn is_empty(&self) -> bool;   
    fn length(&self) -> usize;
    fn clear(&mut self);
    fn top(&self) -> Result<T, &'static str>;
    fn push(&mut self, e: T)->Result<(), &'static str>;
    fn pop(&mut self) -> Result<T, &'static str>;
}

const MAXSTACKSIZE: usize = 100;
pub struct SqStack<T>  where T:Default + Copy + std::fmt::Display + PartialEq{
    elem: Vec<T>,
    len: usize
}

impl<T> Stack<T> for SqStack<T> where T:Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> SqStack<T> 
    {   
        SqStack { 
            elem: vec![T::default(); MAXSTACKSIZE], 
            len: 0}
    }

    fn is_empty(&self) -> bool{
        self.len == 0
    }   

    fn length(&self) -> usize{
        self.len
    }

    fn clear(&mut self){
        //self.elem.clear();
        self.len = 0;
    }  

    fn top(&self) -> Result<T, &'static str>{
        if self.len == 0{
            return Err("The stack is empty.");
        }
        Ok(self.elem[self.len-1])
    }
    fn push(&mut self, e: T)->Result<(), &'static str>{
        if self.len >= MAXSTACKSIZE{
            return Err("push error: stack is full");
        }
        self.elem.push(e);
        self.len += 1;
        Ok(())
    }
    fn pop(&mut self) -> Result<T, &'static str>{
        if self.len == 0{
            return Err("pop error, the stack is empty");
        }
        self.len -= 1;
        Ok(self.elem[self.len])
    }
}

struct SLinkNode<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    data: T,
    next: Option<Rc<RefCell<SLinkNode<T>>>>
}

impl<T: std::default::Default + Copy + std::fmt::Display + PartialEq> SLinkNode<T>{
    fn new() -> SLinkNode<T> { SLinkNode { data: T::default(), next: None } }
}
pub struct LinkStack<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    h: Option<Rc<RefCell<SLinkNode<T>>>>,
    //len: usize
}

impl<T> Stack<T> for LinkStack<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> LinkStack<T> { 
        //let p = Rc::new(RefCell::new(LinkNode::<T>::new())); 
        
        LinkStack { h: None} 
    }

    fn is_empty(&self) -> bool {
        return self.h.is_none()
    }

    fn length(&self) -> usize {
        let mut cnt = 1usize;
        let mut p = Rc::clone(self.h.as_ref().unwrap());

        while p.borrow().next.is_some(){
            let q = Rc::clone(p.borrow().next.as_ref().unwrap());
            p = q;
            cnt += 1;
        }
        cnt
    }

    fn clear(&mut self) {
        let _ = self.h.take();
    }

    fn top(&self) -> Result<T, &'static str>{
        if self.h.is_none(){
            return Err("The stack is empty.");
        }
        Ok(self.h.as_ref().unwrap().borrow().data)
    }
    fn push(&mut self, e: T) -> Result<(), &'static str>{
        let p = Rc::new(RefCell::new(SLinkNode::new()));
        p.borrow_mut().data = e;
        p.borrow_mut().next = self.h.take();
        self.h = Some(p);
        Ok(())
    }
    fn pop(&mut self) -> Result<T, &'static str>{
        if self.h.is_none(){
            return Err("pop error, the stack is empty");
        }
        let e = self.h.as_ref().unwrap().borrow().data;
        let p = self.h.as_ref().unwrap().borrow_mut().next.take();
        self.h = p;
        Ok(e)
    }
}

#[cfg(test)]
mod test{
    use crate::mylib::liblist::stack::Stack;
    use super::LinkStack;

    #[test]
    fn test_link_stack(){
        let mut s = LinkStack::<usize>::new();
        for e in 0..5{
            let _ = s.push(e);
            assert_eq!(s.top().unwrap(), e);
            assert_eq!(s.length(), e+1);
        }

        let mut cnt = 0usize;
        while !s.is_empty()
        {
            match s.pop(){
                Ok(e) => assert_eq!(4-cnt, e),
                Err(e) => panic!("Error: {}", e)
            }
            cnt += 1;
        }
    }
}