use crate::fields::HeightMap;


//tag::create_heightmap[]
pub fn create_heightmap(data: &Vec<String>) -> std::io::Result<HeightMap> {
    let l = data.len();    
   
    let mut rows = Vec::new();

    for i in 0..l {
        let line = data.get(i).unwrap();
       
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(row);
    }

    return Ok(HeightMap::new(rows))
}
//end::create_heightmap[]

