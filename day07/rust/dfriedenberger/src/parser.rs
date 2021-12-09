
//tag::create_swarm[]
pub fn create_swarm(data: &Vec<String>) -> std::io::Result<Vec<u32>> {

    let first_line = data.get(0).unwrap();
    let numbers: Vec<u32> = first_line.split(",").map(|s| s.parse().unwrap()).collect();

    return Ok(numbers)
}
//end::create_swarm[]



//tag::gaussian_sum[]
pub fn gaussian_sum(v : u32) -> u32 {
    return (v * (v+1)) / 2;
}
//end::gaussian_sum[]

pub fn linear(v : u32) -> u32 {
    return v;
}

//tag::calculate_fuel[]
pub fn calculate_fuel(data: &Vec<u32>,h : u32,fuel : fn(u32) -> u32) -> u32 {

    let mut sum = 0;
    for d in data {

        let distance = if *d < h {
             h - *d
        } else {
            *d  - h
        };
        sum += fuel(distance);
    }
    
    return sum;
}
//end::calculate_fuel[]
