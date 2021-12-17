use crate::fields::Caves;
use std::collections::HashMap;


//end::create_bingo_boards[]
pub fn create_caves(data: &Vec<String>) -> std::io::Result<Caves> {


    let l = data.len();
    let mut rows = Vec::new();
    for i in 0..l {
        let line = data.get(i).unwrap();
      
        //parse line
        let row: Vec<String> = line.split('-')
                                     .map(|s| s.trim())    
                                     .filter(|s| !s.is_empty())
                                     .map(|s| s.to_string()).collect();
        rows.push(row);

    }
    
    
    return Ok(Caves::new(rows));

}

pub fn max_lower_case_cavevisits(visited_org : &HashMap<String,usize>) -> usize {

    let mut max = 0;

    for (name,val) in visited_org
    {
        if name.chars().all(|x| 'a' <= x && x <= 'z') {
            //is lower
            if *val > max 
            {
                max = *val;
            }
         }
    }

    //println!("max {:?} {}",visited_org,max);
    return max;
}

pub fn copy_visited(visited_org : &HashMap<String,usize>) -> HashMap<String,usize>
{
    let mut visited : HashMap<String,usize> = HashMap::new();

    for (name,val) in visited_org
    {
        visited.insert(name.to_string(),*val);
    }

    return visited;
}

pub fn find_pathes(caves : &Caves,visited_org : &HashMap<String,usize>,node : &String,max_visits : usize) -> u32 {

    let mut sum = 0;

    if *node == "end" {
        //println!("{:?} [cnt]", visited_org);
        return 1;
    }

    for next in caves.getNext(node) 
    {
        let mut visited : HashMap<String,usize> = copy_visited(visited_org);

        let visits : usize = if visited.contains_key(&next)  { *visited.get(&next).unwrap() } else {0};
   

        if next.chars().all(|x| 'a' <= x && x <= 'z') {
            //is lower
            if visits >= max_visits {
                continue;
            }

            //if other lower case cave was visited twice
            if max_lower_case_cavevisits(visited_org) >= 2 && visits >= 1 {
                continue;
            }

            if next == "start" {
                continue;
            }

        }

        visited.insert(next.to_string(),visits +  1);
        sum += find_pathes(caves,&visited,&next,max_visits);
    }

    return sum;

}

pub fn find_ways(caves : &Caves,max_visits : usize) -> u32 {


    let start = String::from("start");
    let mut visited : HashMap<String,usize> = HashMap::new();
    visited.insert(start.to_string(),1);

    return find_pathes(caves,&visited,&start,max_visits);

}

