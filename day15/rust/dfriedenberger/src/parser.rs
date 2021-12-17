use crate::fields::BingoBoard;
use array2d::Array2D;

pub fn dijeska(cave : &BingoBoard) -> i32 {

    let mut distance : Array2D<i32> = Array2D::filled_with(-1,cave.column_len(),cave.row_len());

    let mut unbesucht = Vec::new();
    unbesucht.push((0,0));

    distance.set(0,0,0).ok();
    loop {

        if unbesucht.len() == 0 {
            break;
        }

        //select shortest node of p
        unbesucht.sort_by(|(x1,y1), (x2,y2)| distance.get(*x1,*y1).unwrap().cmp(distance.get(*x2,*y2).unwrap()));

        let (x,y) = unbesucht.remove(0);
        let d = *distance.get(x,y).unwrap();
        //add neigbours if not happen
        
        //right
        if (x  + 1) < cave.column_len() {
            let nd = d + cave.get(x+1,y) as i32;
            let cd = *distance.get(x+1,y).unwrap();
            if (cd == -1 || nd < cd)
            {
                distance.set(x + 1,y,nd);
                //set vorgaenger p
                if(cd == -1)
                {
                    unbesucht.push((x+1,y));
                }
                //add to queue
            }
        }

        //left
        if x > 0 {
            let nd = d + cave.get(x-1,y) as i32;
            let cd = *distance.get(x-1,y).unwrap();
            if (cd == -1 || nd < cd)
            {
                distance.set(x - 1,y,nd);
                //set vorgaenger p
                if(cd == -1)
                {
                    unbesucht.push((x - 1,y));
                }
                //add to queue
            }
        }

        //down
        if (y  + 1) < cave.row_len() {
            let nd = d + cave.get(x,y+1) as i32;
            let cd = *distance.get(x,y+1).unwrap();
            if (cd == -1 || nd < cd)
            {
                distance.set(x,y + 1,nd);
                //set vorgaenger p
                if(cd == -1)
                {
                    unbesucht.push((x,y+1));
                }
                //add to queue
            }
        }

         //up
         if y > 0 {
            let nd = d + cave.get(x,y-1) as i32;
            let cd = *distance.get(x,y-1).unwrap();
            if (cd == -1 || nd < cd)
            {
                distance.set(x,y - 1,nd);
                //set vorgaenger p
                if(cd == -1)
                {
                    unbesucht.push((x,y-1));
                }
                //add to queue
            }
        }

    }



    return *distance.get(cave.column_len() -1,cave.row_len() - 1).unwrap();
}

//tag::create_bingo_boards[]
pub fn create_bingo_boards(data: &Vec<String>) -> std::io::Result<BingoBoard> {



    let l = data.len();    

 
    let mut rows = Vec::new();

    for i in 0..l {
        let line = data.get(i).unwrap();
        
        //parse line
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(row);

    }
    Ok(BingoBoard::new(rows))
}
//end::create_bingo_boards[]

pub fn create_octopus_grid(data: &Vec<String>) -> std::io::Result<Vec<Vec<u32>>> {

    let l = data.len();    
   
    let mut rows = Vec::new();

    for i in 0..l {
        let line = data.get(i).unwrap();
       
        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        rows.push(row);
    }
    
    //return Ok(OctopusGrid::new(rows))
    Ok(rows)
}

pub fn enlarge(cave: &BingoBoard) -> std::io::Result<BingoBoard> {

    let mut rows = Vec::new();

    for ty in 0..5 {
        
        for y in 0..cave.row_len() {
            let mut row = Vec::new();
            for tx in 0..5 {
                for x in 0..cave.column_len() {
                    let val = cave.get(x,y);

                    let mut nval = val + ty + tx;
                    if nval > 9 {
                        nval = nval - 9
                    }
                    row.push(nval);
                } 
            }
            rows.push(row);
        }
    }

    Ok(BingoBoard::new(rows))

}