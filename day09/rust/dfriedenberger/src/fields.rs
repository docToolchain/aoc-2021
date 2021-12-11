use array2d::Array2D;

//Array2D based field
pub struct HeightMap {
    array : Array2D<u32>
}

impl HeightMap {
    pub fn new(rows : Vec<Vec<u32>>) -> HeightMap
    {
        HeightMap {
            array : Array2D::from_rows(&rows)
        }
    }

    pub fn get(&self,x : usize,y : usize) -> u32 
    {
        return *self.array.get(x,y).unwrap();
    }

    pub fn is_low_point(&self,x : usize,y : usize) -> bool {

        let lx = self.array.column_len() - 1;
        let ly = self.array.row_len() - 1;
        let i = self.get(x,y);
        if 0 < x && self.get(x-1,y) <= i { return false }
        if 0 < y && self.get(x,y-1) <= i { return false }
        if x < lx && self.get(x+1,y) <= i { return false }
        if y < ly && self.get(x,y+1) <= i { return false }

        return true
    }

    pub fn basin_size(&self,visited : &mut Array2D<bool> , x : usize,y : usize) -> u32 {

        let lastx = self.array.column_len() - 1;
        let lasty = self.array.row_len() - 1;
      

        if *visited.get(x,y).unwrap() { return 0; }
        visited.set(x,y,true).ok();
        
        let i = self.get(x,y);
        if i == 9 { return 0; }

        let mut size = 1;

        if x > 0  { size += self.basin_size(visited,x-1,y) }
        if x < lastx { size += self.basin_size(visited,x+1,y) }
        if y > 0 { size += self.basin_size(visited,x,y-1) }
        if y < lasty { size += self.basin_size(visited,x,y+1) }
        
        return size;
    }

    pub fn lowest_points(&self) -> Vec<(usize,usize)> {
        let mut points = Vec::new();

        for y in 0..self.array.row_len() {
            for x in 0..self.array.column_len() {
                if self.is_low_point(x,y) {
                    points.push((x,y));
                }
            }
        }

        return points;
    }


    pub fn cnt_risk_levels(&self) -> u32
    {

        let lp = self.lowest_points();

        let mut sum = 0;
        for (x,y) in lp {

            let i = self.get(x,y);
            sum += i + 1;
        }
        return sum;
    }

    pub fn largest_basin_sizes(&self) -> u32
    {
        let mut sizes : Vec<u32> = Vec::new();
        let lp = self.lowest_points();
        for (x,y) in lp {
            let mut visited = Array2D::filled_with(false,self.array.column_len(),self.array.row_len());
            sizes.push(self.basin_size(&mut visited,x,y));
        }

        sizes.sort_by(|a, b| b.cmp(a));

        return sizes[0] * sizes[1] * sizes[2] 
    }

}

