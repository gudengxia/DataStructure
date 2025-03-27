use core::cell::RefCell;
use core::cmp::PartialEq;
use core::result::Result::{self, Err, Ok};
use crate::mylib::liblist::list::List;
use std::rc::Rc;

struct LinkNode<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    data: T,
    next: Option<Rc<RefCell<LinkNode<T>>>>
}

impl<T: std::default::Default + Copy + std::fmt::Display + PartialEq> LinkNode<T>{
    fn new() -> LinkNode<T> { LinkNode { data: T::default(), next: None } }
}
pub struct LinkList<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    h: Option<Rc<RefCell<LinkNode<T>>>>,
    //len: usize
}

impl<T> List<T> for LinkList<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> LinkList<T> { 
        let p = Rc::new(RefCell::new(LinkNode::<T>::new())); 
        
        LinkList { h: Some(p)} 
    }

    fn is_empty(&self) -> bool {
        let p = Rc::clone(self.h.as_ref().unwrap());
        return p.borrow().next.is_none();
    }

    fn length(&self) -> usize {
        let mut cnt = 0usize;
        let mut p = Rc::clone(self.h.as_ref().unwrap());

        while p.borrow().next.is_some(){
            let q = Rc::clone(p.borrow().next.as_ref().unwrap());
            p = q;
            cnt += 1;
        }
        cnt
    }

    fn clear(&mut self) {
        let p = self.h.as_ref().unwrap().borrow_mut().next.take();
    }


    fn get(&self, pos: usize) -> Result<T, &'static str> {
        let mut i = 1 as usize;
        let mut p = self.h.as_ref().unwrap().borrow().next.as_ref().map(|node| Rc::clone(node));

        while i < pos && p.is_some(){
            p = p.unwrap().borrow().next.as_ref().map(|next| Rc::clone(next));
            i += 1;
        }
        if p.is_none() || i != pos{
            return Err("get: position error");
        }
        Ok(p.unwrap().borrow().data)  
    }

    fn set(&mut self, pos: usize, e: T) -> Result<(), &'static str> {
        let mut i = 1 as usize;
        let mut p = self.h.as_ref().unwrap().borrow().next.as_ref().map(|node| Rc::clone(node));

        while i < pos && p.is_some(){
            p = p.unwrap().borrow().next.as_ref().map(|next| Rc::clone(next));
            i += 1;
        }
        if p.is_none() || i != pos{
            return Err("get: position error");
        }
        p.unwrap().borrow_mut().data = e;
        Ok(())  
    }

    fn insert(&mut self, pos: usize, e: T) -> Result<(), &'static str> {
        let mut i = 0 as usize; 
        let mut p = self.h.as_ref().map(|node| Rc::clone(node));

        while p.is_some() && i < pos-1{
            let q = p.as_ref().unwrap().borrow().next.as_ref().map(|next| Rc::clone(next));
            i += 1;
            p = q;
        }
        if p.is_none() || i > pos-1{
            return Err("insert: position error");
        }
        let node = Rc::new(RefCell::new(LinkNode::<T>::new()));
        node.borrow_mut().data = e;
        node.borrow_mut().next = p.as_ref().unwrap().borrow_mut().next.take();
        p.as_ref().unwrap().borrow_mut().next = Some(node);  
        Ok(())   
    }

    fn delete(&mut self, pos: usize) -> Result<T, &'static str> { 
        let mut p = self.h.as_ref().map(|node| Rc::clone(node));
        let mut i = 0 as usize;

        while p.is_some() && i < pos-1{
            let t = p.as_ref().unwrap().borrow().next.as_ref().map(|next| Rc::clone(next));
            p = t;
            i += 1;
        }
        let q = p.as_ref().unwrap().borrow().next.as_ref().map(|next| Rc::clone(next));
        if q.is_none() || i > pos-1{
            return Err("delete: position error");
        }       
        let e = p.as_ref().unwrap().borrow().data;
        p.as_ref().unwrap().borrow_mut().next = q.as_ref().unwrap().borrow_mut().next.take(); 
        Ok(e)
    }

    fn traverse(&self) {
        let mut p = self.h.as_ref().map(|node| Rc::clone(node));

        while let Some(node) = p{
            print!("-->{}", node.borrow().data); 
            p = node.borrow().next.as_ref().map(|next| Rc::clone(next));
        }
        println!("");
    }
}

#[cfg(test)]
mod test{
    use super::LinkList; 
    use crate::mylib::liblist::list::List;  
    #[test]
    fn test_link_list()
    {
        let mut l = LinkList::<i32>::new();
        for i in 1..=3{
            let r = l.insert(i, i as i32);
            match r {
                Ok(()) => println!("insert {} ok", i),
                Err(e) => println!("insert {} error:{}", i, e)
            }
        }    
        l.traverse();
        
        let mut r = l.insert(1, -1);
        match r {
            Ok(()) => println!("insert {} ok", -1),
            Err(e) => println!("insert {} error:{}", -1, e)
        }
        l.traverse();

        r = l.insert(7, -2);
        match r {
            Ok(()) => println!("insert {} ok", -2),
            Err(e) => println!("insert {} error:{}", -2, e)
        }   
        l.traverse();

        r = l.insert(3, -5);
        match r {
            Ok(()) => println!("insert {} ok", -5),
            Err(e) => println!("insert {} error:{}", -5, e)
        } 
        l.traverse();

        let mut ans = l.delete(3);
        match ans {
            Ok(e) => println!("delete {} ok, e = {}", 3, e),
            Err(e) => println!("delete {} error:{}", 3, e)
        }
        l.traverse();

        ans = l.delete(1);
        match ans {
            Ok(e) => println!("delete {} ok, e = {}", 1, e),
            Err(e) => println!("delete {} error:{}", 1, e)
        }
        l.traverse();

        ans = l.delete(4);
        match ans{
            Ok(e) => println!("delete {} ok", e),
            Err(e) => println!("delete error:{}", e)
        }
        l.traverse();

        ans = l.delete(4);
        match ans {
            Ok(e) => println!("delete {} ok", e),
            Err(e) => println!("delete error:{}", e)
        }
        l.traverse();

        l.clear();
        l.traverse();
        println!("{}", l.length());
    }
}