pub mod reader;
pub mod parser;

#[cfg(test)]
mod tests {
    
 
    use crate::parser::create_swarm;
    use crate::parser::calculate_fuel;
    use crate::parser::gaussian_sum;
    use crate::parser::linear;

    #[test]
    fn test_swarm() -> Result<(), String> {

        //tag::testdata[]
        let vec1 = vec![
            "16,1,2,0,4,2,7,1,2,14"
        ];
        //end::testdata[] 

        let vec2 : Vec<String> = vec1.into_iter().map(|e| String::from(e)).collect();

        //tag::test1[]
        let swarm = create_swarm(&vec2).unwrap();
        let cnt = calculate_fuel(&swarm,2,linear);
        assert_eq!(cnt, 37);

        let cnt = calculate_fuel(&swarm,10,linear);
        assert_eq!(cnt, 71);

        let mut min = 0;
        for h in 0..10 {
            let cnt = calculate_fuel(&swarm,h,linear);
            println!("sum {}",cnt);
    
            if min == 0 || cnt < min {
                min = cnt;
            }
        }
        assert_eq!(min, 37);

        //end::test1[]
        

        

         //tag::test2[]
         let mut min = 0;
         for h in 0..10 {
            let cnt = calculate_fuel(&swarm,h,gaussian_sum);
            println!("sum {}",cnt);
            if min == 0 || cnt < min {
                min = cnt;
            }
        }
        assert_eq!(min, 168);
         //end::test2[]

        Ok(())
    }

}