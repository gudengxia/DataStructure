#[derive(Debug)]
pub struct Edge<T: std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd>{
    tailvex: T,
    headvex: T,
    info: Option<E>
}

impl <T:std::default::Default + Copy + std::fmt::Display + PartialEq, E: std::default::Default + Copy + std::fmt::Display + PartialEq + PartialOrd> Edge<T, E>{
    pub fn new() ->Edge<T, E>{
        Edge{tailvex: T::default(), headvex: T::default(), info: None}
    }
    pub fn new_with_ends(t: T, h: T)->Edge<T, E>{
        Edge{tailvex: t, headvex: h, info: None}
    }
    pub fn new_with_ends_weight(t: T, h: T, w: E)->Edge<T, E>{
        Edge{tailvex: t, headvex: h, info: Some(w)}
    }
}

pub mod orthogonal_list;
pub mod adjmul_list;
pub mod adjmatrix;