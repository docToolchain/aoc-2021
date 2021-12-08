use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use array2d::Array2D;
use std::io::{Error, ErrorKind};

pub struct Board {
    array : Array2D<i32>,
    mark : Array2D<bool>
}


impl Board {
    pub fn new(rows : Vec<Vec<i32>>) -> Board
    {
        Board {
            array : Array2D::from_rows(&rows),
            mark : Array2D::filled_with(false,rows.len(),rows[0].len())
        }
    }

    pub fn get(&self,y : usize,x : usize) -> i32 
    {
        return *self.array.get(y,x).unwrap();
    }

    pub fn mark_number(&mut self,n : i32)
    {
        for y in 0..self.array.row_len() {
            for x in 0..self.array.column_len() {
                let i = *self.array.get(y,x).unwrap();
                if i == n {
                    self.mark.set(y,x,true);
                }
            }
        }
    }

    fn has_row_bingo(&self,y : usize) -> bool 
    {
        for x in 0..self.array.column_len() {
            if *self.mark.get(y,x).unwrap() == false {
                return false;
            }
        }
        true
    }

    fn has_column_bingo(&self,x : usize) -> bool 
    {
        for y in 0..self.array.row_len() {
            if *self.mark.get(y,x).unwrap() == false {
                return false;
            }
        }
        true
    }

    pub fn has_bingo(&self) -> bool
    {
        for y in 0..self.array.row_len() {
            if self.has_row_bingo(y) {
                return true;
            }
        }

        for x in 0..self.array.column_len() {
            if self.has_column_bingo(x) {
                return true;
            }
        }

        return false;
    }

    pub fn summarize_unmarked_numbers(&self) -> i32
    {
        let mut sum = 0;
        for y in 0..self.array.row_len() {
            for x in 0..self.array.column_len() {
                let i = *self.array.get(y,x).unwrap();
                let m = *self.mark.get(y,x).unwrap();
                if !m { sum += i; }
            }
        }
        return sum;
    }
}

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
        }

    };

    return Ok(data);
     
}

//tag::count-bits[]
pub fn create_game(data: &Vec<String>) -> std::io::Result<(Vec<i32>,Vec<Board>)> {



    let l = data.len();    

    let first_line = data.get(0).unwrap();
    let numbers: Vec<i32> = first_line.split(",").map(|s| s.parse().unwrap()).collect();

    let mut boards = Vec::new();
    let mut rows = Vec::new();

    for i in 1..l {
        let line = data.get(i).unwrap();
        if line.is_empty() {

            if rows.len() > 0
            {
                boards.push(Board::new(rows));
            }
            rows = Vec::new();
            continue;
        }
        //parse line
        let row: Vec<i32> = line.split(' ')
                                     .map(|s| s.trim())    
                                     .filter(|s| !s.is_empty())
                                     .map(|s| s.parse().unwrap()).collect();
        rows.push(row);

    }
    boards.push(Board::new(rows));


    return Ok((numbers,boards))
}
//end::count-bits[]

//tag::play-game-until-first-bingo[]
pub fn play_game_util_first_bingo(numbers: &Vec<i32>,boards : &mut Vec<Board>) -> std::io::Result<(i32,i32)>
{
    for &n in numbers {
        let l = boards.len();
        for i in 0..l {
            boards[i].mark_number(n);
            if boards[i].has_bingo() {
                return Ok((boards[i].summarize_unmarked_numbers(),n))
            }
        }
    }

    Err(Error::new(ErrorKind::Other, "oh no!"))
}
//end::play-game-until-first-bingo[]

pub fn all_boards_ready(boards : &mut Vec<Board>) -> bool 
{
    let l = boards.len();
        
    for i in 0..l {
        if !boards[i].has_bingo() {
            return false
        }
    }
    return true
}

pub fn play_game_util_last_bingo(numbers: &Vec<i32>,boards : &mut Vec<Board>) -> std::io::Result<(i32,i32)>
{

    for &n in numbers {

        let l = boards.len();
        
        for i in 0..l {

            boards[i].mark_number(n);

            //tag::all-boards-ready[]
            if boards[i].has_bingo() {
                if all_boards_ready(boards) {
                    return Ok((boards[i].summarize_unmarked_numbers(),n))
                }
            }
            //end::all-boards-ready[]
        }


    }

    Err(Error::new(ErrorKind::Other, "oh no!"))
}