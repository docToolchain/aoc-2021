use crate::fields::SnailFishNumber;



pub fn create_octopus_grid(data: &Vec<String>) -> std::io::Result<Vec<Vec<u32>>> {

    let l = data.len();    
   
    let mut rows = Vec::new();

    for i in 0..l {
        let line = data.get(i).unwrap();
       
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(row);
    }
    
    //return Ok(OctopusGrid::new(rows))
    Ok(rows)
}


pub fn parse(number: &Vec<char>,i : &mut usize) -> SnailFishNumber {

        if number[*i] != '[' { 
            let value = number[*i].to_digit(10).unwrap();
            *i += 1;
            return SnailFishNumber::new(value,None,None);
        }
        
        *i += 1; //[
        let left = parse(number,i);
        *i += 1; //,
        let right = parse(number,i);
        *i += 1; //]

        SnailFishNumber::new(0,Some(Box::new(left)),Some(Box::new(right)))
}

pub fn create(number: &String) -> SnailFishNumber {
    
    let chars : Vec<char> = number.chars().collect();

    let mut i = 0;
    return parse(&chars,&mut i);

}

pub fn reduce(number: &String) -> String {

    let mut n = create(number);

    //reduce
    loop {

        //explode
        let mut ix = 0;
        let e = n.explode(1,&mut ix);
        //println!("Explode ?  {:?}",e);

        ix = 0;
        match e {
            None => {},
            Some(u) => {
                n.add(u.0,u.1,u.2,&mut ix);
                //println!("after explode: {}",n.to_string());
                continue; //explode again
            }
        }

        //split
        let s = n.split();
        if s {
            //println!("after split: {}",n.to_string());
            continue;
        }

        break;
        //let mut v = Vec::new();
        //n.flaten(&mut v);
        //println!("{:?}",v);


    }

    return n.to_string();


}



pub fn add(a: &String,b: &String) -> String {


    let n1 = create(a);
    let n2 = create(b);

    let sum = SnailFishNumber::new(0,Some(Box::new(n1)),Some(Box::new(n2)));

    return sum.to_string();

}

pub fn magnitude(number: &String) -> u32 {


    let n = create(number);
   

    return n.magnitude();

}

pub fn calculate(data : &Vec<String>) -> u32 {


    

    let mut num = data[0].to_string();
    let l = data.len();

    for i in 1..l {

        num = add(&num,&data[i]);
        num = reduce(&num);

    }

    return magnitude(&num);

}

pub fn calculate_max(data : &Vec<String>) -> u32 {


    

    let l = data.len();
    let mut max = 0;

    for i in 0..l {
        //println!(" {} of {}",i,l);
        for j in 0..l {
            if i == j { continue;}

            let mut num = add(&data[i],&data[j]);
            num = reduce(&num);
            let m = magnitude(&num);
            if m > max { max = m }
        }
    }

    return max;

}