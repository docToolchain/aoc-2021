pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::parser::create_caves;
    use crate::parser::find_ways;

    
    #[test]
    fn test_example0() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let caves = create_caves(&vec2).unwrap();

        let cnt = find_ways(&caves,1);
        assert_eq!(cnt, 10);
        //end::test1[]
        

        

        //tag::test2[]
        let cnt = find_ways(&caves,2);
        assert_eq!(cnt, 36);
        //end::test2[]

        Ok(())
    }
  
    #[test]
    fn test_example1() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "dc-end",
            "HN-start",
            "start-kj",
            "dc-start",
            "dc-HN",
            "LN-dc",
            "HN-end",
            "kj-sa",
            "kj-HN",
            "kj-dc"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let caves = create_caves(&vec2).unwrap();

        let cnt = find_ways(&caves,1);
        assert_eq!(cnt, 19);


        //end::test1[]
        

        

        //tag::test2[]
        let cnt = find_ways(&caves,2);
        assert_eq!(cnt, 103);
        //end::test2[]

        Ok(())
    }


    #[test]
    fn test_example2() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "fs-end",
            "he-DX",
            "fs-he",
            "start-DX",
            "pj-DX",
            "end-zg",
            "zg-sl",
            "zg-pj",
            "pj-he",
            "RW-he",
            "fs-DX",
            "pj-RW",
            "zg-RW",
            "start-pj",
            "he-WI",
            "zg-he",
            "pj-fs",
            "start-RW"
        ];

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let caves = create_caves(&vec2).unwrap();

        let cnt = find_ways(&caves,1);
        assert_eq!(cnt, 226);


        //end::test1[]
        

        

        //tag::test2[]
        let cnt = find_ways(&caves,2);
        assert_eq!(cnt, 3509);
        //end::test2[]

        Ok(())
    }
}