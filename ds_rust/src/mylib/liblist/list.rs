pub trait List<T> where T: std::default::Default + Copy + std::fmt::Display + PartialEq{
    fn new() -> Self;
    fn is_empty(&self) -> bool;   
    fn length(&self) -> usize;
    fn clear(&mut self);
    fn get(&self, pos: usize) -> Result<T, &'static str>;
    fn set(&mut self, pos: usize, e: T) -> Result<(), &'static str>;
    fn insert(&mut self, pos: usize, e: T) -> Result<(), &'static str>;
    fn delete(&mut self, pos: usize) -> Result<T, &'static str>;
    fn traverse(&self);
}
