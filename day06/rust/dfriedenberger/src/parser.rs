
//tag::create_swarm[]
pub fn create_swarm(data: &Vec<String>) -> std::io::Result<Vec<u64>> {

    let mut swarm = vec![0,0,0,0,0,0,0,0,0];
    let first_line = data.get(0).unwrap();
    let numbers: Vec<u64> = first_line.split(",").map(|s| s.parse().unwrap()).collect();


    for n in numbers {
        let i = n as usize;
        swarm[i] += 1;
    }

    return Ok(swarm)
}
//end::create_swarm[]

//tag::incr_and_count[]
pub fn incr_and_count(data: &mut Vec<u64>,days: u32) -> u64 {

    for _d in 0..days {
        let n = data.remove(0);
        data.push(0);
        data[6] += n;
        data[8] += n;
    }
    let mut sum = 0;
    for n in data {
        sum += *n;
    }
    return sum;
}
//end::incr_and_count[]
