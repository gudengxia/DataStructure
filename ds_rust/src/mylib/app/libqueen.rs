pub struct EQueen{
    board: Vec<Vec<bool>>,
    n: usize
}

#[derive(Copy, Clone)]
struct Loc{
    i: usize,
    j: usize
}
impl EQueen{
    pub fn new(n: usize) -> EQueen{
        let board = vec![vec![false; n]; n];
        EQueen{board: board, n: n}
    }

    pub fn solve(&mut self)->usize{
        let mut num_of_solutions = 0usize;
        let mut s = Vec::<Loc>::new();
        s.push(Loc{i: 0, j: 0});
        self.board[0][0] = true;
        //println!("push({}, {})", 0, 0);
        while !s.is_empty(){
            if let Some(cur) = s.clone().last_mut(){
                if cur.i == self.n - 1{
                    num_of_solutions += 1;
                    println!("Solution #{}", num_of_solutions);
                    self.print_board(); //find a solution
                    println!("");

                    //backtrack, try next solution
                    s.pop();
                    self.board[cur.i][cur.j] = false;
                    //println!("pop({}, {})", cur.i, cur.j);
                    while self.next_safe_pos_in_same_row(cur.clone()).is_none() && !s.is_empty(){
                        //if let Some(pos) = s.last(){
                            //cur = &s.pop().unwrap().clone();
                            if let Some(pos) = s.pop(){
                                cur.i = pos.i;
                                cur.j = pos.j;
                                self.board[cur.i][cur.j] = false;
                                //println!("pop({}, {})", cur.i, cur.j);
                            }                            
                            //cur = pos;
                        //}
                    }
                    if let Some(pos) = self.next_safe_pos_in_same_row(cur.clone()){
                        //find a new safe pos in the same row, try another posibility in this row again
                        self.board[pos.i][pos.j] = true;
                        //println!("push({}, {})", pos.i, pos.j);
                        s.push(pos);
                    }
                } // the abortion condition

                if let Some(pos) = self.next_safe_pos_in_next_row(cur.clone()){
                    // find a new safe pos in the next row
                    self.board[pos.i][pos.j] = true;
                    s.push(Loc{i: pos.i, j: pos.j});
                    //println!("push({}, {})", pos.i, pos.j);
                }else {//no safe pos in the next row, backtrack
                    s.pop(); 
                    self.board[cur.i][cur.j] = false;
                    //println!("pop({}, {})", cur.i, cur.j);
                    //find a new safe pos in the same row
                    while self.next_safe_pos_in_same_row(cur.clone()).is_none() && !s.is_empty(){
                        if let Some(pos) = s.clone().last(){    
                            s.pop();
                            self.board[pos.i][pos.j] = false;
                            //println!("pop({}, {})", pos.i, pos.j);
                            cur.i = pos.i;
                            cur.j = pos.j;
                        }
                    }
                    if let Some(pos) = self.next_safe_pos_in_same_row(cur.clone()){
                        //find a new safe pos in the same row, try another posibility in this row again                
                        s.push(pos);
                        self.board[pos.i][pos.j] = true;
                        //println!("push({}, {})", pos.i, pos.j);
                    }
                }
            }
        }
        num_of_solutions
    }

    fn next_safe_pos_in_next_row(&self, cur: Loc)->Option<Loc>{
        let i = cur.i + 1;
        let mut j = 0;

        while j < self.n{
            if self.is_safe(i, j){
                return Some(Loc{i: i, j: j});
            }
            j += 1;
        }
        return None;
    }
    fn next_safe_pos_in_same_row(&self, cur: Loc)->Option<Loc>{
        let i = cur.i;
        let mut j = cur.j + 1;
        while j < self.n{
            if self.is_safe(i, j){
                return Some(Loc{i: i, j: j});
            }
            j += 1;
        }
        return None;
    }

    fn is_safe(&self, row: usize, col: usize) -> bool{
        for i in 0..row{
            if self.board[i][col]{
                return false;
            }        
        }
        for j in 0..col{
            if self.board[row][j]{
                return false;
            }
        }
        

        if row != 0 && col != 0{
            let mut i = (row - 1) as isize;
            let mut j = (col - 1) as isize;
            while i >= 0 as isize && j >= 0 as isize{
                if self.board[i as usize][j as usize]{
                    return false;
                }
                i -= 1;
                j -= 1;
            }
        }
        
        if row != 0 && col != self.n - 1{
            let mut i = (row - 1) as isize;
            let mut j = (col + 1) as isize;
            while i >= 0 as isize && j < self.n as isize{
                if self.board[i as usize][j as usize]{
                    return false;
                }
                i -= 1;
                j += 1;
            }
        }

        return true;
    }
    pub fn print_board(&self){
        for i in 0..self.n{    
            for j in 0..self.n{
                if self.board[i][j]{
                    print!("X");
                }else{
                    print!("O");
                }
            }
            println!("");
        }
    }
}
