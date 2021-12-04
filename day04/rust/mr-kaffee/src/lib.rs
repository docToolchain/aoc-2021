//tag::board[]
/// structure for a bingo board with data containing numbers and marked containing
/// flags which numbers have been drawn
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Board {
    data: Vec<usize>,
    marked: Vec<bool>,
}

impl Board {
    /// apply draws to board
    ///
    /// return the index into the ``draws`` slice of the winning number if any as an ``Option``
    pub fn apply_draws(&mut self, draws: &[usize]) -> Option<usize> {
        let mut k = 0;
        for draw in draws {
            if let Some((idx, _)) = self.data.iter().enumerate().find(|(_, v)| *v == draw) {
                self.marked[idx] = true;

                let x0 = idx % 5;
                let y0 = idx / 5;

                if (0..5).all(|x| self.marked[x + 5 * y0])
                    || (0..5).all(|y| self.marked[x0 + 5 * y])
                {
                    return Some(k);
                }
            }
            k += 1;
        }

        None
    }

    /// sum the numbers not yet drawn
    pub fn sum_not_drawn(&self) -> usize {
        self.data
            .iter()
            .zip(self.marked.iter())
            .filter(|(_, marked)| !*marked)
            .map(|(v, _)| v)
            .sum()
    }
}
//end::board[]

//tag::parse[]
pub fn parse(input: &str) -> (Vec<Board>, Vec<usize>) {
    let mut lines = input.lines();

    let draws = lines
        .next()
        .expect("No draws")
        .split(',')
        .map(|part| part.parse().expect("Could not parse number"))
        .collect();

    let mut boards = Vec::new();
    let mut data = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim().len() == 0 && data.len() > 0 {
            boards.push(Board {
                data,
                marked: vec![false; 25],
            });
            data = Vec::new();
        } else {
            line.trim()
                .split_whitespace()
                .map(|v| v.parse().expect("Could not parse number"))
                .for_each(|v| data.push(v));
        }
    }
    if data.len() > 0 {
        boards.push(Board {
            data,
            marked: vec![false; 25],
        });
    }

    (boards, draws)
}
//end::parse[]

//tag::play[]
/// play bingo
///
/// returns a two tuples of a board index and the last number drawn for the board which wins first
/// and the board which wins last
pub fn play(boards: &mut [Board], draws: &[usize]) -> ((usize, usize), (usize, usize)) {
    // vector of tuples of board and draw indices
    let results: Vec<_> = boards
        .iter_mut()
        .map(|board| board.apply_draws(&draws).expect("Board never wins"))
        .enumerate()
        .collect();

    // find winner
    let (winner_board_idx, winner_draw_idx) = results
        .iter()
        .min_by(|(_, idx_a), (_, idx_b)| idx_a.cmp(idx_b))
        .expect("No winner");
    // find looser
    let (looser_board_idx, looser_draw_idx) = results
        .iter()
        .max_by(|(_, idx_a), (_, idx_b)| idx_a.cmp(idx_b))
        .expect("No looser");

    (
        (*winner_board_idx, draws[*winner_draw_idx]),
        (*looser_board_idx, draws[*looser_draw_idx]),
    )
}
//end::play[]

//tag::get_scores[]
/// calculate winners and loosers final scores
pub fn get_scores(boards: &mut [Board], draws: &[usize]) -> (usize, usize) {
    let ((winner_idx, winner_draw), (looser_idx, looser_draw)) = play(boards, draws);
    (
        boards[winner_idx].sum_not_drawn() * winner_draw,
        boards[looser_idx].sum_not_drawn() * looser_draw,
    )
}
//end::get_scores[]

//tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
    
 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6
    
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse() {
        let (boards, draws) = parse(CONTENT);
        assert_eq!(
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ],
            draws
        );
        assert_eq!(
            vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19
            ],
            boards[0].data
        );
    }

    #[test]
    fn test_board_apply_draws() {
        let (mut boards, draws) = parse(CONTENT);

        // 11th number wins for board at index 2
        assert_eq!(Some(11), boards[2].apply_draws(&draws));
    }

    #[test]
    fn test_play() {
        let (mut boards, draws) = parse(CONTENT);

        // board at index two wins first with with 24, board at index 1 wins last with 13
        assert_eq!(((2, 24), (1, 13)), play(&mut boards, &draws));
    }

    #[test]
    fn test_get_scores() {
        let (mut boards, draws) = parse(CONTENT);
        assert_eq!((4512, 1924), get_scores(&mut boards, &draws));
    }
}
//end::tests[]
