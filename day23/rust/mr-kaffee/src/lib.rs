#![feature(map_first_last)]
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fmt,
    hash::Hash,
};

// tag::burrow[]
#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct BurrowLarge {
    data: [Option<u8>; 11 + 4 * 4],
}

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct BurrowSmall {
    data: [Option<u8>; 11 + 4 * 2],
}

pub trait Burrow {
    const ROOM_LEN: usize;
    const LEN: usize;
    const MAP: &'static [&'static [usize]];
    const MIN_COST: &'static [[usize; 4]];
    const TARGET: Self;

    fn get(&self, idx: usize) -> Option<u8>;
    fn set(&mut self, idx: usize, val: Option<u8>);
}

pub trait BurrowCommon {
    const HALLWAY_LEN: usize;
    const ENERGIES: [usize; 4];
    fn format(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
    fn is_door(idx: usize) -> bool;
    fn get_room(idx: usize) -> Option<u8>;
    fn get_room_mn(room: u8) -> usize;
    fn get_room_mx(room: u8) -> usize;
    fn move_pod(&mut self, idx_from: usize, idx_to: usize);
    fn get_min_cost(&self) -> usize;
}

impl<B> BurrowCommon for B
where
    B: Burrow,
{
    const HALLWAY_LEN: usize = 11;
    const ENERGIES: [usize; 4] = [1, 10, 100, 1000];

    fn format(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#############\n")?;
        write!(f, "#")?;
        for k in 0..Self::HALLWAY_LEN {
            match self.get(k) {
                Some(0) => write!(f, "A")?,
                Some(1) => write!(f, "B")?,
                Some(2) => write!(f, "C")?,
                Some(3) => write!(f, "D")?,
                _ => write!(f, ".")?,
            };
        }
        write!(f, "#\n")?;

        for row in 0..Self::ROOM_LEN {
            if row == 0 {
                write!(f, "###")?;
            } else {
                write!(f, "  #")?;
            }

            for room in 0..4 {
                match self.get(Self::HALLWAY_LEN + Self::ROOM_LEN * room + row) {
                    Some(0) => write!(f, "A#")?,
                    Some(1) => write!(f, "B#")?,
                    Some(2) => write!(f, "C#")?,
                    Some(3) => write!(f, "D#")?,
                    _ => write!(f, ".#")?,
                };
            }

            if row == 0 {
                write!(f, "##\n")?;
            } else {
                write!(f, "\n")?;
            }
        }
        write!(f, "  #########")
    }

    fn is_door(idx: usize) -> bool {
        idx == 2 || idx == 4 || idx == 6 || idx == 8
    }

    fn get_room(idx: usize) -> Option<u8> {
        if idx >= Self::HALLWAY_LEN {
            Some(((idx - Self::HALLWAY_LEN) / Self::ROOM_LEN) as u8)
        } else {
            None
        }
    }

    fn get_room_mn(room: u8) -> usize {
        Self::HALLWAY_LEN + room as usize * Self::ROOM_LEN
    }

    fn get_room_mx(room: u8) -> usize {
        Self::HALLWAY_LEN + (room as usize + 1) * Self::ROOM_LEN
    }

    fn move_pod(&mut self, idx_from: usize, idx_to: usize) {
        assert_eq!(None, self.get(idx_to));
        self.set(idx_to, self.get(idx_from));
        self.set(idx_from, None);
    }

    fn get_min_cost(&self) -> usize {
        let mut cost = 0;
        for k in 0..Self::LEN {
            if let Some(p) = self.get(k) {
                // lookup minimum cost from table
                cost += Self::MIN_COST[k][p as usize];
                if let Some(r) = Self::get_room(k) {
                    if p == r
                        && (k + 1..Self::get_room_mx(r))
                            .any(|k| self.get(k).map(|p2| p2 != p).unwrap_or(false))
                    {
                        // foreigners below, need to
                        // - move out (+1) and
                        // - move one to the side (+1)
                        // - and back again (*2)
                        let adder = 2
                            * (k - Self::get_room_mn(r) as usize + 2)
                            * Self::ENERGIES[p as usize];
                        cost += adder;
                    }
                }
            }
        }

        for e in Self::ENERGIES {
            cost -= Self::ROOM_LEN * (Self::ROOM_LEN - 1) / 2 * e;
        }

        cost
    }
}

impl Burrow for BurrowLarge {
    const TARGET: Self = Self {
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
            Some(0),
            Some(0),
            Some(1),
            Some(1),
            Some(1),
            Some(1),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            Some(3),
            Some(3),
        ],
    };
    const ROOM_LEN: usize = 4;
    const LEN: usize = 11 + 4 * 4;
    const MAP: &'static [&'static [usize]] = &[
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
    const MIN_COST: &'static [[usize; 4]] = &[
        // hallway
        [6, 80, 1000, 12000],
        [5, 70, 900, 11000],
        [4, 60, 800, 10000],
        [5, 50, 700, 9000],
        [6, 40, 600, 8000],
        [7, 50, 500, 7000],
        [8, 60, 400, 6000],
        [9, 70, 500, 5000],
        [10, 80, 600, 4000],
        [11, 90, 700, 5000],
        [12, 100, 800, 6000],
        // room 1
        [3, 70, 900, 11000],
        [2, 80, 1000, 12000],
        [1, 90, 1100, 13000],
        [0, 100, 1200, 14000],
        // room 2
        [7, 30, 700, 9000],
        [8, 20, 800, 10000],
        [9, 10, 900, 11000],
        [10, 0, 1000, 12000],
        // room 3
        [9, 70, 300, 7000],
        [10, 80, 200, 8000],
        [11, 90, 100, 9000],
        [12, 100, 0, 10000],
        // room 4
        [11, 90, 700, 3000],
        [12, 100, 800, 2000],
        [13, 110, 900, 1000],
        [14, 120, 1000, 0],
    ];

    fn get(&self, idx: usize) -> Option<u8> {
        self.data[idx]
    }

    fn set(&mut self, idx: usize, val: Option<u8>) {
        self.data[idx] = val;
    }
}

