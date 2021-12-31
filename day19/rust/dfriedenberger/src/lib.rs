pub mod reader;
pub mod fields;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::reader::read_file;
    use crate::parser::create_scanners;
    use crate::parser::matching;
    use crate::parser::join_all;
    use crate::parser::manhattan;
    use std::collections::HashSet;
    use std::iter::FromIterator;


    #[test]
    fn test_create_scanners() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "--- scanner 0 ---",
            "-1,-1,1",
            "-2,-2,2",
            "-3,-3,3",
            "-2,-3,1",
            "5,6,-4",
            "8,0,7"
        ];

        let data : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let scanners = create_scanners(&data).unwrap();



        assert_eq!(scanners.len(), 1);


        let variants = scanners[0].get_variants();

        for v in &variants {
            println!("{:?}",v);
        }


        assert_eq!(variants.len(), 24);


        Ok(())
    }



    #[test]
    fn test_matching() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "--- scanner 0 ---",
            "404,-588,-901",
            "528,-643,409",
            "-838,591,734",
            "390,-675,-793",
            "-537,-823,-458",
            "-485,-357,347",
            "-345,-311,381",
            "-661,-816,-575",
            "-876,649,763",
            "-618,-824,-621",
            "553,345,-567",
            "474,580,667",
            "-447,-329,318",
            "-584,868,-557",
            "544,-627,-890",
            "564,392,-477",
            "455,729,728",
            "-892,524,684",
            "-689,845,-530",
            "423,-701,434",
            "7,-33,-71",
            "630,319,-379",
            "443,580,662",
            "-789,900,-551",
            "459,-707,401",
            "",
            "--- scanner 1 ---",
            "686,422,578",
            "605,423,415",
            "515,917,-361",
            "-336,658,858",
            "95,138,22",
            "-476,619,847",
            "-340,-569,-846",
            "567,-361,727",
            "-460,603,-452",
            "669,-402,600",
            "729,430,532",
            "-500,-761,534",
            "-322,571,750",
            "-466,-666,-811",
            "-429,-592,574",
            "-355,545,-477",
            "703,-491,-529",
            "-328,-685,520",
            "413,935,-424",
            "-391,539,-444",
            "586,-435,557",
            "-364,-763,-893",
            "807,-499,-711",
            "755,-354,-619",
            "553,889,-390"
        ];

        let data : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();
        //end::testdata[] 


        //tag::test1[]
        let scanners = create_scanners(&data).unwrap();

        assert_eq!(scanners.len(), 2);

        let (v1, _d) = matching(&scanners[0],&scanners[1]).unwrap();


        let set1 : HashSet<(i32,i32,i32)> = HashSet::from_iter(scanners[0].get_coords().iter().cloned());
        let set2 : HashSet<(i32,i32,i32)> = HashSet::from_iter(v1.iter().cloned());
        let intersection : HashSet<(i32,i32,i32)> = set1.intersection(&set2).map(|c| (c.0,c.1,c.2)).collect();
        assert_eq!(intersection.len(), 12);

        Ok(())
    }

    #[test]
    fn test_data() -> Result<(), String> {

       let data = read_file("input_test.txt").unwrap();
   
       let mut scanners = create_scanners(&data).unwrap();
   
       let mut centers = Vec::new();
       centers.push((0,0,0));
       let cnt = join_all(&mut scanners,&mut centers);
       assert_eq!(79,cnt);
     
       let cnt = manhattan(&centers);
   
       assert_eq!(3621,cnt);
       Ok(())

    }
}