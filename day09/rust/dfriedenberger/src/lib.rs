pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::parser::create_heightmap;

   

   

    #[test]
    fn test_create_heightmap() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 

        //tag::test1[]
        let heightmap = create_heightmap(&vec2).unwrap();

        assert_eq!(heightmap.cnt_risk_levels(), 15);
        //end::test1[]
        
         //tag::test2[]
         assert_eq!(heightmap.largest_basin_sizes(), 1134);
         //end::test2[]

        Ok(())
    }

}