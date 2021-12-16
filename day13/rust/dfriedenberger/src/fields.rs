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
    field : HashMap<String, (i32,i32)>
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

        //let val = match self.field.get(&key) {
        //    Some(cnt) => *cnt,
        //    None => 0
        //};
        self.field.insert(key,(x,y));
    }

    pub fn foldX(&mut self,xx : i32) {

        let keys : Vec<String> = self.field.keys().map(|s| s.to_string()).collect();
        for k in keys {
            let (x,y) = self.field.get(&k).unwrap();
            if(*x > xx) {
                let nx = xx - (*x - xx);
                self.set(nx,*y);
                self.field.remove(&k);
            }

        }
    }

    pub fn foldY(&mut self,yy : i32) {
        let keys : Vec<String> = self.field.keys().map(|s| s.to_string()).collect();
        for k in keys {
            let (x,y) = self.field.get(&k).unwrap();
            if(*y > yy) {
                let ny = yy - (*y - yy);
                //println!("move {},{} -> {},{}",*x,*y,*x,ny);
                self.set(*x,ny);
                self.field.remove(&k);
            }

        }
    }

    pub fn cnt(&self,) -> usize {
        return self.field.keys().len();
    }

    pub fn dump(&self) 
    {
        let mut maxx = 0;
        let mut maxy = 0;
        for k in self.field.keys() {
            let (x,y) = self.field.get(k).unwrap();
            if *x > maxx { maxx = *x}
            if *y > maxy { maxy = *y}
        }
        let height = maxy + 1;
        let width = maxx + 1;

        for y in 0..height {

            let mut line = String::new();
            for x in 0..width {
                let key = format!("{},{}", x,y);
                let val = match self.field.get(&key) {
                    Some(_) => "#".to_string(),
                    None => ".".to_string()
                };
                line.push_str(&val);
            }
            println!("{}",line);
        }
    }
}
