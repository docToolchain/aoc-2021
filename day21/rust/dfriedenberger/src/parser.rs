use crate::fields::Universe;

pub fn play(pos1 :&mut u32,pos2 :&mut  u32) -> u32 {


    let mut score1 = 0;
    let mut score2 = 0;

    let mut dice = 1;
    let mut rolled = 0;
    loop {

        //tree time
        let m1 = dice + (dice + 1) + (dice + 2);
        dice = (dice + 3 -1) % 100 + 1;
        rolled += 3;

        *pos1 = (*pos1 + m1 - 1) % 10 + 1;
        score1 += *pos1;


        if score1 >= 1000 { break; }

        let m2 = dice + (dice + 1) + (dice + 2);
        dice = (dice + 3 -1) % 100 + 1;
        rolled += 3;

        *pos2 = (*pos2 + m2 - 1) % 10 + 1;
        score2 += *pos2;

    }

    println!("{} * {}",score2 , rolled);
    rolled * score2

}

/*
49 universes scores min 1 max 7
2401 universes scores min 3 max 17
105987 universes scores min 6 max 20
2212448 universes scores min 8 max 20
13856556 universes scores min 10 max 20
25878972 universes scores min 13 max 20
13859125 universes scores min 15 max 20
1656396 universes scores min 17 max 20
14364 universes scores min 20 max 20
0 universes scores min 500 max 0
*/

/* 
Universe { pos1: 1, pos2: 1, score1: 1, score2: 1, copies: 6 }
Universe { pos1: 1, pos2: 1, score1: 8, score2: 3, copies: 9 }
Universe { pos1: 1, pos2: 1, score1: 8, score2: 4, copies: 54 }
Universe { pos1: 1, pos2: 1, score1: 8, score2: 5, copies: 126 }
Universe { pos1: 1, pos2: 1, score1: 8, score2: 6, copies: 126 }
Universe { pos1: 1, pos2: 1, score1: 8, score2: 7, copies: 54 }
Universe { pos1: 1, pos2: 1, score1: 8, score2: 8, copies: 9 }
Universe { pos1: 1, pos2: 1, score1: 9, score2: 3, copies: 9 }
Universe { pos1: 1, pos2: 1, score1: 9, score2: 4, copies: 54 }
Universe { pos1: 1, pos2: 1, score1: 9, score2: 5, copies: 126 }
Universe { pos1: 1, pos2: 1, score1: 9, score2: 6, copies: 126 }
Universe { pos1: 1, pos2: 1, score1: 9, score2: 7, copies: 54 }
Universe { pos1: 1, pos2: 1, score1: 9, score2: 8, copies: 9 }
Universe { pos1: 1, pos2: 1, score1: 3, score2: 3, copies: 9 }
Universe { pos1: 1, pos2: 1, score1: 3, score2: 4, copies: 54 }
Universe { pos1: 1, pos2: 1, score1: 3, score2: 5, copies: 126 }
Universe { pos1: 1, pos2: 1, score1: 3, score2: 6, copies: 126 }
Universe { pos1: 1, pos2: 1, score1: 3, score2: 7, copies: 54 }
Universe { pos1: 1, pos2: 1, score1: 3, score2: 8, copies: 9 }
Universe { pos1: 1, pos2: 1, score1: 4, score2: 3, copies: 9 }
Universe { pos1: 1, pos2: 1, score1: 4, score2: 4, copies: 54 }
Universe { pos1: 1, pos2: 1, score1: 4, score2: 5, copies: 126 }
Universe { pos1: 1, pos2: 1, score1: 4, score2: 6, copies: 126 }
Universe { pos1: 1, pos2: 1, score1: 4, score2: 7, copies: 54 }
Universe { pos1: 1, pos2: 1, score1: 4, score2: 8, copies: 9 }
*/
pub fn play2(pos1 :u32,pos2 :u32) -> (u64, u64) {

    //pos1 , pos2 , score1,score2 , copies
    let mut universes = vec![Universe::new(pos1,pos2,0,0,1)];
    let mut player1 = 0;
    let mut player2 = 0;


    loop { 


        let mut min_score = 500;
        let mut max_score = 0;

        if universes.len() == 0 { break; }
        let mut next_universes = Vec::new();

        for universe in &universes {
            //println!("{:?}",universe);
            /*
                3^3 Möglichkeiten = 27
                3 -> 1
                4 -> 3 
                5 -> 6
                6 -> 7 
                7 -> 6
                8 -> 3
                9 -> 1
            */


            for (m,copies) in vec![(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)] {
                let u = universe.roll_p1(m,copies);
                //todo check if ready 
                match u.winner() {
                    Some(p) => if p == 1 { player1 += u.get_copies() } else { player2 += u.get_copies() }
                    None => next_universes.push(u)
                } 
                
               
            }

        }

        universes = Vec::new();

        for universe in &next_universes {
            //println!("{:?}",universe);

            for (m,copies) in vec![(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)] {
                let u = universe.roll_p2(m,copies);
                //todo check if ready 
                match u.winner() {
                    Some(p) => if p == 1 { player1 += u.get_copies() } else { player2 += u.get_copies() }
                    None => {
                        let (min,max) = u.get_scores();
                        if min < min_score { min_score = min; }
                        if max > max_score { max_score = max; }
                        universes.push(u);
                    }
                } 
            }
        }

        println!("{:?} universes scores min {} max {}",universes.len(),min_score,max_score);

    }

    println!("p1: {} p2: {}",player1,player2);

    (player1,player2)

}
