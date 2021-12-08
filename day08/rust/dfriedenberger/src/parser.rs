use crate::fields::Entry;


pub fn count_digits1478(entries: &Vec<Entry>) -> u32 {
    let mut sum = 0; 
    for e in entries {
        sum += e.count_digits1478();
    }
    return sum;
}

fn sorted(s : String) -> String {

    let mut l: Vec<char> = s.chars().collect();
    l.sort_unstable();
    return l.into_iter().collect();

}

pub fn parse_entries(data: &Vec<String>) -> std::io::Result<Vec<Entry>> {

    let l = data.len();    
    let mut entries = Vec::new();

    for i in 0..l {
        let line = data.get(i).unwrap();
        let parts: Vec<String> = line.split("|").map(|s| s.trim()).map(|s| s.to_string()).collect();
        let signals: Vec<String> = parts[0].split(" ").map(|s| s.trim()).map(|s| sorted(s.to_string())).collect();
        let digets: Vec<String> = parts[1].split(" ").map(|s| s.trim()).map(|s| sorted(s.to_string())).collect();

       
        entries.push(Entry::new(signals,digets));
          
    
    }


    return Ok(entries);
}


pub fn determine(entries : &Vec<Entry>) -> u32 {
    let mut sum = 0;
    for entry in entries {
        sum += entry.determine();
    }

    return sum;
}

//end::create_bingo_boards[]

