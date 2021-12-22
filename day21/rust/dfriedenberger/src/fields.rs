//Playing fields

#[derive(Debug)]
pub struct Universe {
    pos1 : u32,
    pos2 : u32,
    score1 : u32,
    score2 : u32,
    copies : u64
}

impl Universe {
    pub fn new(pos1 : u32,pos2 : u32,score1 : u32,score2 : u32,copies : u64) -> Universe
    {
        Universe {
            pos1 ,
            pos2,
            score1,
            score2,
            copies
        }
    }

    pub fn get_copies(&self) -> u64 {
        self.copies
    }

    pub fn get_scores(&self) -> (u32,u32) {
        if self.score1 > self.score2 {
            (self.score1 , self.score2)
        } else {
            (self.score2 , self.score1)
        }
    }

    

    pub fn winner(&self) -> Option<u32> {


        if self.score1 >= 21 {
            return Some(1);
        }
        if self.score2 >= 21 {
            return Some(2);
        }
        None
    }

    pub fn roll_p1(&self,m1 : u32,copies : u64) -> Universe {

        let next_pos1 = (self.pos1 + m1 - 1) % 10 + 1;
        let next_score1 = self.score1 + next_pos1;
        let next_copies = self.copies * copies; 
        return Universe::new(next_pos1,self.pos2,next_score1,self.score2,next_copies);

    }

    pub fn roll_p2(&self,m2 : u32,copies : u64) -> Universe {

        let next_pos2 = (self.pos2 + m2 - 1) % 10 + 1;
        let next_score2 = self.score2 + next_pos2;
        let next_copies = self.copies * copies; 
        return Universe::new(self.pos1,next_pos2,self.score1,next_score2,next_copies);

    }

}