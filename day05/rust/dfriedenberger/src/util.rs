use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
//use array2d::Array2D;
//use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use regex::Regex;

//tag::board[]
pub struct Board {
    field : HashMap<String, i32>
}

impl Board {
    pub fn new() -> Board
    {
        Board {
            field : HashMap::new()
        }
    }

    pub fn set(&mut self,x : i32,y : i32)
    {
        let key = format!("{},{}", x,y);

        let val = match self.field.get(&key) {
            Some(cnt) => *cnt,
            None => 0
        };
        self.field.insert(key,val+1);
    }

    pub fn count_crossing(&self) -> i32
    {
        let mut sum = 0;
        for (_key, val) in self.field.iter() {
            if *val > 1 { sum += 1;}
        }
        return sum;
    }

    pub fn dump(&self,width : usize,height : usize) 
    {
        for y in 0..height {

            let mut line = String::new();
            for x in 0..width {
                let key = format!("{},{}", x,y);
                let val = match self.field.get(&key) {
                    Some(cnt) => format!("{}",*cnt),
                    None => ".".to_string()
                };
                line.push_str(&val);
            }
            println!("{}",line);
        }
    }
}
//end::board[]

pub fn read_file(filename: &str) -> std::io::Result<Vec<String>> {

    let path = Path::new(filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();


    let mut data = Vec::new();

    for line in lines {
        match line {
            Err(why) => panic!("couldn't read line: {}", why),
            Ok(l) => data.push(l)
        };
    };

    return Ok(data);
     
}

//tag::count-bits[]
pub fn create_game(data: &Vec<String>,with_diagonals : bool) -> std::io::Result<Board> {


    let re = Regex::new(r"^(\d+),(\d+)\s->\s(\d+),(\d+)$").unwrap();
    let l = data.len();    


    let mut board = Board::new();
    for i in 0..l {
        let line = data.get(i).unwrap();
        //parse line
        for cap in re.captures_iter(line) {


            let mut x1 = cap[1].parse::<i32>().unwrap();
            let mut y1 = cap[2].parse::<i32>().unwrap();
            let mut x2 = cap[3].parse::<i32>().unwrap();
            let mut y2 = cap[4].parse::<i32>().unwrap();
            

           

            if x1 == x2 || y1 == y2 {
                //println!("{},{} -> {},{}",x1,y1,x2,y2);

                //tag::horizontal-vertical[]
                if x1 > x2 {
                    let xs = x1;
                    x1 = x2;
                    x2 = xs;
                }
                if y1 > y2 {
                    let ys = y1;
                    y1 = y2;
                    y2 = ys;
                }
                for x in x1..x2+1 {
                    for y in y1..y2+1 {
                        board.set(x,y);
                    }
                }
                //end::horizontal-vertical[]
            } else if with_diagonals {
                //consider diagonals
                //println!("{},{} -> {},{}",x1,y1,x2,y2);
                //tag::diagonal[]
                let mut x = x1;
                let mut y = y1;

                let xo = if x1 < x2 { 1 } else { -1};
                let yo = if y1 < y2 { 1 } else { -1};

                let cnt = if x1 < x2 { x2 - x1 + 1 } else { x1 - x2 + 1 };

                for _i in 0..cnt {
                    board.set(x,y);
                    x += xo;
                    y += yo;
                }
                //end::diagonal[]

            }



        }

    }


    return Ok(board)
}
//end::count-bits[]

