pub struct EularSieve{
    pub ans: Vec<usize>,
    //n: usize
}

impl EularSieve{
    pub fn new()-> EularSieve{
        EularSieve{
            ans: Vec::<usize>::new(),
            //n: max
        }
    }

    pub fn solve(&mut self, n: usize){
        if n == 0 || n == 1{
            return;
        }

        self.ans.clear();
        let mut isprime = vec![true; n+1];
        isprime[0] = false;
        isprime[1] = false;
        for i in 2..=n{
            if isprime[i]{
                self.ans.push(i);
            }
            for p in self.ans.clone(){
                if p * i <= n{
                    isprime[p * i] = false;
                }
                else{
                    break;
                }
                if i % p == 0{
                    break;
                } 
            }
        }    
    }
} 

pub fn is_prime(n : usize) -> bool{
    if n == 0 || n == 1{
        return false;
    }

    let mut i = 2usize;
    while i * i <= n{
        if n % i == 0{
            return false;
        }
        i += 1;
    }
    return true;
}

#[cfg(test)]
mod test{
    use super::{is_prime, EularSieve};
    use std::time::Instant;

    #[test]
    fn test_eular(){
        let n = 100000;

        let mut eular = EularSieve::new();
        eular.solve(n);
        
        let mut ans = Vec::<usize>::new();
        for i in 1..n{
            if is_prime(i){
                ans.push(i);
            }
        }

        assert_eq!(ans.len(), eular.ans.len());
        for i in 0..ans.len(){
            assert_eq!(ans[i], eular.ans[i])
        }  
    }

/*************  ✨ Codeium Command 🌟  *************/
    #[test]
    /// why Eular Sieve preformace is worse than the common method?
    /// The data is too small?
    #[test] //why Eular Sieve preformace is worse than the common method, the data is too small?
    fn test_eular_performance(){
        let n = 100000;
        let mut eular = EularSieve::new();
        let start1 = Instant::now();
        eular.solve(n);
        let duration1 = start1.elapsed();
        println!("Eular Sieve run time:{:?}", duration1);
    
        let start2 = Instant::now();
        let mut ans = Vec::<usize>::new();
        for i in 1..=n{
            if is_prime(i)
            {
                ans.push(i);
            }
        }
        let duration2 = start2.elapsed();
        println!("Common method run time:{:?}", duration2);
    }
/******  8ee96707-e633-49c1-856b-a14f0559014c  *******/
}

