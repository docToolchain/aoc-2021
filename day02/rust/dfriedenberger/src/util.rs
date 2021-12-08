use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use regex::Regex;

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

pub fn parse(data: &Vec<String>) -> std::io::Result<Vec<(String,i32)>> {

    let re = Regex::new(r"^([a-z]+)\s(\d+)$").unwrap();

    let mut parsed_data = Vec::new();

    let l = data.len();
    for i in 0..l {
        for cap in re.captures_iter(data.get(i).unwrap()) {
            let cmd = cap[1].to_string();
            let numb = cap[2].parse::<i32>().unwrap();
            parsed_data.push((cmd,numb));
        }
        
    }

    return Ok(parsed_data)
}

//tag::course[]
pub fn course(data: &Vec<(String,i32)>) -> std::io::Result<(i32,i32)> {
    
    let mut deep = 0;
    let mut position = 0;

    let l = data.len();
    for i in 0..l {

        let step = data.get(i).unwrap();

        if step.0 == "forward" { position += step.1 }
        if step.0 == "up" { deep -= step.1; }
        if step.0 == "down" { deep += step.1; }

    }

    return Ok((position,deep))

}
//end::course[]

//tag::course2[]
pub fn course2(data: &Vec<(String,i32)>) -> std::io::Result<(i32,i32)> {
    
    let mut deep = 0;
    let mut position = 0;
    let mut aim = 0;

    let l = data.len();
    for i in 0..l {

        let step = data.get(i).unwrap();

        if step.0 == "forward" { position += step.1; deep += aim * step.1; }
        if step.0 == "up" { aim -= step.1; }
        if step.0 == "down" { aim += step.1; }

    }

    return Ok((position,deep))
    
}
//end::course2[]
