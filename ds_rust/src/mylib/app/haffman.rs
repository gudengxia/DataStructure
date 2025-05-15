use num_traits::{Zero, Bounded};
use crate::mylib::app::haffman;

#[derive(Debug, Clone)]
struct HaffmanNode<W: Copy + PartialOrd + num_traits::Zero + num_traits::Bounded + std::fmt::Display + std::fmt::Debug> {
    pub weight: W,
    pub parent: isize,
    pub ld: isize,
    pub rd: isize
}

impl<W: PartialOrd + num_traits::Zero + num_traits::Bounded + Copy + std::fmt::Display + std::fmt::Debug> HaffmanNode<W> {
    pub fn new() -> Self {
        Self { weight: W::zero(), parent: 0, ld: 0, rd: 0 }
    }
}


pub struct HaffmanTree<T: Copy + std::fmt::Display, W: Copy + PartialOrd + num_traits::Zero + num_traits::Bounded + std::fmt::Display + std::fmt::Debug> {
    data: Vec<T>,
    weight: Vec<W>,
    haffmantable: Vec<HaffmanNode<W>>,
    n: usize
}

impl<T: std::fmt::Display + Copy, W:  Copy + PartialOrd + num_traits::Zero + num_traits::Bounded + std::fmt::Display +  std::fmt::Debug> HaffmanTree<T, W> {
    pub fn new() -> Self {
        Self{data: Vec::new(), weight: Vec::new(), haffmantable: Vec::new(), n: 0}
    }

    pub fn get_data(& mut self, ch: Vec<T>, w: Vec<W>){
        self.n = ch.len();
        for i in 0..self.n {
            self.data.push(ch[i]);
            self.weight.push(w[i]); //self.weight[i] = w[i];
        }
        
    }

    pub fn create_haffman_tree(&mut self){
        for i in 0..self.n {
            let mut haffman = HaffmanNode::new();
            haffman.weight = self.weight[i];
            haffman.parent = -1;
            haffman.ld = -1;
            haffman.rd = -1;
            self.haffmantable.push(haffman);
        }

        for i in self.n..(2*self.n-1){
            //1 find two min weights in haffmantable
            let mut min1 = W::max_value();
            let mut min2 = W::max_value();
            let mut index1 = -1isize;
            let mut index2 = -1isize;
            for j in 0..i{
                if self.haffmantable[j].parent == -1 && self.haffmantable[j].weight < min1{
                    index2 = index1;
                    index1 = j as isize;
                    min2 = min1;
                    min1 = self.haffmantable[j].weight;
                }else if self.haffmantable[j].parent == -1 && self.haffmantable[j].weight < min2{
                    index2 = j as isize;
                    min2 = self.haffmantable[j].weight;
                }
            }
            if index1 > index2{
                let tmp = index1;
                index1 = index2;
                index2 = tmp;   
            }
            
            //2 create new node
            let mut haffman = HaffmanNode::new();
            haffman.weight = min1 + min2;
            haffman.parent = -1;
            haffman.ld = index1;
            haffman.rd = index2;
            self.haffmantable[index1 as usize].parent = i as isize;
            self.haffmantable[index2 as usize].parent = i as isize;
            self.haffmantable.push(haffman);
        }
    }

    pub fn encode(&mut self){
        for i in 0..self.n{
            let mut s = String::new();
            let mut j = i as isize;
            while self.haffmantable[j as usize].parent != -1{
                let parent = self.haffmantable[j as usize].parent;
                if self.haffmantable[parent as usize].ld == j{
                    s.push('0');
                }else{
                    s.push('1');
                }
                j = self.haffmantable[j as usize].parent;
            }
            println!("{}: {:?}", self.data[i], s.chars().rev());
        }
    }

    pub fn decode(&mut self, s: &String)-> Result<Vec<T>, &str>{
        let mut r = Vec::<T>::new();
        let root = (2*self.n-2) as isize;   
        let mut i = 0usize;
        let mut p = root;

        while i < s.len() {
            if s.chars().nth(i).unwrap() == '0'{
                p = self.haffmantable[p as usize].ld;
            }else if s.chars().nth(i).unwrap() == '1' {
                p = self.haffmantable[p as usize].rd;
            }else{
                return Err("decode error: code must be 0 or 1");
            }
            if self.haffmantable[p as usize].ld == -1 && self.haffmantable[p as usize].rd == -1{
                r.push(self.data[p as usize]);
                p = root;
            }
            i += 1;
        }
        if p == root{
            return Ok(r);
        }
        return Err("decode error");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_haffman(){
        let w = [5, 29, 7, 8, 14, 23, 3, 11];
        let ch = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let mut tree = HaffmanTree::<char, usize>::new();
        tree.get_data(ch.to_vec(), w.to_vec());
        tree.create_haffman_tree();
        tree.encode();
        let decode = tree.decode(&String::from("11100100110"));
        match decode {
            Ok(s) => println!("{:?}", s),
            Err(e) => println!("{}", e)
        }
    }
}