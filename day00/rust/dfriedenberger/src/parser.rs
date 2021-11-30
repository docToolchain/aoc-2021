use std::collections::HashMap;

pub struct Parser { }

impl Parser {
    pub fn parse(&self,line: &str) -> (i32 , i32) {

    
        let mut charactors = HashMap::new();

        for c in line.chars() { 

                let n = match charactors.get(&c) {
                        Some(i) => i + 1,
                        _ => 1
                };
                charactors.insert(c, n);
   
        }
        
        let mut a = 0;
        let mut b = 0;

        for (_, &cnt) in charactors.iter() {
            if cnt == 2 { a = 1; }
            if cnt == 3 { b = 1; }
        }
        return (a,b)
    }
}
