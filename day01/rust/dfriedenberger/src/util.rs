use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

pub fn read_file(filename: &str) -> std::io::Result<Vec<i32>> {

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
            Ok(l) => {
                match l.parse::<i32>() {
                    Ok(v) => data.push(v),
                    Err(why) => panic!("couldn't parse: {}", why),
                }
            }
        }
    };

    return Ok(data);
     
}

//tag::calculate-increases[]
pub fn calculate_increases(data : &Vec<i32>) -> std::io::Result<i32>
{
    let mut sum = 0;
    let l = data.len();
    for ix in 1..l {
        if data[ix] > data[ix-1] { sum+= 1; }
    }
    return Ok(sum);
}
//end::calculate-increases[]

//tag::calculate-window[]
pub fn calculate_window(data : &Vec<i32>) ->  std::io::Result<Vec<i32>> 
{
    let l = data.len();
    let mut data2 = Vec::new();
    for ix in 2..l {
        let window = data[ix] + data[ix-1] + data[ix-2];
        data2.push(window)
    }
    return Ok(data2);
}
//end::calculate-window[]
