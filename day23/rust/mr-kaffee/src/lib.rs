#![feature(map_first_last)]
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt,
};

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Burrow2 {
    //  0 .. =10 : hallway
    // 11 .. =12 : room A
    // 13 .. =14 : room B
    // 15 .. =16 : room C
    // 17 .. =19 : room D
    data: [Option<u8>; 27],
}

impl fmt::Debug for Burrow2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#############\n")?;
        write!(f, "#")?;
        for k in 0..11 {
            match self.data[k] {
                Some(0) => write!(f, "A")?,
                Some(1) => write!(f, "B")?,
                Some(2) => write!(f, "C")?,
                Some(3) => write!(f, "D")?,
                _ => write!(f, ".")?,
            };
        }
        write!(f, "#\n")?;

        write!(f, "###")?;
        match self.data[11] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[15] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[19] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[23] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "###\n")?;

        write!(f, "  #")?;
        match self.data[12] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[16] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[20] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[24] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#\n")?;

        write!(f, "  #")?;
        match self.data[13] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[17] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[21] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[25] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#\n")?;

        write!(f, "  #")?;
        match self.data[14] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[18] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[22] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[26] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#\n")?;

        write!(f, "  #########\n")
    }
}

impl Burrow2 {
    pub const LEN: usize = 27;
    pub const HALLWAY_LEN: usize = 11;
    pub const ENERGIES: [usize; 4] = [1, 10, 100, 1000];
    pub const DOORS: [usize; 4] = [2, 4, 6, 8];
    pub const ROOMS_1: [usize; 4] = [11, 15, 19, 23];
    pub const ROOMS_2: [usize; 4] = [12, 16, 20, 24];
    pub const ROOMS_3: [usize; 4] = [13, 17, 21, 25];
    pub const ROOMS_4: [usize; 4] = [14, 18, 22, 26];
    pub const ADJ: &'static [&'static [usize]] = &[
        &[1],
        &[0, 2],
        &[1, 3, 11],
        &[2, 4],
        &[3, 5, 15],
        &[4, 6],
        &[5, 7, 19],
        &[6, 8],
        &[7, 9, 23],
        &[8, 10],
        &[9],
        // room 1
        &[2, 12],
        &[11, 13],
        &[12, 14],
        &[13],
        // room 2
        &[4, 16],
        &[15, 17],
        &[16, 18],
        &[17],
        // room 3
        &[6, 20],
        &[19, 21],
        &[20, 22],
        &[21],
        // room 4
        &[8, 24],
        &[23, 25],
        &[24, 26],
        &[25],
    ];

    pub fn from(burrow: Burrow) -> Self {
        let mut data = [None; Burrow2::LEN];

        for k in 0..Burrow::HALLWAY_LEN {
            data[k] = burrow.data[k];
        }

        for k in 0..4 {
            data[Self::ROOMS_1[k]] = burrow.data[Burrow::ROOMS_1[k]];
            data[Self::ROOMS_4[k]] = burrow.data[Burrow::ROOMS_2[k]];
        }

        data[12] = Some(3);
        data[13] = Some(3);
        data[16] = Some(2);
        data[17] = Some(1);
        data[20] = Some(1);
        data[21] = Some(0);
        data[24] = Some(0);
        data[25] = Some(2);

        Burrow2 { data }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Burrow {
    //  0 .. =10 : hallway
    // 11 .. =12 : room A
    // 13 .. =14 : room B
    // 15 .. =16 : room C
    // 17 .. =19 : room D
    data: [Option<u8>; 19],
}

impl fmt::Debug for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#############\n")?;
        write!(f, "#")?;
        for k in 0..11 {
            match self.data[k] {
                Some(0) => write!(f, "A")?,
                Some(1) => write!(f, "B")?,
                Some(2) => write!(f, "C")?,
                Some(3) => write!(f, "D")?,
                _ => write!(f, ".")?,
            };
        }
        write!(f, "#\n")?;

        write!(f, "###")?;
        match self.data[11] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[13] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[15] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[17] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "###\n")?;

        write!(f, "  #")?;
        match self.data[12] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[14] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[16] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.data[18] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#\n")?;

        write!(f, "  #########\n")
    }
}