impl Burrow for BurrowSmall {
    const TARGET: Self = Self {
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
    };
    const ROOM_LEN: usize = 2;
    const LEN: usize = 11 + 4 * 2;
    const MAP: &'static [&'static [usize]] = &[
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
        // room 1
        &[2, 12],
        &[11],
        // room 2
        &[4, 14],
        &[13],
        // room 3
        &[6, 16],
        &[15],
        // room 4
        &[8, 18],
        &[17],
    ];
    const MIN_COST: &'static [[usize; 4]] = &[
        // hallway
        [4, 60, 800, 10000],
        [3, 50, 700, 9000],
        [2, 40, 600, 8000],
        [3, 30, 500, 7000],
        [4, 20, 400, 6000],
        [5, 30, 300, 5000],
        [6, 40, 200, 4000],
        [7, 50, 300, 3000],
        [8, 60, 400, 2000],
        [9, 70, 500, 3000],
        [10, 80, 600, 4000],
        // room 1
        [1, 50, 700, 9000],
        [0, 60, 800, 10000],
        // room 2
        [5, 10, 500, 7000],
        [6, 0, 600, 8000],
        // room 3
        [7, 50, 100, 5000],
        [8, 60, 0, 6000],
        // room 4
        [9, 70, 500, 1000],
        [10, 80, 600, 0],
    ];

    fn get(&self, idx: usize) -> Option<u8> {
        self.data[idx]
    }

    fn set(&mut self, idx: usize, val: Option<u8>) {
        self.data[idx] = val;
    }
}

impl fmt::Debug for BurrowSmall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for BurrowSmall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for BurrowLarge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Display for BurrowLarge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format(f)
    }
}

