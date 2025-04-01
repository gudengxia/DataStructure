/*We assume that next[i] is the longest common prefix and suffix of P[0..i]*/
pub fn get_next<T>(t: Vec<T>) -> Vec<isize> where T: PartialEq + std::fmt::Debug {
    let mut next = vec![-1isize; t.len()];
    let mut i = 0isize;
    let mut j = -1isize;
    let n = t.len() as isize;
    next[0] = -1;
    // i: the current position to be compared with j, which the the last matched position with t[i-1]
    while i < n-1 {
        if j == -1 || t[i as usize] == t[j as usize] {
            i += 1;
            j += 1;
            next[i as usize] = j; 
        }
        else{
            //i stays the same
            j = next[j as usize]; 
            //chech if t[i] != t[j], then j jumps to next[j], prepare to check if t[1] == t[j]. Note that t[0..next[j]] == t[(i-next[j]).. i]. Before i, the pattern is [[XXX], ..., [XXX]], where length of [XXX] is next[j]  
        }
    }
    return next;
}
pub fn index_kmp<T>(s: Vec<T>, t: Vec<T>, pos: usize) -> usize 
where T: PartialEq + std::fmt::Debug {
    let mut i = pos as isize; // i: pattern index, j: string index
    let mut j = 0isize; 
    let n = s.len() as isize; // S: string, P: pattern
    let m = t.len() as isize;    
    let next = vec![-1isize; m as usize];
    while i < n && j < m { // i: pattern index, j: string index        
        if j == -1 || s[i as usize] == t[j as usize] { // if S[i] == P[j], j++
            i += 1;
            j += 1;
        } else { // if S[i] != P[j], j = next[j]
            j = next[j as usize];
        }
    }   
    if j >= m{
        return (i-m) as usize;
    }
    return 0usize;
}

#[cfg(test)]
mod test{
    use super::get_next;
    #[test]
    fn test_kmp(){
        let s = String::from("abaabcac");
        let v = s.into_bytes(); // convert string to vec<char>
        let p = get_next(v);
        let r = [-1, 0, 0, 1, 1, 2, 0, 1];
        assert_eq!(p.len(), r.len());
        for i in 0..p.len()-1{
            assert_eq!(p[i], r[i]);
        }
    }
}