use crate::fields::Scanner;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

//tag::create_bingo_boards[]
pub fn create_scanners(data: &Vec<String>) -> std::io::Result<Vec<Scanner>> {



    let l = data.len();    
    let re = Regex::new(r"^---\sscanner\s\d+\s---$").unwrap();

    let mut scanners = Vec::new();
    let mut coords = Vec::new();

    for i in 0..l {

        let line = data.get(i).unwrap();
        if re.is_match(line) {
            continue;
        }

        if line.is_empty() {
          
            scanners.push(Scanner::new(coords));
            coords = Vec::new();
            continue;
        }

        //parse line
        let coord: Vec<i32> = line.split(',')
                                     .map(|s| s.trim())    
                                     .filter(|s| !s.is_empty())
                                     .map(|s| s.parse().unwrap()).collect();
        coords.push((coord[0],coord[1],coord[2]));

    }

    scanners.push(Scanner::new(coords));


    return Ok(scanners)
}
//end::create_bingo_boards[]

pub fn distances(s: &Vec<(i32,i32,i32)>) -> HashMap<String,(usize,usize)>
{
    let mut dists = HashMap::new();

    let l = s.len();
    for i in 0..l {
        for j in 0..l {
            if i == j { continue; }
            let d = (s[i].0 - s[j].0 , s[i].1 - s[j].1 ,  s[i].2 - s[j].2 );
            let s = format!("{:?}",d);
            dists.insert(s,(i,j));
        }
    }
    return dists;
}

pub fn transform(s: &Vec<(i32,i32,i32)>, (x,y,z) : (i32,i32,i32) ) -> Vec<(i32,i32,i32)> {
    let mut v = Vec::new();

    for p in s {
        v.push(( p.0 + x , p.1 + y ,p.2 + z));
    }

    return v;
}

pub fn matchv(s1: &Vec<(i32,i32,i32)>,s2: &Vec<(i32,i32,i32)>) -> Option<(Vec<(i32,i32,i32)>,(i32,i32,i32))> {


    let d1 = distances(s1);
    let d2 = distances(s2);


    let set1: HashSet<String> = d1.keys().cloned().collect();
    let set2: HashSet<String> = d2.keys().cloned().collect();


    let u: HashSet<String> = set1.intersection(&set2).map(|s| s.to_string()).collect();

    //println!("matching {}",u.len());


    let mut map = HashMap::new();
    for k in u {
        
        //println!("{:?} == {:?}",d1.get(&k),d2.get(&k));

        let (i1,j1) = match d1.get(&k) { 
            Some((a,b)) => (*a,*b),
            _ => panic!("Something went wrong")
        };

        let (i2,j2) = match d2.get(&k) { 
            Some((a,b)) => (*a,*b),
            _ => panic!("Something went wrong")
        };

        match map.get(&i1) {
            Some(i) => if *i != i2 { panic!("Something went wrong {} {} {}",i,i1,i2)},
            None => { }
        };
        map.insert(j1,j2);
        match map.get(&j1) {
            Some(j) => if *j != j2 { panic!("Something went wrong {} {} {}",j,j1,j2)},
            None => {  }
        };
        map.insert(j1,j2);
        //i1 == i2
        //j1 == j2
    }

    if map.len() >= 12 {

    
        for &i in map.keys() {
            let j : usize = *map.get(&i).unwrap();
            let d = (s1[i].0 - s2[j].0 , s1[i].1 - s2[j].1 ,  s1[i].2 - s2[j].2 );
            //transform with d
            let v = transform(s2,d);
            return Some((v,d));
        }
    }


    return None;
}

pub fn matching(s1: &Scanner,s2: &Scanner) ->  Option<(Vec<(i32,i32,i32)>,(i32,i32,i32))> {

    for variant in s2.get_variants() {
        match matchv(&s1.get_coords(),&variant) {
            Some(v) => { return Some(v) },
            None => {}
        }
    }
    None
}


pub fn join_next(scanner: &mut Vec<Scanner>,centers : &mut Vec<(i32,i32,i32)>) {

    let l = scanner.len();
    for j in 1..l {

        match matching(&scanner[0],&scanner[j]) {
            Some((v,d)) => {
                //println!("join {} {} d: {:?}",i,j,d);
                centers.push(d);
                let set1 : HashSet<(i32,i32,i32)> = HashSet::from_iter(scanner[0].get_coords().iter().cloned());
                let set2 : HashSet<(i32,i32,i32)> = HashSet::from_iter(v.iter().cloned());
                let u : HashSet<(i32,i32,i32)> = set1.union(&set2).map(|c| (c.0,c.1,c.2)).collect();
                let vn : Vec<(i32,i32,i32)> = u.iter().cloned().collect();
                scanner[0] = Scanner::new(vn);
                scanner.remove(j);
                return;
            },
            None => {}
        }
    }

    panic!("Something went wrong");

}
pub fn join_all(scanner: &mut Vec<Scanner>,centers : &mut Vec<(i32,i32,i32)>) -> usize {

   
    while scanner.len() > 1 {
        println!("Try join coord system of {} scanners",scanner.len());
        join_next(scanner,centers);
    }

    scanner[0].get_coords().len()
}

pub fn manhattan(v : &Vec<(i32,i32,i32)>) -> i32 {

    let l = v.len();
    let mut max = 0;
    for i in 0..l {
        for j in i+1..l {

            let d = (v[i].0 -v[j].0).abs() + (v[i].1 -v[j].1).abs() + (v[i].2 -v[j].2).abs();

            if d > max {
                max = d;
            }
        }
    }

    return max;

}