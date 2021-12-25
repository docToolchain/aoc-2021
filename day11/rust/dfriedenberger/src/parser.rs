use crate::fields::OctopusGrid;


//tag::create_octopus_grid[]
pub fn create_octopus_grid(data: &Vec<String>) -> std::io::Result<OctopusGrid> {

    let l = data.len();    
   
    let mut rows = Vec::new();

    for i in 0..l {
        let line = data.get(i).unwrap();
       
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(row);
    }

    return Ok(OctopusGrid::new(rows))
}
//end::create_octopus_grid[]

//tag::simulate_steps[]
pub fn simulate_steps(grid : &mut OctopusGrid,steps : u32) -> u32
{
    let mut flashes = 0;
    for _i in 0..steps {
        grid.incr();
        grid.flash();
        flashes += grid.zero();
    }
    return flashes;

}
//end::simulate_steps[]

//tag::simulate_until_superflash[]
pub fn simulate_until_superflash(grid : &mut OctopusGrid) -> u32
{
    let mut cnt = 0;
    let size = grid.size() as u32;
    loop {
        cnt += 1;
        grid.incr();
        grid.flash();
        let sum = grid.zero();
        if sum == size {
            //Superflash 8-)
            return cnt;
        }
    }

}
//end::simulate_until_superflash[]
