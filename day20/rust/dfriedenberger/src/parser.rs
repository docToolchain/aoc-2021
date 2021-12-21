use crate::fields::Diagram;


//tag::create_bingo_boards[]
pub fn create_bingo_boards(data: &Vec<String>) -> std::io::Result<(Vec<char>,Diagram)> {



    let l = data.len();    


    let mut algorithm = Vec::new();
    let mut image = Diagram::new('.');

    let mut is_algorithm = true;
    let mut y = 0;

    for i in 0..l {
        let line = data.get(i).unwrap();

        if line.is_empty() {
            is_algorithm = false;
            continue;
        }

        let mut chars: Vec<char> = line.chars().collect();

        if is_algorithm {
            algorithm.append(&mut chars);
        } else {
            let l = chars.len();
            for x in 0..l {
                image.set(x as i32,y,chars[x]);
            }
            y += 1;
        }

    }


    return Ok((algorithm,image))
}


pub fn to_num(pattern : &Vec<char>) -> u32 {

    let mut n = 0;
    for c in pattern {
        n *= 2;

        if *c == '#' {
            n += 1;
        }


    }
    return n;
}

pub fn enhance(algorithm : &Vec<char>,image : &Diagram,outside : char) -> Diagram {



    let mut new_image = Diagram::new(outside);

 
    let (min_x , max_x , min_y , max_y ) = image.get_range();

    for y in min_y-1..max_y+2 {
        for x in min_x-1..max_x+2 {

            let pattern = image.get_pattern(x,y);
            let num = to_num(&pattern);
            let c = algorithm[num as usize];
            new_image.set(x,y,c);

        } 
    } 


    return new_image;
}



//end::create_bingo_boards[]

