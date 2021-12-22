
fn  rot90(v : &Vec<(i32,i32,i32)>) -> Vec<(i32,i32,i32)> {
    let mut n = Vec::new();
    for c in v {
        n.push( ( c.0 , c.2 , -1 * c.1 ));
    }
    return n;
}

fn  rot180(v : &Vec<(i32,i32,i32)>) -> Vec<(i32,i32,i32)> {
    let mut n = Vec::new();
    for c in v {
        n.push( ( c.0 , -1 * c.1 , -1 * c.2 ));
    }
    return n;
}

fn  rot270(v : &Vec<(i32,i32,i32)>) -> Vec<(i32,i32,i32)> {
    let mut n = Vec::new();
    for c in v {
        n.push( ( c.0 , -1 * c.2 ,  c.1 ));
    }
    return n;
}

fn  rot(v : Vec<(i32,i32,i32)>) -> Vec<Vec<(i32,i32,i32)>> {
    let mut n = Vec::new();
    n.push(rot90(&v));
    n.push(rot180(&v));
    n.push(rot270(&v));
    n.push(v);

    return n;
}


fn  to_neg(v : &Vec<(i32,i32,i32)>) -> Vec<(i32,i32,i32)> {
    let mut n = Vec::new();
    for c in v {
        n.push( ( -1 * c.0 , -1 * c.1 , c.2 ));
    }
    return n;
}

fn  to_y(v : &Vec<(i32,i32,i32)>) -> Vec<(i32,i32,i32)> {
    let mut n = Vec::new();
    for c in v {
        n.push( ( -1 * c.1 , c.0 , c.2 ));
    }
    return n;
}

fn  to_z(v : &Vec<(i32,i32,i32)>) -> Vec<(i32,i32,i32)> {
    let mut n = Vec::new();
    for c in v {
        n.push( ( c.2 , -1 * c.1 , c.0 ));
    }
    return n;
}

//Array2D based field
pub struct Scanner {
    coords : Vec<(i32,i32,i32)>,
}

impl Scanner {
    pub fn new(coords : Vec<(i32,i32,i32)>) -> Scanner
    {
        Scanner {
            coords
        }
    }

    pub fn get_coords(&self) -> Vec<(i32,i32,i32)> {
        return self.coords.to_vec();
    }
    
    pub fn get_variants(&self) -> Vec<Vec<(i32,i32,i32)>>
    {
        let mut variants = Vec::new();


        let x = self.coords.to_vec();
        let x2 = to_neg(&self.coords);

        let y = to_y(&x);
        let y2 = to_neg(&y);

        let z = to_z(&x);
        let z2 = to_neg(&z);

        
        variants.append(&mut rot(x));
        variants.append(&mut rot(x2));
        variants.append(&mut rot(y));
        variants.append(&mut rot(y2));
        variants.append(&mut rot(z));
        variants.append(&mut rot(z2));

        return variants;
    }
  
}
