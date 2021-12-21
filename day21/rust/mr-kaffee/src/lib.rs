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
pub fn solution_1((mut pos_1, mut pos_2): (usize, usize)) -> usize {
    let mut score_1 = 0;
    let mut score_2 = 0;

    let mut dice = 0;

    let mut turn_1 = true;
    while score_1 < 1000 && score_2 < 1000 {
        if turn_1 {
            pos_1 = (pos_1 + 3 * dice + 6 - 1) % 10 + 1;
            score_1 += pos_1;
        } else {
            pos_2 = (pos_2 + 3 * dice + 6 - 1) % 10 + 1;
            score_2 += pos_2;
        }

        dice += 3;
        turn_1 = !turn_1;
    }

    dice * cmp::min(score_1, score_2)
}
// end::part1[]

// tag::part2_stack[]
#[cfg(feature = "stack")]
/// play with dirac quantum dice (stack based)
///
/// return the number of universes won by the player who wins more often
pub fn solution_2((pos_1, pos_2): (usize, usize)) -> usize {
    // 7 possible outcomes with three dice
    // 3 - 1      (1 1 1)
    // 4 - 3      (1 1 2) (1 2 1) (2 1 1)
    // 5 - 6      (1 1 3) (1 2 2) (1 3 1) (2 1 2) (2 2 1) (3 1 1)
    // 6 - 7      (1 2 3) (1 3 2) (2 1 3) (2 2 2) (2 3 1) (3 1 2) (3 2 1)
    // 7 - 6      (1 3 3) (2 2 3) (2 3 2) (3 1 3) (3 2 2) (3 3 1)
    // 8 - 3      (2 3 3) (3 2 3) (3 3 2)
    // 9 - 1      (3 3 3)

    // push states with multiplicty on stack
    // since I push to the end and pop from the end, this is depth first search
    let mut stack = Vec::new();
    stack.push((pos_1, pos_2, 0, 0, 1, true));

    // counters for wins
    let mut wins_1 = 0;
    let mut wins_2 = 0;

    let mut max_stack_size = 1;
    let mut rounds = 0;

    // while there are unprocessed states
    while let Some((pos_1, pos_2, score_1, score_2, mult_state, turn_1)) = stack.pop() {
        // loop over dice outcomes with multiplicity
        for (dice, mult_dice) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            if turn_1 {
                // player 1's turn
                // update position and score
                let pos = (pos_1 + dice - 1) % 10 + 1;
                let score = score_1 + pos;
                if score >= 21 {
                    // win
                    wins_1 += mult_dice * mult_state;
                } else {
                    // continue to play later
                    stack.push((pos, pos_2, score, score_2, mult_dice * mult_state, false));
                }
            } else {
                // player 2's turn
                // update position and score
                let pos = (pos_2 + dice - 1) % 10 + 1;
                let score = score_2 + pos;
                if score >= 21 {
                    // win
                    wins_2 += mult_dice * mult_state;
                } else {
                    // continue to play later
                    stack.push((pos_1, pos, score_1, score, mult_dice * mult_state, true));
                }
            }
        }

        max_stack_size = cmp::max(max_stack_size, stack.len());
        rounds += 1;
    }

    println!("Max stack size in {} rounds: {}", rounds, max_stack_size);

    cmp::max(wins_1, wins_2)
}
// end::part2_stack[]

// tag::part2_list[]
#[cfg(not(feature = "stack"))]
/// play with dirac quantum dice (state list based)
///
/// return the number of universes won by the player who wins more often
pub fn solution_2((pos_1, pos_2): (usize, usize)) -> usize {
    let f_idx = |pos_1: usize, pos_2: usize, score_1: usize, score_2: usize, turn: usize| {
        pos_1 + 10 * pos_2 + 100 * score_1 + 2100 * score_2 + 44100 * turn
    };

    let mut state_mults = vec![0; 10 * 10 * 21 * 21 * 2];
    // insert at pos_1 - 1 and pos_2 - 1 because I use zero based position internally
    state_mults[f_idx(pos_1 - 1, pos_2 - 1, 0, 0, 0)] = 1;

    let mut wins_1 = 0;
    let mut wins_2 = 0;

    for sum_score in 0..=40 {
        for score_1 in 0..=cmp::min(20, sum_score) {
            let score_2 = sum_score - score_1;
            if score_2 > 20 {
                continue;
            }

            for pos_1 in 0..10 {
                for pos_2 in 0..10 {
                    let state_mult_1 = state_mults[f_idx(pos_1, pos_2, score_1, score_2, 0)];
                    let state_mult_2 = state_mults[f_idx(pos_1, pos_2, score_1, score_2, 1)];

                    if state_mult_1 == 0 && state_mult_2 == 0 {
                        continue; // don't loop if no need
                    }

                    for (dice, dice_mult) in
                        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)]
                    {
                        let pos_1_upd = (pos_1 + dice) % 10;
                        let score_1_upd = score_1 + pos_1_upd + 1;
                        if score_1_upd >= 21 {
                            wins_1 += state_mult_1 * dice_mult;
                        } else {
                            state_mults[f_idx(pos_1_upd, pos_2, score_1_upd, score_2, 1)] +=
                                state_mult_1 * dice_mult;
                        }

                        let pos_2_upd = (pos_2 + dice) % 10;
                        let score_2_upd = score_2 + pos_2_upd + 1;
                        if score_2_upd >= 21 {
                            wins_2 += state_mult_2 * dice_mult;
                        } else {
                            state_mults[f_idx(pos_1, pos_2_upd, score_1, score_2_upd, 0)] +=
                                state_mult_2 * dice_mult;
                        }
                    }
                }
            }
        }
    }

    cmp::max(wins_1, wins_2)
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
