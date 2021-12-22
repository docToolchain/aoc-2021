pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    

    use crate::parser::create_bingo_boards;
    use crate::parser::enhance;
    

    #[test]
    fn test_create_bingo_boards() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##",
            "#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###",
            ".######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.",
            ".#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....",
            ".#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..",
            "...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....",
            "..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
            "",
            "#..#.",
            "#....",
            "##..#",
            "..#..",
            "..###"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let (algorithm,mut image) = create_bingo_boards(&vec2).unwrap();

        assert_eq!(algorithm.len(), 512);
        //end::test1[]

        
        image.dump();

        for _i in 0..2 {
            image = enhance(&algorithm,&image,'.');
            println!("-----");
            image.dump();
        }
        

        assert_eq!(image.pixels(), 35);

         //tag::test2[]

         //end::test2[]

        Ok(())
    }

    #[test]
    fn test_create_bingo_boards2() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##",
            "#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###",
            ".######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.",
            ".#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....",
            ".#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..",
            "...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....",
            "..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
            "",
            "#..#.",
            "#....",
            "##..#",
            "..#..",
            "..###"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let (algorithm,mut image) = create_bingo_boards(&vec2).unwrap();

        assert_eq!(algorithm.len(), 512);
        //end::test1[]

        

        for _i in 0..50 {
            image = enhance(&algorithm,&image,'.');
         
        }
        

        assert_eq!(image.pixels(), 3351);

         //tag::test2[]

         //end::test2[]

        Ok(())
    }
}