//Playing fields
use array2d::Array2D;

//Array2D based field
pub struct OctopusGrid {
    array : Array2D<u32>
}

impl OctopusGrid {
    pub fn new(rows : Vec<Vec<u32>>) -> OctopusGrid
    {
        OctopusGrid {
            array : Array2D::from_rows(&rows)
        }
    }

    pub fn get(&self,x : usize,y : usize) -> u32 
    {
        return *self.array.get(x,y).unwrap();
    }

    pub fn set(&mut self,x : usize,y : usize,v : u32) 
    {
        self.array.set(x,y,v).ok();
    }

    pub fn size(&self) -> usize {
        self.array.row_len() * self.array.column_len()
    }

    pub fn incr(&mut self)
    {
        for y in 0..self.array.row_len() {
            for x in 0..self.array.column_len() {
                let v = self.get(x,y);
                self.set(x,y,v + 1);
            }
        }
    }

    pub fn start_points(&self) -> Vec<(usize,usize)> {

        let mut points = Vec::new();
        for y in 0..self.array.row_len() {
            for x in 0..self.array.column_len() {
                let v = self.get(x,y);
                if v > 9 {
                    points.push((x,y));
                }
            }
        }
        return points;
    }

    pub fn point_test_incr_flash(&mut self,flashes : &mut Array2D<bool>,x : usize,y : usize,xoff : i32,yoff : i32)
    {
        let lastx = self.array.column_len() - 1;
        let lasty = self.array.row_len() - 1;

        //test
        if xoff == -1 && x == 0 { return; }
        if xoff ==  1 && x == lastx { return; }
        if yoff == -1 && y == 0 { return; }
        if yoff ==  1 && y == lasty { return; }

        let nx : usize = (x as i32 + xoff) as usize;
        let ny : usize = (y as i32 + yoff) as usize;

        let v = self.get(nx,ny) + 1;
        self.set(nx,ny,v);

        if v > 9 {
            self.flash_point(flashes,nx,ny);
        }
        
    }

    pub fn flash_point(&mut self,flashes : &mut Array2D<bool>,x : usize,y : usize)
    {
        if *flashes.get(x,y).unwrap() {
            return;
        }
        flashes.set(x,y,true).ok();

        //incr around this point
        self.point_test_incr_flash(flashes,x,y,-1,-1);
        self.point_test_incr_flash(flashes,x,y,-1,0);
        self.point_test_incr_flash(flashes,x,y,-1,1);
        self.point_test_incr_flash(flashes,x,y,0,-1);
        self.point_test_incr_flash(flashes,x,y,0,1);
        self.point_test_incr_flash(flashes,x,y,1,-1);
        self.point_test_incr_flash(flashes,x,y,1,0);
        self.point_test_incr_flash(flashes,x,y,1,1);

    }

    pub fn flash(&mut self)
    {
        let mut flashes = Array2D::filled_with(false,self.array.column_len(),self.array.row_len());
        let sp = self.start_points();

        for (x,y) in sp {
            self.flash_point(&mut flashes,x,y);
        }
       
    }

    pub fn zero(&mut self) -> u32
    {
        let mut sum = 0;
        for y in 0..self.array.row_len() {
            for x in 0..self.array.column_len() {
                let v = self.get(x,y);
                if v > 9 {
                    sum += 1;
                    self.set(x,y,0);
                }
            }
        }
        return sum;
    }
    
}
