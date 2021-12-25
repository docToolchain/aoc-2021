pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
    use crate::parser::create_octopus_grid;
    use crate::parser::simulate_steps;
    use crate::parser::simulate_until_superflash;
    
    #[test]
    fn test_one_step_for_simple_grid() -> Result<(), String> {

        let vec1 = vec![
            "11111",
            "19991",
            "19191",
            "19991",
            "11111"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        
        let mut board = create_octopus_grid(&vec2).unwrap();

        let cnt = simulate_steps(&mut board,1);
        assert_eq!(cnt, 9);

        Ok(())

    }

    #[test]
    fn test_star1_and_star2() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let mut grid = create_octopus_grid(&vec2).unwrap();

        let cnt = simulate_steps(&mut grid,100);
        assert_eq!(cnt, 1656);
        //end::test1[]
        

        

         //tag::test2[]
         let mut grid = create_octopus_grid(&vec2).unwrap();

         let steps = simulate_until_superflash(&mut grid);
         assert_eq!(steps, 195);
         //end::test2[]

        Ok(())
    }

}