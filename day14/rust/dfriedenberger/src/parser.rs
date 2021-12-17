use crate::fields::BingoBoard;
use std::collections::HashMap;


//tag::create_bingo_boards[]
pub fn create_bingo_boards(data: &Vec<String>) -> std::io::Result<(String,HashMap<String,String>)> {

    let l = data.len();    

    let polymer_template = data.get(0).unwrap().trim().to_string();

    let mut rules = HashMap::new();

    for i in 2..l {
        let line = data.get(i).unwrap();
       
        //parse line
        let row: Vec<String> = line.split("->")
                                     .map(|s| s.trim())    
                                     .map(|s| s.to_string()).collect();
        rules.insert(row[0].to_string(),row[1].to_string());

    }


    return Ok((polymer_template,rules))
}
//end::create_bingo_boards[]


pub fn add(counter : &mut HashMap<String,u64>,a : &HashMap<String,u64>)
{
    for (key,c) in a {
        let cnt : u64 = if counter.contains_key(key) { *counter.get(key).unwrap() } else { 0 };
        counter.insert(key.to_string(),cnt + c);
    }
}

pub fn copy(counter : &HashMap<String,u64>) -> HashMap<String,u64> {

    let mut n = HashMap::new();

    for (key,cnt) in counter {
        n.insert(key.to_string(),*cnt);
    }

    return n;
}

pub fn create(left : &String,right : &String,rules: &HashMap<String,String>,cache :&mut HashMap<String,HashMap<String,u64>> ,deep : u32) -> std::io::Result<HashMap<String,u64>> {

    let mut counter = HashMap::new();

    if deep > 0 {
      
        let mut key = left.to_string();
        key.push_str(right);
        
        let mut cachekey = left.to_string();
        cachekey.push_str(right);
        cachekey.push_str(&deep.to_string());

        //check cache
        if cache.contains_key(&cachekey) {
            return Ok(copy(cache.get(&cachekey).unwrap()));
        }

        let middle = rules.get(&key).unwrap();

        let cnt : u64 = if counter.contains_key(middle) { *counter.get(middle).unwrap() } else { 0 };
        counter.insert(middle.to_string(),cnt + 1);

        let c1 = create(left,&middle,rules,cache,deep - 1).unwrap();
        let c2 = create(&middle,right,rules,cache,deep - 1).unwrap();

        add(&mut counter,&c1);
        add(&mut counter,&c2);

        //put cache
        cache.insert(cachekey,copy(&counter));
    }

    Ok(counter)

}

pub fn create_polymer(polymer_template : &String,rules: &HashMap<String,String>,deep : u32) -> std::io::Result<u64> {

    let chars : Vec<String> = polymer_template.chars().map(|s| s.to_string()).collect();
    let l = chars.len();
    
    let mut s  = HashMap::new();

    for i in 0..l {
        let cnt : u64 = if s.contains_key(&chars[i]) { *s.get(&chars[i]).unwrap() } else { 0 };
        s.insert(chars[i].to_string(),cnt + 1);
    }

    let mut cache : HashMap<String,HashMap<String,u64>> = HashMap::new();

    for i in 0..(l-1) {
        let c = create(&chars[i],&chars[i+1],rules,&mut cache,deep).unwrap();
        add(&mut s,&c);
    }

    println!("{:?}",s);

    let mut min : u64 = 0;
    let mut max : u64 = 0;
    let mut first = true;

    for (_,c) in s.into_iter() {
        if c < min || first { min = c};
        if c > max || first { max = c};
        first = false;
    }

    println!("{} -> {}",min,max);


    Ok(max -min)
}