impl Burrow {
    pub const LEN: usize = 19;
    pub const HALLWAY_LEN: usize = 11;
    pub const ENERGIES: [usize; 4] = [1, 10, 100, 1000];
    pub const DOORS: [usize; 4] = [2, 4, 6, 8];
    pub const ROOMS_1: [usize; 4] = [11, 13, 15, 17];
    pub const ROOMS_2: [usize; 4] = [12, 14, 16, 18];
    pub const ADJ: &'static [&'static [usize]] = &[
        &[1],
        &[0, 2],
        &[1, 3, 11],
        &[2, 4],
        &[3, 5, 13],
        &[4, 6],
        &[5, 7, 15],
        &[6, 8],
        &[7, 9, 17],
        &[8, 10],
        &[9],
        &[2, 12],
        &[11],
        &[4, 14],
        &[13],
        &[6, 16],
        &[15],
        &[8, 18],
        &[17],
    ];

    pub fn parse(content: &str) -> Self {
        let mut lines = content.lines().skip(1);

        let mut data = [None; 19];

        for (k, c) in lines
            .next()
            .expect("No hallway")
            .chars()
            .skip(1)
            .enumerate()
        {
            data[k] = match c {
                'A' => Some(0),
                'B' => Some(1),
                'C' => Some(2),
                'D' => Some(3),
                '.' => None,
                '#' => break,
                _ => panic!("Unexpected char: {}", c),
            }
        }

        for row in 0..2 {
            let mut i = 0;
            for (_, c) in lines
                .next()
                .expect("No hallway")
                .chars()
                .skip(1)
                .enumerate()
                .filter(|(k, _)| Self::DOORS.contains(&k))
            {
                let pod = match c {
                    'A' => Some(0),
                    'B' => Some(1),
                    'C' => Some(2),
                    'D' => Some(3),
                    '.' => None,
                    _ => panic!("Unexpected character: {}", c),
                };
                data[11 + 2 * i + row] = pod;
                i += 1;
            }
        }

        Self { data }
    }
}

#[derive(Debug)]
pub struct Search {
    heap: BTreeSet<(usize, Burrow)>,
    settled: HashSet<Burrow>,
    costs: HashMap<Burrow, usize>,
}

impl Search {
    pub fn init(burrow: Burrow) -> Self {
        let mut search = Self {
            heap: BTreeSet::new(),
            settled: HashSet::new(),
            costs: HashMap::new(),
        };
        search.heap.insert((0, burrow));
        search.settled.insert(burrow);
        search.costs.insert(burrow, 0);
        search
    }

    pub fn pop(&mut self) -> Option<(usize, Burrow)> {
        if let Some((weight, burrow)) = self.heap.pop_first() {
            self.settled.insert(burrow);
            Some((weight, burrow))
        } else {
            None
        }
    }

    pub fn push(&mut self, cost: usize, weight: usize, burrow: Burrow) -> bool {
        if self.settled.contains(&burrow) {
            return false;
        }

        if let Some(cur_cost) = self.costs.get(&burrow) {
            if cost + weight < *cur_cost {
                self.heap.remove(&(*cur_cost, burrow));
            } else {
                return false;
            }
        }

        self.heap.insert((cost + weight, burrow));
        self.costs.insert(burrow, cost + weight);

        true
    }
}

// tag::parse[]
pub fn parse(content: &str) -> Burrow {
    Burrow::parse(content)
}
// end::parse[]

pub fn solution_1(burrow: Burrow) -> usize {
    solve(
        burrow,
        Burrow {
            data: [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(0),
                Some(0),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(3),
                Some(3),
            ],
        },
    )
}

