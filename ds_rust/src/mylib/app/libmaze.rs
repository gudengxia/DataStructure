//use std::io::BufReader;
//use std::io::BufRead;
//use std::fs::File;
use calamine::{open_workbook, Reader, Xlsx};
type Stack<T> = Vec<T>;
pub struct MazeSolution{
    maze: Vec<Vec<bool>>,
    m: usize,
    n: usize,
    s: Stack<Loc>
}
#[derive(Clone, Debug)]
pub struct Loc{
    pub i: usize,
    pub j: usize, 
    pub dir: usize
}

impl Loc{
    /* pub fn new()->Loc{
        Loc{i: 0, j: 0, dir: 0}
    } */
}
impl MazeSolution{
    pub fn new() -> MazeSolution{
        let p = Vec::<Vec<bool>>::new();
        let s = Stack::<Loc>::new();
        MazeSolution{maze: p, m: 0, n: 0, s: s}
    }

    pub fn solve(&mut self, start: Loc, end: Loc){
        self.s.push(start.clone());
        self.maze[start.clone().i][start.clone().j] = false;
        while let Some(cur) = self.s.clone().last(){
            if cur.i == end.i && cur.j == end.j{
                break;
            }
            println!("top: ({},{}, {})", cur.clone().i, cur.clone().j, cur.clone().dir);
            if let Some(next) = self.nextpos(cur.clone()){//1. new position is available
                //1. next position is available
                self.s.push(next.clone());
                println!("({},{}) push", next.clone().i, next.clone().j);
                self.maze[next.clone().i][next.clone().j] = false;
            }else{
                //2. next posiotion is unvailable
                //2-1 change direction
                self.s.pop();
                println!("pop");
                if cur.dir < 3{
                    let mut nextpos = cur.clone();
                    nextpos.dir += 1;
                    self.s.push(nextpos.clone());
                    println!("({},{}, {}) push", nextpos.clone().i, nextpos.clone().j, nextpos.dir);
                }
                //2-2 backtrack 
                //self.maze[cur.i][cur.j] = false;
            }
        }
    }

    pub fn nextpos(&self, cur: Loc)->Option<Loc>{
        /*let mut loc = Loc::new();
        loc.i = self.i;
        loc.j = self.j;
        loc.dir = self.j;*/
        let mut loc = cur; 
        
        match loc.dir{
            0 => loc.j += 1,
            1 => loc.i += 1,
            2 => loc.j -= 1,
            3 => loc.i -= 1,
            _ => return None
        }
        if self.maze[loc.i][loc.j] == false{
            return None;
        }
        loc.dir = 0;
        Some(loc)
    }

    pub fn print_path(&mut self){
        if self.s.is_empty(){
            println!("No path.")
        }
        else{
            while let Some(e) = self.s.pop(){
                print!("{}, {},", e.i, e.j);
                match e.dir{
                    0 => println!(">"),
                    1 => println!("V"),
                    2 => println!("<"),
                    3 => println!("^"),
                    _ => println!(" ")
                }
            }
        }
    }
    /* pub fn read_maze(&mut self, file_path: &str)-> Result<(), Box<dyn std::error::Error>>{
        let file = File::open(file_path)?;
        // Create a buffered reader
        let reader = BufReader::new(&file);
        
        // Read characters one by one
        for line in reader.lines(){
            let line = line?;
            let mut v = Vec::<char>::new();
            let mut flag = false;
            for ch in line.chars(){
                if ch == '\n'{
                    break
                }
                if ch == '#'
                {
                    flag = true;
                }
                if flag{
                    v.push(ch);
                }
            }
            if v.len() > self.n{
                self.n = v.len();
            }
            self.maze.push(v);
            self.m += 1;
        }
        Ok(())
    } */
    pub fn read_maze(&mut self, file_path: &str, sheet: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Open the workbook
        let mut workbook: Xlsx<_> = open_workbook(file_path).expect("open xlsx file error.");
    
        // Access the specified sheet
        if let Some(Ok(range)) = workbook.worksheet_range(sheet) {
            let mut cols;
            let mut flag;
            for row in range.rows() {
                cols = 0;
                flag = false;
                let mut v = Vec::<bool>::new();
                for cell in row {
                    if let calamine::DataType::String(s) = cell {
                        // Handle non-empty cells (e.g., strings like "#")   
                        if s == "x" {
                            flag = true;
                            v.push(false); // Treat "#" as "true"
                            cols += 1;
                        }else{
                            if flag {
                                v.push(true);
                                cols += 1;
                            }
                        }
                    }else if *cell == calamine::DataType::Empty{
                        if flag {
                            v.push(true);
                            cols += 1;
                        } 
                    }
                }
                if flag{
                    self.maze.push(v);
                    self.m += 1;
                    if cols > self.n{
                        self.n = cols;
                    }
                }            
            }
        }else {
            return Err("Sheet not found or could not be read.".into());
        }
        Ok(())
    }
    pub fn print_maze(&self){
        for row in self.maze.clone(){
            for cell in row{
                if cell{
                    print!(" ");
                }
                else{
                    print!("x");
                }
            }
            println!("");
        }
        println!("m={}, n ={}", self.m, self.n);
    }
}

#[cfg(test)]
mod test{
    use super::{Loc, MazeSolution};
    #[test]
    fn test_maze(){
        let file_path = "./src/data/maze.xlsx"; // Relative path
        let sheet_name = "maze1";
        let mut sol = MazeSolution::new();
        let _= sol.read_maze(file_path, sheet_name);
        sol.print_maze();
        let start = Loc{i: 1, j: 1, dir: 0};
        let end = Loc{i: 8, j: 8, dir: 0};
        sol.solve(start, end);
        sol.print_path();
    }
}