//tag::board[]
/// structure for a bingo board with ``data`` containing numbers, ``marked`` containing
/// flags which numbers have been drawn, ``draws_count`` holding the count af draws
/// (including draws that did not match), ``last`` holding the last number drawn, and
/// ``bingo`` is a flag indicating whether the board has at least one row or column completely
/// marked
pub struct Board {
    data: Vec<usize>,
    marked: Vec<bool>,
    count: usize,
    last: Option<usize>,
    bingo: bool,
}

impl Board {
    /// dimension of the boad
    pub const N: usize = 5;

    /// crate new board from data vector
    pub fn new(data: Vec<usize>) -> Self {
        if data.len() != Self::N * Self::N {
            panic!("Illegal data length: {}", data.len());
        }
        Board {
            data,
            marked: vec![false; Self::N * Self::N],
            count: 0,
            last: None,
            bingo: false,
        }
    }

    /// apply draws to board, stops applying draws if a row or column is entirely marked
    pub fn apply_draws(&mut self, draws: &[usize]) {
        if self.bingo {
            return;
        }

        for draw in draws {
            self.count += 1;
            self.last = Some(*draw);
            if let Some((idx, _)) = self.data.iter().enumerate().find(|(_, v)| *v == draw) {
                self.marked[idx] = true;

                let x0 = idx % Self::N;
                let y0 = idx / Self::N;

                if (0..5).all(|x| self.marked[x + Self::N * y0])
                    || (0..5).all(|y| self.marked[x0 + Self::N * y])
                {
                    self.bingo = true;
                    return;
                }
            }
        }
    }

    /// get score (sum of numbers not marked multiplied by last number drawn)
    pub fn get_score(&self) -> usize {
        if !self.bingo {
            panic!("Board not solved.");
        }

        self.data
            .iter()
            .zip(self.marked.iter())
            .filter(|(_, marked)| !*marked)
            .map(|(v, _)| v)
            .sum::<usize>()
            * self.last.expect("No draws")
    }
}
//end::board[]

//tag::parse[]
pub fn parse(input: &str) -> (Vec<Board>, Vec<usize>) {
    let mut lines = input.lines();

    // parse drawn numbers
    let draws = lines
        .next()
        .expect("No draws")
        .split(',')
        .map(|part| part.parse().expect("Could not parse number"))
        .collect();

    // parse boards
    let mut boards = Vec::new();
    let mut data = Vec::with_capacity(Board::N * Board::N);
    loop {
        let (eof, line) = lines.next().map(|line| (false, line)).unwrap_or((true, ""));
        if line.trim().len() == 0 && data.len() > 0 {
            // block complete
            boards.push(Board::new(data));
            data = Vec::with_capacity(Board::N * Board::N);
        } else {
            // add data to block
            line.split_whitespace()
                .map(|v| v.parse().expect("Could not parse number"))
                .for_each(|v| data.push(v));
        }

        if eof {
            // no more lines
            return (boards, draws);
        }
    }
}
//end::parse[]

//tag::play[]
/// play bingo
///
/// returns scores of winning and loosing board
pub fn play<'a>(boards: &'a mut [Board], draws: &[usize]) -> (usize, usize) {
    // boards that wins first and board that wins last with draw index
    let mut winner: Option<&Board> = None;
    let mut looser: Option<&Board> = None;
    for board in boards {
        board.apply_draws(&draws);

        winner = winner
            .filter(|winner| board.count >= winner.count)
            .or(Some(board));
        looser = looser
            .filter(|looser| board.count <= looser.count)
            .or(Some(board));
    }

    // unwrap results
    (
        winner.expect("No winner").get_score(),
        looser.expect("No looser").get_score(),
    )
}
//end::play[]

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

    const EXP_DRAWS: &'static [usize] = &[
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    const DATA_0: &'static [usize] = &[
        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
    ];

    #[test]
    fn test_board_new() {
        let board = Board::new(DATA_0.to_owned());
        assert_eq!(DATA_0, board.data);
        assert_eq!(vec![false; Board::N * Board::N], board.marked);
        assert_eq!(false, board.bingo);
        assert_eq!(0, board.count);
        assert_eq!(None, board.last);
    }

    #[test]
    fn test_parse() {
        let (boards, draws) = parse(CONTENT);
        assert_eq!(EXP_DRAWS, draws);
        assert_eq!(3, boards.len());
        assert_eq!(DATA_0, boards[0].data);
    }

    #[test]
    #[should_panic]
    fn test_get_score_panics() {
        let (boards, _) = parse(CONTENT);
        // this fails since no bingo yet
        boards[0].get_score();
    }

    #[test]
    fn test_board_apply_draws() {
        let (mut boards, draws) = parse(CONTENT);

        // 15th number (24) wins for board at index 2
        boards[1].apply_draws(&draws);
        assert_eq!(15, boards[1].count);
        assert_eq!(Some(13), boards[1].last);
        assert!(boards[1].bingo);

        // 12th number (13) wins for board at index 2
        boards[2].apply_draws(&draws);
        assert_eq!(12, boards[2].count);
        assert_eq!(Some(24), boards[2].last);
        assert!(boards[2].bingo);
    }

    #[test]
    fn test_play() {
        let (mut boards, draws) = parse(CONTENT);
        assert_eq!((4512, 1924), play(&mut boards, &draws));
    }
}
//end::tests[]
