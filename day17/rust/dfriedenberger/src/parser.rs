

pub fn flight_to(xmin : i32,xmax: i32,ymin: i32,ymax: i32,vx: i32,vy: i32) -> Option<i32> {

    let mut x = 0;
    let mut y = 0;
    let mut vxc = vx;
    let mut vyc = vy;

    let mut max_y = 0;
    loop {

        x = x + vxc;
        y = y + vyc;

        //max height
        if y > max_y { max_y = y; }

        if vxc > 0 { vxc -= 1; }
        vyc  -= 1;

        if xmin <= x && x <= xmax && ymin <= y && y <= ymax {
            //target area 
            return Some(max_y);
        }

        if x > xmax { break;}
        if y < ymin { break; }
    }

    None
}


//tag::style_shot[]
pub fn style_shot(xmin : i32,xmax :i32,ymin : i32,ymax :i32) -> std::io::Result<i32> {

    let mut max_height = 0;

    for vx in 1..xmax+1 {
        for vy in 1..1000 {
            match flight_to(xmin,xmax,ymin,ymax,vx,vy) {
                Some(height) => {
                    if height > max_height {
                        max_height = height;
                    }
                },
                None => {} //ignore
            }

        }
    }

    Ok(max_height)
}
//end::style_shot[]
pub fn within_target(xmin : i32,xmax :i32,ymin : i32,ymax :i32) -> std::io::Result<i32> {


    let mut cnt = 0;

    for vx in 1..xmax+1 {
        for vy in -300..1000 {
            match flight_to(xmin,xmax,ymin,ymax,vx,vy) {
                Some(_) => { 
                    cnt += 1; 
                },
                None => {} //ignore
            }
        }
    }

    Ok(cnt)
}