// tag::part1[]
pub fn solve(burrow: Burrow, target: Burrow) -> usize {
    let mut search = Search::init(burrow);

    while let Some((cost, burrow)) = search.pop() {
        if burrow == target {
            return cost; // target reached
        }

        for k in 0..Burrow::LEN {
            if let Some(p) = burrow.data[k] {
                let mut done = [false; Burrow::LEN];
                let mut stack = Vec::new();

                done[k] = true;
                for k_adj in Burrow::ADJ[k] {
                    done[*k_adj] = true;
                    stack.push((1, *k_adj));
                }

                while let Some((steps, k_adj)) = stack.pop() {
                    if burrow.data[k_adj].is_some() {
                        // cannot move on top of other
                        continue;
                    }

                    // don't create adjacent in foreign room but continue to look around. I might be moving out
                    let foreign_room = k_adj >= Burrow::HALLWAY_LEN
                        && (k_adj - Burrow::HALLWAY_LEN) / 2 != p as usize;

                    if !Burrow::DOORS.contains(&k_adj) && !foreign_room {
                        // if I am not in front of a door, this is a valid adjacent
                        let mut adjacent = burrow.clone();
                        adjacent.data[k] = None;
                        adjacent.data[k_adj] = Some(p);
                        search.push(cost, steps * Burrow::ENERGIES[p as usize], adjacent);
                    }

                    for k_adj_2 in Burrow::ADJ[k_adj] {
                        if !done[*k_adj_2] {
                            done[*k_adj_2] = true;
                            stack.push((steps + 1, *k_adj_2));
                        }
                    }
                }
            }
        }
    }

    panic!("No path found");
}
// end::part1[]

// tag::part2[]
pub fn solution_2(_burrow: Burrow) -> usize {
    1
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT_1: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    const BURROW_1: Burrow = Burrow {
        data: [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(1),
            Some(0),
            Some(2),
            Some(3),
            Some(1),
            Some(2),
            Some(3),
            Some(0),
        ],
    };

    const BURROW_2: Burrow2 = Burrow2 {
        data: [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(1),
            Some(3),
            Some(3),
            Some(0),
            Some(2),
            Some(2),
            Some(1),
            Some(3),
            Some(1),
            Some(1),
            Some(0),
            Some(2),
            Some(3),
            Some(0),
            Some(2),
            Some(0),
        ],
    };

    #[test]
    fn test_parse() {
        let burrow = parse(CONTENT_1);
        assert_eq!(BURROW_1, burrow);
        println!("{:?}", burrow);
    }

    const CONTENT_2: &str = "#############
#...B.......#
###B#C#.#D###
  #A#D#C#A#
  #########";
    const CONTENT_3: &str = "#############
#...B.......#
###B#.#C#D###
  #A#D#C#A#
  #########";
    const CONTENT_4: &str = "#############
#.....D.....#
###B#.#C#D###
  #A#B#C#A#
  #########";
    const CONTENT_5: &str = "#############
#.....D.....#
###.#B#C#D###
  #A#B#C#A#
  #########";
    const CONTENT_6: &str = "#############
#.....D.D.A.#
###.#B#C#.###
  #A#B#C#.#
  #########";
    const CONTENT_7: &str = "#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########";
    const CONTENT_8: &str = "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";

    #[test]
    fn test_solution_1() {
        let burrows = [
            BURROW_1,
            parse(CONTENT_2),
            parse(CONTENT_3),
            parse(CONTENT_4),
            parse(CONTENT_5),
            parse(CONTENT_6),
            parse(CONTENT_7),
            parse(CONTENT_8),
        ];

        let costs = [
            8 + 7000 + 2003 + 40 + 3030 + 400 + 40,
            8 + 7000 + 2003 + 40 + 3030 + 400,
            8 + 7000 + 2003 + 40 + 3030,
            8 + 7000 + 2003 + 40,
            8 + 7000 + 2003,
            8 + 7000,
            8,
            0,
        ];

        println!("Test is target");
        assert!(burrows.last().unwrap().is_target());

        println!("5 to 6: {}", solve(burrows[4], burrows[5]));

        for (k, burrow) in burrows.iter().enumerate().rev() {
            println!("Test {}\n{:?}", k + 1, burrow);
            assert_eq!(costs[k], solution_1(burrows[k]));
        }
    }

    #[test]
    fn test_burrow2_from() {
        assert_eq!(BURROW_2, Burrow2::from(BURROW_1));
    }
}
// end::tests[]