impl BurrowSmall {
    pub fn parse(content: &str) -> Self {
        let mut lines = content.lines().skip(1);

        let mut data = [None; Self::LEN];

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

        for row in 0..Self::ROOM_LEN {
            let mut i = 0;
            for (_, c) in lines
                .next()
                .expect("No hallway")
                .chars()
                .skip(1)
                .enumerate()
                .filter(|(k, _)| Self::is_door(*k))
            {
                let pod = match c {
                    'A' => Some(0),
                    'B' => Some(1),
                    'C' => Some(2),
                    'D' => Some(3),
                    '.' => None,
                    _ => panic!("Unexpected character: {}", c),
                };
                data[Self::HALLWAY_LEN + i * Self::ROOM_LEN + row] = pod;
                i += 1;
            }
        }

        Self { data }
    }
}

impl BurrowLarge {
    pub fn from(burrow: BurrowSmall) -> Self {
        let mut data = [None; Self::LEN];

        for k in 0..Self::HALLWAY_LEN {
            data[k] = burrow.data[k];
        }

        for k in 0..4 {
            data[Self::HALLWAY_LEN + k * Self::ROOM_LEN + 0] =
                burrow.data[BurrowSmall::HALLWAY_LEN + k * BurrowSmall::ROOM_LEN + 0];
            data[Self::HALLWAY_LEN + k * Self::ROOM_LEN + 3] =
                burrow.data[BurrowSmall::HALLWAY_LEN + k * BurrowSmall::ROOM_LEN + 1];
        }

        data[Self::HALLWAY_LEN + 0 * Self::ROOM_LEN + 1] = Some(3);
        data[Self::HALLWAY_LEN + 0 * Self::ROOM_LEN + 2] = Some(3);
        data[Self::HALLWAY_LEN + 1 * Self::ROOM_LEN + 1] = Some(2);
        data[Self::HALLWAY_LEN + 1 * Self::ROOM_LEN + 2] = Some(1);
        data[Self::HALLWAY_LEN + 2 * Self::ROOM_LEN + 1] = Some(1);
        data[Self::HALLWAY_LEN + 2 * Self::ROOM_LEN + 2] = Some(0);
        data[Self::HALLWAY_LEN + 3 * Self::ROOM_LEN + 1] = Some(0);
        data[Self::HALLWAY_LEN + 3 * Self::ROOM_LEN + 2] = Some(2);

        Self { data }
    }
}
// end::burrow[]

// tag::search[]
#[derive(Debug)]
pub struct Search<T> {
    heap: BTreeSet<(usize, usize, T)>,
    settled: HashSet<T>,
    costs: HashMap<T, (usize, usize)>,
    parents: HashMap<T, (usize, Option<T>)>,
    trace_path: bool,
}

impl<T> Search<T>
where
    T: Eq + Ord + Hash + Copy + fmt::Debug,
{
    pub fn init(start: T) -> Self {
        let mut search = Self {
            heap: BTreeSet::new(),
            settled: HashSet::new(),
            costs: HashMap::new(),
            parents: HashMap::new(),
            trace_path: false,
        };
        search.heap.insert((0, 0, start));
        search.settled.insert(start);
        search.costs.insert(start, (0, 0));
        search
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        if let Some((_, cost, burrow)) = self.heap.pop_first() {
            self.settled.insert(burrow);
            Some((cost, burrow))
        } else {
            None
        }
    }

    pub fn push(
        &mut self,
        cost: usize,
        parent: T,
        weight: usize,
        bound_rem: usize,
        adjacent: T,
    ) -> bool {
        if self.settled.contains(&adjacent) {
            return false;
        }

        if let Some((cur_bound, cur_cost)) = self.costs.get(&adjacent) {
            if cost + weight + bound_rem < *cur_bound {
                self.heap.remove(&(*cur_bound, *cur_cost, adjacent));
            } else {
                return false;
            }
        }

        self.heap
            .insert((cost + weight + bound_rem, cost + weight, adjacent));
        self.costs
            .insert(adjacent, (cost + weight + bound_rem, cost + weight));
        if self.trace_path {
            self.parents.insert(adjacent, (weight, Some(parent)));
        }

        true
    }

    pub fn get_path_to(&self, target: T) -> VecDeque<(usize, T)> {
        let mut path = VecDeque::new();

        let mut current = target;
        while let Some((cost, parent)) = self.parents.get(&current) {
            path.push_front((*cost, current));
            if let Some(parent) = parent {
                current = *parent;
            } else {
                break;
            }
        }

        path
    }

    pub fn print_path_to(&self, target: T) {
        for (weight, burrow) in self.get_path_to(target) {
            println!("==({})==>\n{:?}", weight, burrow);
        }
    }
}
// end::search[]

