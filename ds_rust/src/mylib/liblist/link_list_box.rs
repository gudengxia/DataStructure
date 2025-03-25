use crate::mylib::liblist::list::List;
use core::{cmp::PartialEq};

#[derive(Debug)]
pub struct BLinkListNode<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    data: T,
    next: Option<Box<BLinkListNode<T>>>,
}

impl<T> BLinkListNode<T>  where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> BLinkListNode<T> {
        BLinkListNode { data: T::default(), next: None}
    }
}

pub struct BLinkList<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    head: Option<Box<BLinkListNode<T>>>,
    len: usize,
}


impl<T> List<T> for BLinkList<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> BLinkList<T> {
        let p = Box::new(BLinkListNode::new());
        BLinkList {
            head: Some(p),
            len: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn get(&self, pos: usize) -> Result<T, &'static str> {
        if pos < 1 || pos > self.len {
            return Err("get: position error");
        }
        let mut p = self.head.as_ref().unwrap();
        let mut cnt = 0;
        while cnt < pos {
            p = p.next.as_ref().unwrap();
            cnt += 1;
        }
        Ok(p.data)
    }

    fn length(&self) -> usize {
        self.len
    }
    fn set(&mut self, pos: usize, e: T) -> Result<(), &'static str> {
        if pos < 1 || pos > self.len {
            return Err("get: position error");
        }
        let mut p = self.head.as_mut().unwrap();
        let mut cnt = 0;
        while cnt < pos {
            p = p.next.as_mut().unwrap();
            cnt += 1;
        }
        p.data = e;
        Ok(())
    }

    fn clear(&mut self) {
        let p = self.head.as_mut().unwrap().next.take();
        drop(p);
        self.len = 0;
    }

    fn insert(&mut self, pos: usize, e: T) -> Result<(), &'static str> {
        if pos < 1 || pos > self.len + 1 {
            return Err("get: position error");
        }
        let mut p = self.head.as_mut().unwrap();
        let mut cnt = 0;
        while cnt < pos - 1 {
            p = p.next.as_mut().unwrap();
            cnt += 1;
        }
        let mut q = Box::new(BLinkListNode::new());
        q.data = e;
        q.next = p.next.take();
        p.next = Some(q);
        self.len += 1;
        Ok(())
    }

    fn delete(&mut self, pos: usize) -> Result<T, &'static str> {
        if pos < 1 || pos > self.len{
            return Err("delete: position error");
        }
        let mut p = self.head.as_mut().unwrap();
        let mut cnt = 0;
        while cnt < pos - 1 {
            p = p.next.as_mut().unwrap();
            cnt += 1;
        }

        let q = p.next.as_mut().unwrap();
        let e = q.data;
        p.next = q.next.take();
        self.len -= 1;        
        Ok(e)
    }

    fn traverse(&self) {
        let mut p = self.head.as_ref().unwrap();
        let mut cnt = 0;
        while p.next.is_some() {
            p = p.next.as_ref().unwrap();
            print!("-->{}", p.data);
            cnt += 1;
        }
        println!("\nThere are {} elements.\n", cnt);
    }
}

// test SqList functions in test mod 
#[cfg(test)]
mod test{
    use super::BLinkList; 
    use crate::mylib::liblist::list::List;  
    #[test]
    fn test_blist(){
        let mut l:BLinkList<i32> = BLinkList::<i32>::new();
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

        let ans = l.delete(3);
        match ans {
            Ok(e) => println!("delete {} ok, e = {}", 3, e),
            Err(e) => println!("delete {} error:{}", 3, e)
        }
        l.traverse();

        let ans = l.delete(1);
        match ans {
            Ok(e) => println!("delete {} ok, e = {}", 3, e),
            Err(e) => println!("delete {} error:{}", 3, e)
        }
        l.traverse();
    }
}
