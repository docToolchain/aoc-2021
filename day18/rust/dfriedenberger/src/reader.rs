use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;


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
