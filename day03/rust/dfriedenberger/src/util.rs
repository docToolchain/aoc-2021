use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
pub enum FilterStrategy {
    MostCommonBit,
    LeastCommonBit,
}

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

//tag::count-bits[]
pub fn count_bits(data: &Vec<String>) -> std::io::Result<Vec<i32>> {


    let mut bits = Vec::new();

    let l = data.len();
    let size = data.get(0).unwrap().len();
    for _p in 0..size {
        bits.push(0);
    }


    for i in 0..l {
        let line = data.get(i).unwrap();
        let b: &[u8] = line.as_bytes();
        for p in 0..size {
            let c: char = b[p] as char;  
            match c {
                '1' => bits[p] += 1,
                '0' => (),
                _ => panic!("unknown char {}",c)
            }
        }
       
        
    }

    return Ok(bits)
}
//end::count-bits[]

//tag::power-consumption[]
pub fn power_consumption(bits: &Vec<i32>,numbers : usize) -> std::io::Result<(i32,i32)> {

    let mut gamma : i32 = 0;
    let mut epsilon : i32 = 0;

    let l = bits.len();

    for ix in 0..l {

        gamma <<= 1;
        epsilon <<= 1;

        let cnt1 = *bits.get(ix).unwrap();
        if cnt1 > ((numbers as i32)/2)
        {
            //'1' is the most common bit
            gamma += 1;
        }
        else
        {
            //'0' is the most common bit
            epsilon += 1;
        }
        
    }

    Ok((gamma,epsilon))

}
//end::power-consumption[]

pub fn filter_by_criteria(data: &Vec<String>, bit : char, position : usize) -> std::io::Result<Vec<String>> {

    let mut vec = Vec::new();

    let l = data.len();
    for i in 0..l {
        let line = data.get(i).unwrap();
        let b: &[u8] = line.as_bytes();
        let c = b[position] as char;
        if c == bit {
            vec.push(String::from(line));
        }
    }
    return Ok(vec);
}

pub fn convert2number(line : &String) -> std::io::Result<i32> {

    let mut number = 0;
    let ll = line.len();
    let b: &[u8] = line.as_bytes();

    for p in 0..ll {

        number <<= 1;

        let c: char = b[p] as char;  
        match c {
            '0' => (),
            '1' => number += 1,
            _ => panic!("unknonw bit {}",c)
        }
    }
    Ok(number)
}



pub fn count_bit(data: &Vec<String>, position : usize,filterStrategy : FilterStrategy) -> std::io::Result<char> 
{
    let mut bit0 = 0;
    let mut bit1 = 0;

    let l = data.len();
    for i in 0..l {
        let line = data.get(i).unwrap();
        let b: &[u8] = line.as_bytes();

        let c: char = b[position] as char;  
        match c {
            '1' => bit1 += 1,
            '0' => bit0 += 1,
            _ => panic!("unknown char {}",c)
        }
    }


    match filterStrategy {
        crate::util::FilterStrategy::LeastCommonBit => {
            if bit0 <= bit1 {
                return Ok('0');
            }
            return Ok('1');
        },
        crate::util::FilterStrategy::MostCommonBit => {
            if bit0 > bit1 {
                return Ok('0');
            }
        
            return Ok('1');
        },
    }
  
}

//tag::filter-by-strategy[]
pub fn filter_by_strategy(data: &Vec<String>,filterStrategy : FilterStrategy) -> std::io::Result<i32> {

    let mut ref_data : Vec<String> = data.to_vec();
    let ll = data.get(0).unwrap().len();

    for p in 0..ll {

        match ref_data.len() {
            0 => panic!("something went wrong"),
            1 => /* ready */ break,
            _ => (),
        }

        //reduce
        let bit = count_bit(&ref_data,p,filterStrategy).unwrap();
        let bl = ref_data.len();
        ref_data = filter_by_criteria(&ref_data,bit,p).unwrap();
        let al = ref_data.len();

    }

    if ref_data.len() != 1 {
        panic!("Something went wrong")
    }

    let number = convert2number(ref_data.get(0).unwrap()).unwrap();

    return Ok(number);

}
//end::filter-by-strategy[]

//tag::filter[]
pub fn filter(data: &Vec<String>) -> std::io::Result<(i32,i32)> {

    let oxygen_generator_rating = filter_by_strategy(data,crate::util::FilterStrategy::MostCommonBit).unwrap();
    let co2_scrubber_rating = filter_by_strategy(data,crate::util::FilterStrategy::LeastCommonBit).unwrap();

    Ok((oxygen_generator_rating,co2_scrubber_rating))

}
//end::filter[]
