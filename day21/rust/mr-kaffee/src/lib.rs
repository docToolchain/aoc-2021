use std::cmp;

pub fn parse(content: &str) -> (usize, usize) {
    let mut lines = content.lines();
    let a = lines
        .next()
        .expect("No player 1")
        .trim()
        .split(": ")
        .skip(1)
        .next()
        .expect("No player 1's score")
        .parse()
        .expect("Could not parse player 1's score");
    let b = lines
        .next()
        .expect("No player 2")
        .trim()
        .split(": ")
        .skip(1)
        .next()
        .expect("No player 2's score")
        .parse()
        .expect("Could not parse player 2's score");

    (a, b)
}

// tag::part1[]
/// play with a deterministic dice, winning threshold: 1000
///
/// return the looser's score times the number the dice was thrown
pub fn solution_1((mut p1, mut p2): (usize, usize)) -> u64 {
    let mut s1 = 0;
    let mut s2 = 0;

    let mut dice = 0;

    let mut t = true;
    while s1 < 1000 && s2 < 1000 {
        if t {
            p1 = (p1 + 3 * dice + 6 - 1) % 10 + 1;
            s1 += p1;
        } else {
            p2 = (p2 + 3 * dice + 6 - 1) % 10 + 1;
            s2 += p2;
        }

        dice += 3;
        t = !t;
    }

    (dice * cmp::min(s1, s2)) as u64
}
// end::part1[]

// tag::part2_stack[]
/// 7 possible outcomes with three dice
///
/// * 3 - 1: (1 1 1)
/// * 4 - 3: (1 1 2) (1 2 1) (2 1 1)
/// * 5 - 6: (1 1 3) (1 2 2) (1 3 1) (2 1 2) (2 2 1) (3 1 1)
/// * 6 - 7: (1 2 3) (1 3 2) (2 1 3) (2 2 2) (2 3 1) (3 1 2) (3 2 1)
/// * 7 - 6: (1 3 3) (2 2 3) (2 3 2) (3 1 3) (3 2 2) (3 3 1)
/// * 8 - 3: (2 3 3) (3 2 3) (3 3 2)
/// * 9 - 1: (3 3 3)
pub const DICE_MULTS: [(usize, u64); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[cfg(feature = "stack")]
/// play with dirac quantum dice (stack based)
///
/// return the number of universes won by the player who wins more often
pub fn solution_2((p1, p2): (usize, usize)) -> u64 {
    // push states with multiplicty on stack
    // since I push to the end and pop from the end, this is depth first search
    let mut stack = Vec::new();
    stack.push((p1, p2, 0, 0, true, 1));

    // win counts
    let mut w1 = 0;
    let mut w2 = 0;

    // while there are unprocessed states
    while let Some((p1, p2, s1, s2, t, mult)) = stack.pop() {
        // loop over dice outcomes with multiplicity
        for (v, n) in DICE_MULTS {
            if t {
                // player 1's turn
                // update position and score
                let p1_upd = (p1 + v - 1) % 10 + 1;
                let s1_upd = s1 + p1_upd;
                if s1_upd >= 21 {
                    // win
                    w1 += n * mult;
                } else {
                    // continue to play later
                    stack.push((p1_upd, p2, s1_upd, s2, false, n * mult));
                }
            } else {
                // player 2's turn
                // update position and score
                let p2_upd = (p2 + v - 1) % 10 + 1;
                let s2_upd = s2 + p2_upd;
                if s2_upd >= 21 {
                    // win
                    w2 += n * mult;
                } else {
                    // continue to play later
                    stack.push((p1, p2_upd, s1, s2_upd, true, n * mult));
                }
            }
        }
    }

    cmp::max(w1, w2)
}
// end::part2_stack[]

// tag::part2_list[]
#[cfg(not(feature = "stack"))]
/// play with dirac quantum dice (state list based)
///
/// return the number of universes won by the player who wins more often
pub fn solution_2((p1, p2): (usize, usize)) -> u64 {
    // index to state multiplicity list
    const F_IDX: fn(usize, usize, usize, usize, usize) -> usize =
        |p1, p2, s1, s2, t| p1 + 10 * p2 + 100 * s1 + 2100 * s2 + 44100 * t;

    // state multiplicity list
    let mut mults = vec![0; 10 * 10 * 21 * 21 * 2];

    // insert at pos_1 - 1 and pos_2 - 1 because position is zero based internally
    mults[F_IDX(p1 - 1, p2 - 1, 0, 0, 0)] = 1;

    // win counts
    let mut w1 = 0;
    let mut w2 = 0;

    for sum in 0..=40 {
        for s1 in if sum > 20 { sum - 20..=20 } else { 0..=sum } {
            let s2 = sum - s1;

            for p1 in 0..10 {
                for p2 in 0..10 {
                    let mult_1 = mults[F_IDX(p1, p2, s1, s2, 0)];
                    let mult_2 = mults[F_IDX(p1, p2, s1, s2, 1)];

                    if mult_1 == 0 && mult_2 == 0 {
                        continue; // don't loop if no need
                    }

                    for (v, n) in DICE_MULTS {
                        let p1_upd = (p1 + v) % 10;
                        let s1_upd = s1 + p1_upd + 1;
                        if s1_upd >= 21 {
                            w1 += mult_1 * n;
                        } else {
                            mults[F_IDX(p1_upd, p2, s1_upd, s2, 1)] += mult_1 * n;
                        }

                        let p2_upd = (p2 + v) % 10;
                        let s2_upd = s2 + p2_upd + 1;
                        if s2_upd >= 21 {
                            w2 += mult_2 * n;
                        } else {
                            mults[F_IDX(p1, p2_upd, s1, s2_upd, 0)] += mult_2 * n;
                        }
                    }
                }
            }
        }
    }

    cmp::max(w1, w2)
}
// end::part2_list[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_deterministic() {
        assert_eq!(739_785, solution_1((4, 8)))
    }

    #[test]
    fn test_play_dirac() {
        assert_eq!(444_356_092_776_315, solution_2((4, 8)))
    }
}
// end::tests[]
