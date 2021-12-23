#![feature(map_first_last)]
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt,
    hash::Hash,
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
pub struct Search<T> {
    heap: BTreeSet<(usize, T)>,
    settled: HashSet<T>,
    costs: HashMap<T, usize>,
}

impl<T> Search<T>
where
    T: Eq + Ord + Hash + Copy,
{
    pub fn init(cost: usize, burrow: T) -> Self {
        let mut search = Self {
            heap: BTreeSet::new(),
            settled: HashSet::new(),
            costs: HashMap::new(),
        };
        search.heap.insert((cost, burrow));
        search.settled.insert(burrow);
        search.costs.insert(burrow, cost);
        search
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        if let Some((weight, burrow)) = self.heap.pop_first() {
            self.settled.insert(burrow);
            Some((weight, burrow))
        } else {
            None
        }
    }

    pub fn push(&mut self, cost: usize, adjacent: T) -> bool {
        if self.settled.contains(&adjacent) {
            return false;
        }

        if let Some(cur_cost) = self.costs.get(&adjacent) {
            if cost < *cur_cost {
                self.heap.remove(&(*cur_cost, adjacent));
            } else {
                return false;
            }
        }

        self.heap.insert((cost, adjacent));
        self.costs.insert(adjacent, cost);

        true
    }
}

// tag::parse[]
pub fn parse(content: &str) -> Burrow {
    Burrow::parse(content)
}
// end::parse[]

