use crate::fields::Diagram;
use regex::Regex;

//tag::create_bingo_boards[]
pub fn create_bingo_boards(data: &Vec<String>) -> std::io::Result<(Diagram,Vec<(String,i32)>)> {



    let l = data.len();    

    let mut paper = Diagram::new();

    let mut i = 0;

    while i < l {
        let line = data.get(i).unwrap();
        i+=1;
        if line.is_empty() {
            break;
        }
        //parse line
        let coords: Vec<i32> = line.split(',')
                                     .map(|s| s.trim())    
                                     .filter(|s| !s.is_empty())
                                     .map(|s| s.parse().unwrap()).collect();
        paper.set(coords[0],coords[1]);
        
    }

    let mut instructions = Vec::new();
    let re = Regex::new(r"^fold\salong\s([xy])=(\d+)$").unwrap();

    while i < l {
        let line = data.get(i).unwrap();
        i+=1;

        for cap in re.captures_iter(line) {
            let letter = cap[1].to_string();
            let numb = cap[2].parse::<i32>().unwrap();
            instructions.push((letter,numb));
        }
    }


    return Ok((paper,instructions))
}
//end::create_bingo_boards[]

pub fn fold(paper : &mut Diagram,instructions : &Vec<(String,i32)>,once : bool) -> usize {

   
    for (letter,number) in instructions {
       match letter.as_ref() {
           "x" => {paper.foldX(*number);},
           "y" => {paper.foldY(*number);},
            _  => panic!("Unknown letter {}",letter)
        }
        if once { break;}
    }

    return paper.cnt();
}

