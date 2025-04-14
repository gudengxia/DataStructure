use std::rc::Rc;
use std::cell::RefCell;
pub trait Queue<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> Self;
    fn is_empty(&self) -> bool;   
    fn length(&self) -> usize;
    fn clear(&mut self);
    fn gethead(&self) -> Result<T, &'static str>;
    fn enqueue(&mut self, e: T) -> Result<(), &'static str>;
    fn dequeue(&mut self) -> Result<T, &'static str>;
}

const MAXQUEUESIZE: usize = 100;
pub struct SqQueue<T>  where T:Default + Copy + std::fmt::Display + PartialEq{
    elem: Vec<T>,
    front: usize,
    rear: usize
}

impl<T> Queue<T> for SqQueue<T> where T:Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> SqQueue<T> 
    {   
        SqQueue { 
            elem: vec![T::default(); MAXQUEUESIZE], 
            front: 0,
            rear: 0usize}
    }

    fn is_empty(&self) -> bool{
        self.front == self.rear
    }   

    fn length(&self) -> usize{
        self.front + MAXQUEUESIZE - self.rear
    }

    fn clear(&mut self){
        //self.elem.clear();
        self.front = self.rear;
    }  

    fn gethead(&self) -> Result<T, &'static str>{
        if self.front == self.rear{
            return Err("The Queue is empty.");
        }
        Ok(self.elem[self.front-1])
    }
    fn enqueue(&mut self, e: T) -> Result<(), &'static str>{
        if (self.front + 1) % MAXQUEUESIZE == self.rear{
            return Err("enqueue error: the queue is full")
        }
        self.elem[self.front] = e;
        self.front = (self.front + 1) % MAXQUEUESIZE;
        Ok(())
    }
    fn dequeue(&mut self) -> Result<T, &'static str>{
        if self.front == self.rear{
            return Err("dequeue error: the queue is empty");
        }
        let e = self.elem[self.rear];
        self.rear = (self.rear + 1) % MAXQUEUESIZE;
        Ok(e)
    }
}

struct QLinkNode<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    data: T,
    next: Option<Rc<RefCell<QLinkNode<T>>>>
}

impl<T: std::default::Default + Copy + std::fmt::Display + PartialEq> QLinkNode<T>{
    fn new() -> QLinkNode<T> { QLinkNode { data: T::default(), next: None } }
}
pub struct LinkQueue<T: std::default::Default + Copy + std::fmt::Display + PartialEq>{
    head: Option<Rc<RefCell<QLinkNode<T>>>>,
    rear: Option<Rc<RefCell<QLinkNode<T>>>>
    //len: usize
}

impl<T> Queue<T> for LinkQueue<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> LinkQueue<T> { 
        let p = Rc::new(RefCell::new(QLinkNode::<T>::new())); 
        LinkQueue { head: Some(Rc::clone(&p)), rear: Some(Rc::clone(&p))} 
    }

    fn is_empty(&self) -> bool {
        return Rc::ptr_eq(self.head.as_ref().unwrap(), self.rear.as_ref().unwrap())
    }

    fn length(&self) -> usize {
        let mut cnt = 0usize;
        let mut p = Rc::clone(self.head.as_ref().unwrap());

        while p.borrow().next.is_some(){
            let q = Rc::clone(p.borrow().next.as_ref().unwrap());
            p = q;
            cnt += 1;
        }
        cnt
    }

    fn clear(&mut self) {
        let _ = self.head.as_ref().unwrap().borrow_mut().next.take();
        self.rear = Some(Rc::clone(self.head.as_ref().unwrap()))
    }

    fn gethead(&self) -> Result<T, &'static str>{
        if Rc::ptr_eq(self.head.as_ref().unwrap(), self.rear.as_ref().unwrap()){
            return Err("Error: The queue is empty.");
        }
        Ok(self.head.as_ref().unwrap().borrow().data)
    }
    fn enqueue(&mut self, e: T)->Result<(), &'static str>{
        let p = Rc::new(RefCell::new(QLinkNode::new()));
        p.borrow_mut().data = e;
        //p.borrow_mut().next = self.h.take();
        self.rear.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&p));
        self.rear = Some(p);
        Ok(())
    }
    fn dequeue(&mut self) -> Result<T, &'static str> {
        if Rc::ptr_eq(self.head.as_ref().unwrap(), self.rear.as_ref().unwrap()){
            return Err("dequeue error, the queue is empty");
        }
        //let e = self.head.as_ref().unwrap().borrow().data;
        let p = self.head.as_ref().unwrap().borrow_mut().next.take();
        let e = p.as_ref().unwrap().borrow().data;
        self.head = p;
        // A new method that is different from the one in textbook, which the one removed is next to the head, but an extra step is needed to handle the case when the last element is removed
        Ok(e)
    }
}

#[cfg(test)]
mod test{
    use crate::mylib::liblist::queue::Queue;
    use super::LinkQueue;

    #[test]
    fn test_link_queue(){
        let mut q = LinkQueue::<usize>::new();
        for e in 0..5{
            let _ = q.enqueue(e);
            assert_eq!(q.gethead().unwrap(), 0);
            assert_eq!(q.length(), e+1);
        }

        let mut cnt = 0usize;
        while !q.is_empty()
        {
            match q.dequeue(){
                Ok(e) => assert_eq!(cnt, e),
                Err(e) => panic!("Error: {}", e)
            }
            cnt += 1;
        }
    }
}