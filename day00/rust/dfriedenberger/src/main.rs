
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

use dfrieden_day00::parser::Parser;

fn main() {

    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let lines = io::BufReader::new(file).lines();

    let p = Parser {};

    let mut two = 0;
    let mut three = 0;

    for line in lines {
        match line {
            Err(why) => panic!("couldn't read line: {}", why),
            Ok(l) => {
                let (a,b) = p.parse(&l);
                two += a;
                three += b;
            }
        }
    };
    println!("{}",two * three);

}