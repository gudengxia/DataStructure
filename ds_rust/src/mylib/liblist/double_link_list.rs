use core::cell::RefCell;
use core::cmp::PartialEq;
use core::option::Option::{None, Some};
use core::result::Result::{self, Err, Ok};
use crate::mylib::liblist::list::List;
use std::rc::Rc;
// The cyclic double linked list with header node 
struct DLinkNode<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    data: T,
    pre: Option<Rc<RefCell<DLinkNode<T>>>>,
    next: Option<Rc<RefCell<DLinkNode<T>>>>
}

impl<T: std::default::Default + Copy + std::fmt::Display + PartialEq> DLinkNode<T>{
    fn new() -> DLinkNode<T> { DLinkNode { data: T::default(), pre: None, next: None } }
}
pub struct DLinkList<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    h: Option<Rc<RefCell<DLinkNode<T>>>>,
    //rear: Option<Rc<RefCell<DLinkNode<T>>>>
    //len: usize
}

impl<T> List<T> for DLinkList<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> DLinkList<T> {
        let node = Rc::new(RefCell::new(DLinkNode::<T>::new()));
        node.borrow_mut().next = Some(Rc::clone(&node));
        node.borrow_mut().pre = Some(Rc::clone(&node)); 
        DLinkList { h: Some(Rc::clone(&node))}
    }

    fn is_empty(&self) -> bool {
        let h = Rc::clone(&self.h.as_ref().unwrap());
        if Rc::ptr_eq(self.h.as_ref().unwrap(), &h){
            return true;
        }
        return false;
    }

    fn length(&self) -> usize {
        let mut cnt = 0usize;
        let mut p = self.h.as_ref().unwrap().borrow().next.as_ref().map(|node| Rc::clone(node));
        let h = Rc::clone(&self.h.as_ref().unwrap());
        while !Rc::ptr_eq(p.as_ref().unwrap(), &h){
            cnt += 1;
            let q = p.as_ref().unwrap().borrow().next.as_ref().map(|next| Rc::clone(next));    
            p = q;
        }
        cnt
    }

    fn clear(&mut self){
        let h = self.h.as_ref().map(|node| Rc::clone(node));
        let first = h.as_ref().unwrap().borrow().next.as_ref().map(|next| Rc::clone(next));
        if !Rc::ptr_eq(h.as_ref().unwrap(), first.as_ref().unwrap()){// not null 
            let _ = h.as_ref().unwrap().borrow_mut().next.take();
            let _ = h.as_ref().unwrap().borrow_mut().pre.take();
            h.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(h.as_ref().unwrap()));
            h.as_ref().unwrap().borrow_mut().pre = Some(Rc::clone(h.as_ref().unwrap()));
        } 
    }


    fn get(&self, pos: usize) -> Result<T, &'static str> {
        let mut i = 1 as usize;
        let mut p = self.h.as_ref().unwrap().borrow().next.as_ref().map(|node| Rc::clone(node));
        let h = self.h.as_ref().map(|node| Rc::clone(node));
        while i < pos && !Rc::ptr_eq(p.as_ref().unwrap(), h.as_ref().unwrap()){
            let q = p.as_ref().unwrap().borrow().next.as_ref().map(|next| Rc::clone(next));
            p = q;
            i += 1;
        }
        if i > pos || Rc::ptr_eq(p.as_ref().unwrap(), h.as_ref().unwrap()){
            return Err("get: position error");
        }
        Ok(p.unwrap().borrow().data)
    }

    fn set(&mut self, pos: usize, e: T) -> Result<(), &'static str> {
        let mut i = 1 as usize;
        let mut p = Rc::clone(self.h.as_ref().unwrap().borrow().next.as_ref().unwrap());
        let h = Rc::clone(&self.h.as_ref().unwrap());
        while i < pos && !Rc::ptr_eq(&p, &h){
            let q = Rc::clone(p.borrow().next.as_ref().unwrap());
            p = q;
            i += 1;
        }
        if i > pos || Rc::ptr_eq(&p, &h){
            return Err("set: position error");
        }
        p.borrow_mut().data = e;
        Ok(())
    }

    fn insert(&mut self, pos: usize, e: T) -> Result<(), &'static str> {
        let mut i = 1usize;
        //1. find the pos-th node, while verify if pos is valid 
        let h = Rc::clone(self.h.as_ref().unwrap());
        let mut p = Rc::clone(h.borrow().next.as_ref().unwrap()); 
        // assume p points to the first node
        while i < pos && !Rc::ptr_eq(&p, &h){
            let t = Rc::clone(p.borrow().next.as_ref().unwrap());
            i += 1;
            p = t;    
        }
        
        //i > pos: pos < 1, p == h: pos > n
        //if i > pos || Rc::ptr_eq(&p, &h)
        if i != pos
        {
            return Err("insert: position error.")
        }

        // 2. insert the new node r between p and q
        let q = p;
        p = Rc::clone(q.borrow().pre.as_ref().unwrap());
        let r = Rc::new(RefCell::new(DLinkNode::<T>::new()));
        r.borrow_mut().data = e;
        r.borrow_mut().next = Some(Rc::clone(&q)); //r->next = q
        p.borrow_mut().next = Some(Rc::clone(&r)); //p->next = r
        r.borrow_mut().pre = Some(Rc::clone(&p)); //r->pre = p
        q.borrow_mut().pre = Some(Rc::clone(&r)); //q->pre = r
        Ok(())   
    }

    fn delete(&mut self, pos: usize) -> Result<T, &'static str> { 
        let mut i = 1usize;
        //1. find the pos-th node, while verify if pos is valid 
        let h = Rc::clone(self.h.as_ref().unwrap());
        let mut p = Rc::clone(h.borrow().next.as_ref().unwrap()); 
        // assume p points to the first node
        while i < pos && !Rc::ptr_eq(&p, &h){
            let t = Rc::clone(p.borrow().next.as_ref().unwrap());
            i += 1;
            p = t;    
        }

        //i > pos: pos < 1, p == h: pos > n
        if i > pos || Rc::ptr_eq(&p, &h)
        {
            return Err("delete error.");
        }
        //delete r between p and q
        let r = p;
        p = Rc::clone(r.borrow().pre.as_ref().unwrap());
        let q = Rc::clone(r.borrow().next.as_ref().unwrap());
        let e = r.borrow().data;
        p.borrow_mut().next = r.borrow_mut().next.take();
        q.borrow_mut().pre = r.borrow_mut().pre.take(); 
        Ok(e)
    }

    fn traverse(&self) {
        let mut p = Rc::clone(self.h.as_ref().unwrap().borrow().next.as_ref().unwrap());
        let h = Rc::clone(self.h.as_ref().unwrap());
        println!("+++++++++++++++++++++++++++++++++++");
        while !Rc::ptr_eq(&p, &h){
            print!("-->{}", p.borrow().data);
            let t = Rc::clone(p.borrow().next.as_ref().unwrap()); 
            p = t;
        }
        println!("");
        
        p = Rc::clone(self.h.as_ref().unwrap().borrow().pre.as_ref().unwrap());
        while !Rc::ptr_eq(&p, &h){
            print!("{}<--", p.borrow().data);
            let t = Rc::clone(p.borrow().pre.as_ref().unwrap()); 
            p = t;
        }
        println!("");
        println!("+++++++++++++++++++++++++++++++++++");
    }
}

