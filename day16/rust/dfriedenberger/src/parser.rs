


pub fn hex_to_bin(c : char) -> String {

    let b = match c {
           '0' => "0000",
           '1' => "0001",
           '2' => "0010",
           '3' => "0011",
           '4' => "0100",
           '5' => "0101",
           '6' => "0110",
           '7' => "0111",
           '8' => "1000",
           '9' => "1001",
           'A' => "1010",
           'B' => "1011",
           'C' => "1100",
           'D' => "1101",
           'E' => "1110",
           'F' => "1111",
            _  => panic!("Unkown char {}",c)
        };
    
       b.to_string()
}

pub fn parse_hex(data: &Vec<String>) -> std::io::Result<Vec<u32>> {

    let first_line = data.get(0).unwrap();
    let bits: String = first_line.chars().map(|s| hex_to_bin(s)).collect();
    let v : Vec<u32> = bits.chars().map(|c| c.to_digit(10).unwrap()).collect();

    return Ok(v)
}

pub fn get_vt(data: &Vec<u32>,i : &mut usize,l : usize) -> u32 {

    let mut v = 0;
    for _x in 0..l {
        v <<= 1;
        v += data[*i];
        *i += 1;
    }
    
    return v;

}

pub fn get_literal(data: &Vec<u32>,i : &mut usize) -> i64
{
    let mut val : i64 = 0;
    loop {
        val *= 1 << 4;

        let x = data[*i];
        let v = data[*i +  1] << 3 | data[*i + 2] << 2 | data[*i + 3] << 1 | data[*i + 4];
        *i += 5;
        val += v as i64;
        if x == 0 {
            break;
        }

    }
    return val;
}


pub fn operation(typ : u32,data : &Vec<i64>) -> i64
{
    match typ {
        0 => data.iter().fold(0, |s, x| s + x),
        1 => data.iter().fold(1, |p, x| p * x),
        2 => *data.iter().min().unwrap(),
        3 => *data.iter().max().unwrap(),
        5 => if data[0] > data[1] { 1 } else { 0 },
        6 => if data[0] < data[1] { 1 } else { 0 },
        7 => if data[0] == data[1] { 1 } else { 0 },
        _ => panic!(" Unknown {}",typ)

    }

}


pub fn parse_package(data: &Vec<u32>, i : &mut usize,sum : &mut u32,maxlen : u32, maxcnt : u32) -> Vec<i64> 
{
    let l = data.len();
    let start = *i;
    let mut cnt = 0;
    let mut vec = Vec::new();
    while *i + 10 < l
    {

        //get Version
        let version = get_vt(data,i,3); 
        let typ = get_vt(data,i,3); 

        *sum += version;    

        //println!(" Version {} {} ",version,typ);
        if typ == 4 {
            let value = get_literal(data,i);
            //println!("Value {}",value);
            vec.push(value);
        } else {

            //operator
            let ltyp = get_vt(data,i,1);
            if ltyp == 0 {
                let total_len = get_vt(data,i,15);
                let values = parse_package(data,i,sum,total_len,0);
                let result = operation(typ,&values);
                vec.push(result);
                //println!("{} Bytes",total_len);
            } else {
                // 1
                let subpackets = get_vt(data,i,11);
                let values = parse_package(data,i,sum,0,subpackets);
                let result = operation(typ,&values);
                //println!("{} Packets: {:?} = {}",subpackets,values,result);
                vec.push(result);
            }

        }

        cnt += 1;
        if maxlen > 0 && *i - start >= maxlen as usize {
            break;
        }
        if maxcnt > 0 && cnt >= maxcnt {
            break;
        }
    }

    return vec;

}

pub fn add_versions(data: &Vec<u32>) -> std::io::Result<u32> {

    let mut i = 0;
    let mut sum = 0;
  
    let _v = parse_package(data,&mut i,&mut sum,0,0);

    Ok(sum)

}

pub fn calc_value(data: &Vec<u32>) -> std::io::Result<i64> {

    let mut i = 0;
    let mut sum = 0;
  
    let v = parse_package(data,&mut i,&mut sum,0,0);

    Ok(v[0])

}