pub fn solution_1(burrow: Burrow) -> usize {
    solve1(
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

pub fn settle1(burrow: Burrow) -> Option<(Burrow, usize)> {
    let mut update = burrow;
    let mut weight = 0;

    for k in 0..Burrow::LEN {
        if let Some(p) = burrow.data[k] {
            let cur_room = if k >= Burrow::HALLWAY_LEN {
                ((k - Burrow::HALLWAY_LEN) / 2) as u8
            } else {
                u8::MAX
            };

            if cur_room == p {
                // already in own room -> cannot settle
                continue;
            }

            if burrow.data[Burrow::ROOMS_1[p as usize]].is_some() {
                // no free space in room
                continue;
            }

            if let Some(p2) = burrow.data[Burrow::ROOMS_2[p as usize]] {
                if p2 != p {
                    // room is occupied by foreigner
                    continue;
                }
            }

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

                let adj_room = if k_adj >= Burrow::HALLWAY_LEN {
                    ((k_adj - Burrow::HALLWAY_LEN) / 2) as u8
                } else {
                    u8::MAX
                };

                if adj_room == p
                    && (k_adj + 1..=Burrow::ROOMS_2[p as usize])
                        .all(|k_r| burrow.data[k_r] == Some(p))
                {
                    // all below are settled
                    update.data[k] = None;
                    update.data[k_adj] = Some(p);
                    weight += steps * Burrow::ENERGIES[p as usize];
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

    if weight > 0 {
        Some((update, weight))
    } else {
        None
    }
}

pub fn solve1(burrow: Burrow, target: Burrow) -> usize {
    let mut burrow = burrow;
    let mut cost = 0;
    while let Some((update, weight)) = settle1(burrow) {
        burrow = update;
        cost += weight;
    }
    let mut search = Search::init(cost, burrow);

    while let Some((cost, burrow)) = search.pop() {
        if burrow == target {
            return cost; // target reached
        }

        for k in 0..Burrow::LEN {
            if let Some(p) = burrow.data[k] {
                let cur_room = if k >= Burrow::HALLWAY_LEN {
                    ((k - Burrow::HALLWAY_LEN) / 2) as u8
                } else {
                    u8::MAX
                };

                if cur_room == p
                    && (k + 1..=Burrow::ROOMS_2[p as usize]).all(|k_r| burrow.data[k_r] == Some(p))
                {
                    // position is settled in room and below are only the right ones
                    continue;
                }

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

                    let adj_room = if k_adj >= Burrow::HALLWAY_LEN {
                        ((k_adj - Burrow::HALLWAY_LEN) / 2) as u8
                    } else {
                        u8::MAX
                    };

                    let own_room = adj_room < u8::MAX && adj_room == p;
                    if own_room && cur_room != p {
                        // move in own room

                        if (k_adj + 1..Burrow::ROOMS_2[p as usize])
                            .any(|k_r| burrow.data[k_r].map(|p_2| p_2 != p).unwrap_or(false))
                        {
                            // foreign pods are in room
                            continue;
                        }

                        if (k_adj + 1..Burrow::ROOMS_2[p as usize])
                            .all(|k_r| burrow.data[k_r] == Some(p))
                        {
                            // all below are settled
                            let mut adjacent = burrow.clone();
                            adjacent.data[k] = None;
                            adjacent.data[k_adj] = Some(p);
                            let mut cost = cost + steps * Burrow::ENERGIES[p as usize];
                            while let Some((update, weight)) = settle1(adjacent) {
                                cost += weight;
                                adjacent = update;
                            }
                            search.push(cost, adjacent);
                        }
                        // otherwise do not create an adjacent but continue to look ahaed
                    } else if adj_room != u8::MAX {
                        // move out of room
                        if cur_room != adj_room || done[k_adj - 1] {
                            // moving in
                            continue;
                        }
                        // don't create adjacent in foreign room but continue to look ahead
                    } else if !Burrow::DOORS.contains(&k_adj) {
                        // if I am not in front of a door, this is a valid adjacent
                        let mut adjacent = burrow.clone();
                        adjacent.data[k] = None;
                        adjacent.data[k_adj] = Some(p);
                        let mut cost = cost + steps * Burrow::ENERGIES[p as usize];
                        while let Some((update, weight)) = settle1(adjacent) {
                            cost += weight;
                            adjacent = update;
                        }
                        search.push(cost, adjacent);
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

// tag::part2[]
pub fn solution_2(_burrow: Burrow) -> usize {
    todo!()
}

pub fn solve2(burrow: Burrow2, target: Burrow2) -> usize {
    let mut search = Search::init(0, burrow);

    while let Some((cost, burrow)) = search.pop() {
        if burrow == target {
            return cost; // target reached
        }

        for k in 0..Burrow2::LEN {
            if let Some(p) = burrow.data[k] {
                let mut done = [false; Burrow2::LEN];
                let mut stack = Vec::new();

                done[k] = true;
                for k_adj in Burrow2::ADJ[k] {
                    done[*k_adj] = true;
                    stack.push((1, *k_adj));
                }

                while let Some((steps, k_adj)) = stack.pop() {
                    if burrow.data[k_adj].is_some() {
                        // cannot move on top of other
                        continue;
                    }

                    // don't create adjacent in foreign room but continue to look around. I might be moving out
                    let foreign_room = k_adj >= Burrow2::HALLWAY_LEN
                        && (k_adj - Burrow2::HALLWAY_LEN) / 4 != p as usize;

                    if !Burrow2::DOORS.contains(&k_adj) && !foreign_room {
                        // if I am not in front of a door, this is a valid adjacent
                        let mut adjacent = burrow.clone();
                        adjacent.data[k] = None;
                        adjacent.data[k_adj] = Some(p);
                        search.push(cost + steps * Burrow2::ENERGIES[p as usize], adjacent);
                    }

                    for k_adj_2 in Burrow2::ADJ[k_adj] {
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
    fn test_solve1() {
        let burrows = [
            parse(CONTENT_1),
            parse(CONTENT_2),
            parse(CONTENT_3),
            parse(CONTENT_4),
            parse(CONTENT_5),
            parse(CONTENT_6),
            parse(CONTENT_7),
            parse(CONTENT_8),
        ];

        let exp_costs = [40, 400, 3030, 40, 2003, 7000, 8, 0];

        for k1 in 0..burrows.len() - 1 {
            let start = burrows[k1];
            let mut target = burrows[k1 + 1].clone();
            let mut exp_cost = exp_costs[k1];
            while let Some((update, weight)) = settle1(target) {
                exp_cost += weight;
                target = update;
            }
            let act_cost = solve1(start, target);
            assert_eq!(exp_cost, act_cost);
        }

        let mut start = burrows[0];
        let mut act_cost = 0;
        for k in 1..burrows.len() {
            let mut target = burrows[k];
            let mut exp_adder = 0;
            while let Some((update, weight)) = settle1(target) {
                exp_adder += weight;
                target = update;
            }
            let weight = solve1(start, target);
            act_cost += weight;
            start = target;
            let exp_cost = exp_costs.iter().take(k).sum::<usize>();
            assert_eq!(exp_cost + exp_adder, act_cost);
        }
        assert_eq!(exp_costs.iter().sum::<usize>(), act_cost);
    }

    #[test]
    fn test_solution_1() {
        let costs = [
            40 + 400 + 3030 + 40 + 2003 + 7000 + 8,
            400 + 3030 + 40 + 2003 + 7000 + 8,
            3030 + 40 + 2003 + 7000 + 8,
            40 + 2003 + 7000 + 8,
            2003 + 7000 + 8,
            7000 + 8,
            8,
            0,
        ];

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

        for k in (0..1).rev() {
            println!("{}", k);
            let cost = solution_1(burrows[k]);

            // println!("\n=============\n=============\n");
            // let mut target = Burrow::parse(CONTENT_8);
            // println!("{:?}", target);
            // while let Some((burrow, cost)) = parents.get(&target) {
            //     println!("{:?} reaches parent at {}", burrow, cost);
            //     target = *burrow;
            // }

            assert_eq!(costs[k], cost);
        }
    }

    #[test]
    fn test_burrow2_from() {
        assert_eq!(BURROW_2, Burrow2::from(BURROW_1));
    }

    #[test]
    fn test_step_1() {
        let start = parse(
            "#############
#.....D...A.#
###.#B#C#.###
  #A#B#C#D#
  #########",
        );
        let target = parse(
            "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########",
        );
        let cost = solve1(start, target);
        println!("{}", cost);
    }

    #[test]
    fn test_step_2() {
        let start = parse(
            "#############
#.....D.....#
###.#B#C#D###
  #A#B#C#A#
  #########",
        );
        let target = parse(
            "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########",
        );
        let cost = solve1(start, target);
        assert_eq!(9011, cost);
    }
}
// end::tests[]
