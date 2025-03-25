use core::{cmp::PartialEq, marker::Copy};

#[derive(Debug)]
struct LinkedListNode<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    value: T,
    next: Option<Box<LinkedListNode<T>>>,
}

impl<T> LinkedListNode<T>  where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new(value: T, next: Option<Box<LinkedListNode<T>>>) -> LinkedListNode<T> {
        LinkedListNode { value, next }
    }
}

struct LinkedList<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    head: Option<Box<LinkedListNode<T>>>,
    tail: Option<*mut LinkedListNode<T>>,
}


impl<T> LinkedList<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
        }
    }

    fn prepend(&mut self, value: T) {
        let new_node = Box::new(LinkedListNode::new(value, self.head.take()));
  
        unsafe {
            let raw_node: *mut _ = Box::into_raw(new_node);
  
            if self.tail.is_none() {
                self.tail = Some(raw_node);
            }
  
            self.head = Some(Box::from_raw(raw_node));
        }
    }

    fn append(&mut self, value: T) {
        let new_node = Box::new(LinkedListNode::new(value, None));
    
        unsafe {
            let raw_node: *mut _ = Box::into_raw(new_node);
    
            if self.head.is_none() {
                self.head = Some(Box::from_raw(raw_node));
            } else if let Some(tail) = self.tail {
                (*tail).next = Some(Box::from_raw(raw_node));
            }
    
            self.tail = Some(raw_node);
        }
    }

    fn insert(&mut self, value: T, index: usize) {
        if index == 0 {
            self.prepend(value);
            return;
        }
    
        let mut count = 1;
        let mut current_node = self.head.as_deref_mut();
    
        while let Some(node) = current_node {
            if count == index {
                let new_node = Box::new(LinkedListNode::new(value, node.next.take()));
                node.next = Some(new_node);
                return;
            }
            current_node = node.next.as_deref_mut();
            count += 1;
        }
    
        self.append(value);
    }

    fn find(&self, value: T) -> Option<&LinkedListNode<T>> {
        let mut current = self.head.as_deref();
    
        while let Some(node) = current {
            if node.value == value {
                return Some(node);
            }
            current = node.next.as_deref();
        }
        None
    }

    fn delete(&mut self, value: T) {
        // Handle when the list is empty
        if self.head.is_none() {
            return;
        }
    
        // Handle when the head node needs to be deleted
        if let Some(ref mut head) = self.head {
            if head.value == value {
                self.head = head.next.take();
                if self.head.is_none() {
                    self.tail = None;
                }
                return;
            }
        }
    
        let mut current = self.head.as_deref_mut();
    
        // Handle when a non-head node needs to be deleted
        while let Some(ref mut node) = current {
            if let Some(ref mut next) = node.next {
                if next.value == value {
                    node.next = next.next.take();
                    if node.next.is_none() {
                        self.tail = Some(node);
                    }
                    return;
                }
            }
            current = node.next.as_deref_mut();
        }
    }
}