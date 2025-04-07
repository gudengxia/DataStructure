use core::result::Result;
use std::default::Default;

const MAXTWSSIZE: usize = 100;
pub struct Tws<T>  where T:Default + Copy + std::fmt::Display + PartialEq{
    tws: Vec<T>,
    top1: usize,
    top2: usize
}

impl<T: Default + Copy + std::fmt::Display + PartialEq> Tws<T>{
    fn new() -> Tws<T>{
        Tws{tws: vec![T::default(); MAXTWSSIZE], 
        top1: 0usize,
        top2: MAXTWSSIZE-1}
    }

    fn is_empty(&self, index: bool) -> bool{
        if (self.top1 == 0 && index) || (self.top2 == MAXTWSSIZE-1 && !index){
            return true;
        }
        else{
            return false;
        }
    }

    fn is_full(&self) ->  bool{
        if self.top1 == self.top2{
            return true;
        }
        else{
            return false;
        }
    }

    fn top(&self, index: bool) ->Result<T, &'static str>{
        if (index && self.is_empty(true)) || (!index && self.is_empty(false)){
            return Err("pop error: defined stack is empty");
        }
        if index{
            let e = self.tws[self.top1-1];
            Ok(e)
        }
        else{
            let e = self.tws[self.top2+1];
            Ok(e)
        }
    }

    fn push(&mut self, index: bool, e: T)->Result<(), &'static str>{
        if self.is_full(){
            return Err("push error: tws is full.");
        }

        if index{
            self.tws[self.top1] = e;
            self.top1 += 1;
        }else{
            self.tws[self.top2] = e;
            self.top2 -= 1;
        }
        Ok(())
    }

    fn pop(&mut self, index: bool) ->Result<T, &'static str>{
        if  (index && self.is_empty(true)) || (!index && self.is_empty(false)){
            return Err("pop error: defined stack is empty");
        }
        if index{
            self.top1 -= 1;
            let e = self.tws[self.top1];
            Ok(e)
        }
        else{
            self.top2 += 1;
            let e = self.tws[self.top2];
            Ok(e)
        }  
    }

    fn clear(&mut self){
        self.top1 = 0usize;
        self.top2 = MAXTWSSIZE-1;
    }
}


pub struct Tws2Q<T>  where T:Default + Copy + std::fmt::Display + PartialEq{
    tws: Tws<T>
}

impl<T: Default + Copy + std::fmt::Display + PartialEq> Tws2Q<T>{
    fn new() -> Tws2Q<T>{
        Tws2Q { tws: Tws::new() }
    }

    fn is_empty(&self) -> bool{
        if self.tws.is_empty(true) && self.tws.is_empty(false){
            return true;
        }
        else{
            return false;
        }
    }

    fn is_full(&self) ->  bool{
        if self.tws.is_full(){
            return true;
        }
        else{
            return false;
        }
    }

    fn gethead(&mut self)->Result<T, &'static str>{
        if self.tws.is_empty(true) && self.tws.is_empty(false){
            return Err("gethead error: the queue is empty")
        }

        //let mut ans= self.tws.top(false);
        let mut e = T::default();
        while self.tws.is_empty(true){
            e = self.tws.pop(true).unwrap();
            let _ = self.tws.push(false, e);
            //ans = Ok(e);
        }
        Ok(e)
    }

    fn enqueue(&mut self, e: T)->Result<(), &'static str>{
        if self.tws.is_full(){
            return Err("enqueue error: the queue is full")
        }

        let ans = self.tws.push(true, e);
        ans
    }

    fn dequeue(&mut self)->Result<T, &'static str>{
        if self.tws.is_empty(true) && self.tws.is_empty(false){
            return Err("gethead error: the queue is empty")
        }

        //let mut ans = self.tws.pop(false);
        while let Ok(e) = self.tws.pop(true){
            let _ = self.tws.push(false, e);
            //ans = Ok(e);
        }
        let ans = self.tws.pop(false);
        ans
    }

    pub fn clear(&mut self){
        self.tws.clear();
    }
}

#[cfg(test)]
mod test{
    use super::Tws2Q;

    #[test]
    fn test_tws(){
        let mut q = Tws2Q::<usize>::new();

        for i in 0..5{
            let _ = q.enqueue(i);
        }
        assert_eq!(q.gethead().unwrap(), 0);

        for i in 5..10{
            let _= q.enqueue(i);
        }
        
        let mut i = 0usize;
        while let Ok(e) = q.dequeue(){
            assert_eq!(e, i);
            i += 1;
        }

        assert_eq!(q.is_empty(), true);
    }
}