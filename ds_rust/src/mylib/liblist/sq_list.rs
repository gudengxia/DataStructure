use core::result::Result;
use std::default::Default;
use crate::mylib::liblist::list;
const MAXSIZE: usize = 100;
pub struct SqList<T>  where T:Default + Copy + std::fmt::Display + PartialEq{
    elem: Vec<T>,
    len: usize
}

impl<T> list::List<T> for SqList<T> where T:Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> SqList<T> 
    {   
        SqList { 
            elem: vec![T::default(); MAXSIZE], 
            len: 0}
    }

    fn is_empty(&self) -> bool{
        self.len == 0
    }   

    fn length(&self) -> usize{
        self.len
    }

    fn clear(&mut self){
        self.len = 0;
    }   

    fn get(&self, pos: usize) -> Result<T, &'static str>{   
        if pos < 1 || pos > self.len{
            return Err("position error");
        }   
        Ok(self.elem[pos - 1])
    }

    fn set(&mut self, pos: usize, e: T) -> Result<(), &'static str>{
        if pos < 1 || pos > self.len{
            return Err("position error");
        }   
        self.elem[pos - 1] = e;
        return Ok(());
    }

    fn insert(&mut self, pos: usize, e: T) -> Result<(), &'static str>{
        if pos < 1 || pos > self.len + 1{
            return Err("position error");
        }
        if self.len >= MAXSIZE{
            return Err("overflow");
        }
        for i in (pos..=self.len).rev(){
            self.elem[i] = self.elem[i-1];
        }        
        self.elem[pos - 1] = e;
        self.len += 1;
        Ok(())
    } 

    fn delete(&mut self, pos: usize) -> Result<T, &'static str>{
        if pos < 1 || pos > self.len{
            return Err("position error");
        }
        let e = self.elem[pos - 1];
        for i in pos..self.len{
            self.elem[i-1] = self.elem[i];
        }
        self.len -= 1;
        Ok(e)
    }

    fn traverse(&self){
        for i in 0..self.len-1{
            print!("{},", self.elem[i]);
        }       
        println!("{}", self.elem[self.len-1])
    }
}       

// test SqList functions in test mod 
#[cfg(test)]
mod test{
    use crate::mylib::liblist::list::List;
    use super::SqList;   
    #[test]
    fn test_sqlist(){
        let mut l:SqList<i32> = SqList::new();
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

        ans = l.delete(11);
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
    }
}

