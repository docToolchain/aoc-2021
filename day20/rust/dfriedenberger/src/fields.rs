//Playing fields
use array2d::Array2D;
use std::collections::HashMap;
//use regex::Regex;

//Array2D based field
pub struct BingoBoard {
    array : Array2D<i32>,
    mark : Array2D<bool>
}

impl BingoBoard {
    pub fn new(rows : Vec<Vec<i32>>) -> BingoBoard
    {
        BingoBoard {
            array : Array2D::from_rows(&rows),
            mark : Array2D::filled_with(false,rows[0].len(),rows.len())
        }
    }

    pub fn get(&self,x : usize,y : usize) -> i32 
    {
        return *self.array.get(x,y).unwrap();
    }

    pub fn set(&mut self,x : usize,y : usize,v : i32) 
    {
        self.array.set(x,y,v).ok();
    }

    pub fn size(&self) -> usize {
        self.array.row_len() * self.array.column_len()
    }

    pub fn mark_number(&mut self,n : i32)
    {
        for y in 0..self.array.row_len() {
            for x in 0..self.array.column_len() {
                let i = *self.array.get(x,y).unwrap();
                if i == n {
                    self.mark.set(x,y,true).ok();
                }
            }
        }
    }

}

//Hashmap based 
pub struct Diagram {
    field : HashMap<String, char>,
    min_x : i32,
    max_x : i32,
    min_y : i32,
    max_y : i32,
    outside : char
}

impl Diagram {
    pub fn new(outside : char) -> Diagram
    {
        Diagram {
            field : HashMap::new(),
            min_x : 0,
            max_x : 0,
            min_y : 0,
            max_y : 0,
            outside 
        }
    }
    pub fn get_outside(&self) -> char {
        self.outside
    }

    pub fn set(&mut self,x : i32,y : i32,c : char)
    {
        if x < self.min_x { self.min_x = x; }
        if x > self.max_x { self.max_x = x; }
        if y < self.min_y { self.min_y = y; }
        if y > self.max_y { self.max_y = y; }


        let key = format!("{},{}", x,y);

        self.field.insert(key,c);
    }
    pub fn pixels(&self) -> usize {
        let mut cnt = 0;
        for key in self.field.keys() {
            match self.field.get(key) {
                Some('#') => { cnt += 1; },
                Some(_) => {},
                None => panic!("Something went wrong")
            } 
        }
      cnt
    }
    pub fn get_range(&self) -> (i32,i32,i32,i32) {
        return (self.min_x,self.max_x,self.min_y,self.max_y)
    }

    pub fn get(&self,x : i32,y:i32) -> char {

        let key = format!("{},{}", x,y);
        match self.field.get(&key) {
            Some(c) => *c,
            None => self.outside
        }

    }

    pub fn get_pattern(&self, x : i32,y : i32) -> Vec<char> {

        let mut v = Vec::new();
        for i in y-1..y+2  {
            for j in x-1..x+2  {
                v.push(self.get(j,i));
            } 
        }
        return v;

    }

    pub fn dump(&self) 
    {
        for y in self.min_y..self.max_y+1 {

            let mut line = String::new();
            for x in self.min_x..self.max_x+1 {
                let val = self.get(x,y).to_string();
                line.push_str(&val);
            }
            println!("{}",line);
        }
    }
}
