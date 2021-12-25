//Playing fields
use array2d::Array2D;
use std::collections::HashMap;
//use regex::Regex;
//Array2D based field
pub struct BingoBoard {
    array : Array2D<u32>,
    mark : Array2D<bool>
}

impl BingoBoard {
    pub fn new(rows : Vec<Vec<u32>>) -> BingoBoard
    {
        BingoBoard {
            array : Array2D::from_rows(&rows),
            mark : Array2D::filled_with(false,rows[0].len(),rows.len())
        }
    }

    pub fn get(&self,x : usize,y : usize) -> u32 
    {
        return *self.array.get(x,y).unwrap();
    }

    pub fn set(&mut self,x : usize,y : usize,v : u32) 
    {
        self.array.set(x,y,v).ok();
    }

    pub fn size(&self) -> usize {
        self.array.row_len() * self.array.column_len()
    }
    pub fn row_len(&self) -> usize {
        self.array.row_len()
    }
    pub fn column_len(&self) -> usize {
        self.array.column_len()
    }

    pub fn mark_number(&mut self,n : u32)
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
    field : HashMap<String, i32>
}

impl Diagram {
    pub fn new() -> Diagram
    {
        Diagram {
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
