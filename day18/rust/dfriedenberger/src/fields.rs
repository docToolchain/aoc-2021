
pub struct SnailFishNumber {
    value : u32,
    left : Option<Box<SnailFishNumber>>,
    right : Option<Box<SnailFishNumber>>
}

impl SnailFishNumber {
   
    pub fn new(value : u32,left : Option<Box<SnailFishNumber>>,right : Option<Box<SnailFishNumber>>) -> SnailFishNumber
    {
        SnailFishNumber {
            value,
            left,
            right
        }
    }

    pub fn is_regular_number(&self) -> bool {

        match self.left {
            None => true,
            Some(_) => false
        }
    }

    pub fn is_pair(&self) -> bool {

        if self.is_regular_number() { return false; }

        self.deref_left().is_regular_number() && self.deref_right().is_regular_number()
    }

    pub fn flaten(&self,v : &mut Vec<u32>) {

        if self.is_regular_number() { 
            v.push(self.value);
        } else {
            self.deref_left().flaten(v);
            self.deref_right().flaten(v);
        }

    }

    pub fn magnitude(&self) -> u32 {
        if self.is_regular_number() { 
            return self.value;
        }

        return 3 * self.deref_left().magnitude() + 2 * self.deref_right().magnitude();
    }

    pub fn add(&mut self,i : usize, left_val : u32, right_val : u32,ix : &mut usize) {
        
        if self.is_regular_number() { 
            
            if *ix + 1 == i {
                self.value += left_val;
            }

            if *ix  == i + 1{
                self.value += right_val;
            }

            *ix+=1;

            return;
        } 
        self.deref_mut_left().add(i,left_val,right_val,ix);
        self.deref_mut_right().add(i,left_val,right_val,ix);
    }

    pub fn split(&mut self) -> bool {
        if self.is_regular_number() { 

            if self.value >= 10 {
                //split

                let left_val = self.value / 2;
                let right_val = (self.value + 1) / 2;

                self.value = 0;
                self.left = Some(Box::new(SnailFishNumber::new(left_val,None,None)));
                self.right = Some(Box::new(SnailFishNumber::new(right_val,None,None)));

                return true;
            }

            return false;

        }

        if self.deref_mut_left().split() {
            return true;
        }

        return self.deref_mut_right().split();
    }

    pub fn explode(&mut self,deep : u32,ix : &mut usize) -> Option<(usize,u32,u32)> {

        if self.is_regular_number() { 
            *ix+=1;
            return None;
        } 
        
        if self.deref_left().is_pair() && deep >= 4 {
            let val_left = self.deref_left().deref_left().value;
            let val_right = self.deref_left().deref_right().value;
            //explode left
            //println!("explode [{},{}]",val_left,val_right);
            self.left = Some(Box::new(SnailFishNumber::new(0,None,None)));
           
            return Some((*ix,val_left,val_right))
        }

        match self.deref_mut_left().explode(deep + 1,ix) {
            Some(e) => return Some(e),
            None => {}
        }

        if self.deref_right().is_pair() && deep >= 4 {
            let val_left = self.deref_right().deref_left().value;
            let val_right = self.deref_right().deref_right().value;
            //explode left
            //println!("explode [{},{}]",val_left,val_right);
            self.right = Some(Box::new(SnailFishNumber::new(0,None,None)));
           
            return Some((*ix,val_left,val_right))
        }

        return self.deref_mut_right().explode(deep + 1,ix);
         
    }

    pub fn deref_left(&self) -> &SnailFishNumber {
        match &self.left {
            None => panic!("Some expected"),
            Some(e) => &e
        }
    }

    pub fn deref_mut_left(&mut self) -> &mut SnailFishNumber {
        match &mut self.left {
            None => panic!("Some expected"),
            Some(e) => e
        }
    }
   
    pub fn deref_right(&self) -> &SnailFishNumber {
        match &self.right {
            None => panic!("Some expected"),
            Some(e) => &e
        }
    }

    pub fn deref_mut_right(&mut self) -> &mut SnailFishNumber {
        match &mut self.right {
            None => panic!("Some expected"),
            Some(e) => e
        }
    }
    

    pub fn to_string(&self) -> String
    {
        if self.is_regular_number() { return self.value.to_string(); } 

        let mut s = String::new();
        s.push_str("[");
        s.push_str(&self.deref_left().to_string());
        s.push_str(",");
        s.push_str(&self.deref_right().to_string());
        s.push_str("]");
        return s;
    }
}