// tag::solve[]
pub fn solve<B>(start: B) -> usize
where
    B: Burrow + Ord + Eq + Hash + Copy + fmt::Debug,
{
    let mut search = Search::init(start);

    while let Some((cost, burrow)) = search.pop() {
        if burrow == B::TARGET {
            // target reached
            return cost;
        }

        for k in 0..B::LEN {
            if let Some(p) = burrow.get(k) {
                let cur_room = B::get_room(k);
                if cur_room == Some(p)
                    && (k + 1..B::get_room_mx(p)).all(|k_r| burrow.get(k_r) == Some(p))
                {
                    // position is settled in room and below are only the right ones
                    continue;
                }

                let mut done = vec![false; B::LEN];
                let mut stack = Vec::new();
                done[k] = true;
                for k_adj in B::MAP[k] {
                    done[*k_adj] = true;
                    stack.push((1, *k_adj));
                }

                while let Some((steps, k_adj)) = stack.pop() {
                    if burrow.get(k_adj).is_some() {
                        // cannot move on top of other
                        continue;
                    }

                    let adj_room = B::get_room(k_adj);
                    if adj_room == Some(p)
                        && (k_adj + 1..B::get_room_mx(p)).all(|k_r| burrow.get(k_r) == Some(p))
                    {
                        // move in own room
                        let weight = steps * B::ENERGIES[p as usize];
                        let mut adjacent = burrow;
                        adjacent.move_pod(k, k_adj);
                        search.push(cost, burrow, weight, adjacent.get_min_cost(), adjacent);
                        continue;
                    }

                    if k_adj < B::HALLWAY_LEN && !B::is_door(k_adj) && cur_room.is_some() {
                        // move from room to hallway
                        let weight = steps * B::ENERGIES[p as usize];
                        let mut adjacent = burrow;
                        adjacent.move_pod(k, k_adj);
                        search.push(cost, burrow, weight, adjacent.get_min_cost(), adjacent);
                    }

                    for k_adj_2 in B::MAP[k_adj] {
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
// end::solve[]

pub fn parse(content: &str) -> BurrowSmall {
    BurrowSmall::parse(content)
}

pub fn solution_1(burrow: BurrowSmall) -> usize {
    solve(burrow)
}

pub fn solution_2(burrow: BurrowSmall) -> usize {
    solve(BurrowLarge::from(burrow))
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
    const BURROW_1: BurrowSmall = BurrowSmall {
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

    const BURROW_2: BurrowLarge = BurrowLarge {
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
        let exp_costs = [
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
            let act_cost = solution_1(burrows[k]);
            assert_eq!(exp_costs[k], act_cost);
        }
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(44169, solution_2(BURROW_1));
    }

    #[test]
    fn test_burrow_large_from() {
        assert_eq!(BURROW_2, BurrowLarge::from(BURROW_1));
    }


    #[test]
    fn test_get_min_cost() {
        let burrow = BurrowLarge::from(parse(
            "#############    
#...........#    
###A#D#C#A###    
  #C#D#B#B#
  #########",
        ));
        println!("{:?}", burrow);
        assert_eq!(43365, burrow.get_min_cost());
        assert_eq!(0, BurrowLarge::TARGET.get_min_cost());
    }
}
// end::tests[]
