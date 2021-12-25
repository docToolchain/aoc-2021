
pub fn index(str : &str,z : char) -> i32 {
    match str.find(z) {
        Some(a) => a as i32,
        None => -1
    }
}

pub fn checkline(line : &String) -> Option<char> {

    let row: Vec<char> = line.chars().collect();

    //validate
    let mut stack: Vec<char> = Vec::new();

    for c in row {

        
        if index("[({<",c) >= 0 {
            stack.push(c);            
        }

        if index("])}>",c) >= 0 {                
            //try pop_back
            let c2 = stack.remove(stack.len() - 1);
            
            if index("[({<",c2) != index("])}>",c)
            {
                //println!("corrupt line: {} != {} in line {}",c2,c,line);
                return Some(c);
            }
        }
        //println!("Stack {:?}",stack);

    }
    if stack.len() != 0
    {
        //println!("incomplete line: {}",line);
    }
    return None;


}

pub fn checkline2(line : &String) -> Option<Vec<char>> {

    let row: Vec<char> = line.chars().collect();

    //validate
    let mut stack: Vec<char> = Vec::new();

    for c in row {

        
        if index("[({<",c) >= 0 {
            stack.push(c);            
        }

        if index("])}>",c) >= 0 {                
            //try pop_back
            let c2 = stack.remove(stack.len() - 1);
            
            if index("[({<",c2) != index("])}>",c)
            {
                //println!("corrupt line: {} != {} in line {}",c2,c,line);
                return None;
            }
        }
        //println!("Stack {:?}",stack);

    }
    if stack.len() != 0
    {
        //println!("incomplete line: {}",line);
        return Some(stack);

    }
    return None;


}

//tag::create_bingo_boards[]
pub fn validate(data: &Vec<String>) -> std::io::Result<u32> {

    let l = data.len();    
   
    let mut err = 0;
    for i in 0..l {
        let line = data.get(i).unwrap();
        match checkline(line) {
            Some(c) => {
                err += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!(" unknown char {}",c)
                }
            },
            None => {}
        }
    }

    return Ok(err)
}

pub fn autocomplete(data: &Vec<String>) -> std::io::Result<u64> {

    let l = data.len();    
   
    let mut scores = Vec::new();
    for i in 0..l {
        let line = data.get(i).unwrap();
        match checkline2(line) {
            Some(stack) => {
                let mut score : u64 = 0;
                for c in stack.iter().rev() {
                    score *= 5;
                    score += match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!(" unknown char {}",c)
                    }
                }
                scores.push(score);
                //println!("Stack {:?} scores {} in line {}",stack,score,line)
            },
            None => {}
        }
    }

    //return middle of scores
    scores.sort();
    let l = scores.len();
    return Ok(*scores.get((l-1)/2).unwrap())
}