#[cfg(test)]
mod test{
    use super::DLinkList; 
    use crate::mylib::liblist::list::List;  
    #[test]
    fn test_double_link_list()
    {
        let mut l = DLinkList::<i32>::new();
        let mut pos;
        let mut e;
        for i in 1..=3{
            let r = l.insert(i, i as i32);
            match r {
                Ok(()) => println!("insert {} at pos {} ok", i, i),
                Err(err) => println!("insert {} at pos {} error:{}", i, i, err)
            }
        }    
        l.traverse();

        pos = 1;
        e = -1;  
        let mut r = l.insert(1, -1);
        match r {
            Ok(()) => println!("insert {} at {} ok", e, pos),
            Err(err) => println!("insert {} at {} error:{}", e , pos, err)
        }
        l.traverse();

        pos = 7;
        e = -2;
        r = l.insert(pos, e);
        match r {
            Ok(()) => println!("insert {} at {} ok", e, pos),
            Err(err) =>println!("insert {} at {} error:{}", e , pos, err)
        }
        l.traverse();
        println!("len={}", l.length());


        pos = 3;
        e = -5;
        r = l.insert(pos, e);
        match r {
            Ok(()) => println!("insert {} at {} ok", e, pos),
            Err(err) => println!("insert {} at {} error:{}", e , pos, err)
        }
        l.traverse();
        let n = l.length();
        for i in 0..=(n+1){
            match l.get(i){
                Ok(e) =>{
                    if e > 0{
                        let _ = l.set(i, e*100);
                    }
                }
                Err(err) => println!("get at pos {} error:{}", i, err)
            }
        }
        l.traverse();
        pos = 3;
        let mut ans = l.delete(pos);
        match ans {
            Ok(e) => println!("delete {} at {}", e, pos),
            Err(err) => println!("delete at pos {} error:{}", pos, err)
        }
        l.traverse();

        pos = 1; 
        ans = l.delete(pos);
        match ans {
            Ok(e) => println!("delete {} at {}", e, pos),
            Err(err) => println!("delete at pos {} error:{}", pos, err)
        }
        l.traverse();

        pos = 4;
        ans = l.delete(pos);
        match ans {
            Ok(e) => println!("delete {} at {}", e, pos),
            Err(err) => println!("delete at pos {} error:{}", pos, err)
        }
        l.traverse();

        pos = 4;
        ans = l.delete(pos);
        match ans {
            Ok(e) => println!("delete {} at {}", pos, e),
            Err(err) => println!("delete at pos {} error:{}", pos, err)
        }
        l.traverse();

        l.clear();
        println!("{}", l.length());
    }
}