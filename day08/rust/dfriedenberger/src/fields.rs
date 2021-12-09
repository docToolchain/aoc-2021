use std::collections::HashSet;




fn split(signals : &Vec<String>,crit : &HashSet<char>) -> (Vec<String>,Vec<String>) {
    
    let mut v1 : Vec<String> = Vec::new();
    let mut v2 : Vec<String> = Vec::new();

    for signal in signals {
        let h: HashSet<_> = signal.chars().collect();

        if crit.is_subset(&h) {
            v1.push(signal.to_string());
        }
        else
        {
            v2.push(signal.to_string());
        }
    }

    return (v1,v2);
}

fn diff(a : &String,b : &String) -> HashSet<char> {
       
    let ha: HashSet<_> = a.chars().collect();
    let hb: HashSet<_> = b.chars().collect();
    return ha.symmetric_difference(&hb).map(|c| *c).collect();

}

pub struct Entry {
    signals : Vec<String>,
    digets : Vec<String>
}

impl Entry {
    pub fn new(signals : Vec<String>,digets: Vec<String>) -> Entry
    {
        Entry {
            signals : signals,
            digets : digets
        }
    }
    //tag::count_digits1478[]
    pub fn count_digits1478(&self) -> u32 {
        let mut cnt = 0;
        for diget in &self.digets {
            let l = diget.len();

            if l == 2 || l == 3 || l == 4 || l == 7 {
                cnt += 1;
            } 

        }
        return cnt;
    }
    //end::count_digits1478[]

    pub fn get(&self,len : usize) -> Vec<String> {

        let mut v : Vec<String> = Vec::new();

        for nr in &self.signals {
            if nr.len() == len {
                v.push(nr.clone());
            }
        }
        return v;
    }
    //tag::determine[]
    pub fn determine(&self) -> u32 {
        
        let n1 = self.get(2);
        let n7 = self.get(3);
        let n4 = self.get(4);
        let n235 = self.get(5);
        let n069 = self.get(6);
        let n8 = self.get(7);

        let d48 = diff(&n4[0],&n8[0]); //segments eg
        let d41 = diff(&n4[0],&n1[0]); //segments bd

        let (n2 , n35) = split(&n235,&d48);
        let (n06 , n9) = split(&n069,&d48);

        let (n5, n3) = split(&n35,&d41);
        let (n6, n0) = split(&n06,&d41);

        let mut num = 0;
        for diget in &self.digets {

            num *= 10;
            let mut z = 0;
            if n0[0] == *diget { z = 0; }
            if n1[0] == *diget { z = 1; }
            if n2[0] == *diget { z = 2; }
            if n3[0] == *diget { z = 3; }
            if n4[0] == *diget { z = 4; }
            if n5[0] == *diget { z = 5; }
            if n6[0] == *diget { z = 6; }
            if n7[0] == *diget { z = 7; }
            if n8[0] == *diget { z = 8; }
            if n9[0] == *diget { z = 9; }
            num += z;

        }
        return num;
    }
    //end::determine[]


}

