#![feature(map_first_last)]
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    fmt,
    hash::Hash,
};

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
    fn is_deadlock(&self) -> bool;
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

    fn is_deadlock(&self) -> bool {
        if self.get(3) == Some(3) && self.get(7) == Some(0)
            || self.get(5) == Some(3) && self.get(7) == Some(0)
            || self.get(3) == Some(3) && self.get(5) == Some(0)
            || self.get(3) == Some(2) && self.get(5) == Some(0)
            || self.get(5) == Some(3) && self.get(7) == Some(1)
        {
            return true;
        }

        if self.get(0).is_some() && self.get(1).is_some() {
            if let Some(p_mx) = (Self::get_room_mn(0)..Self::get_room_mx(0))
                .filter_map(|k| self.get(k))
                .max()
            {
                if self.get(3) == Some(0) && p_mx > 0
                    || self.get(5) == Some(0) && p_mx > 1
                    || self.get(7) == Some(0) && p_mx > 2
                {
                    return true;
                }
            }
        }

        if self.get(10).is_some() && self.get(9).is_some() {
            if let Some(p_mn) = (Self::get_room_mn(3)..Self::get_room_mx(3))
                .filter_map(|k| self.get(k))
                .min()
            {
                if self.get(7) == Some(3) && p_mn < 3
                    || self.get(5) == Some(3) && p_mn < 2
                    || self.get(3) == Some(3) && p_mn < 1
                {
                    return true;
                }
            }
        }

        false
    }
}

impl Burrow for BurrowLarge {
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

    fn get(&self, idx: usize) -> Option<u8> {
        self.data[idx]
    }

    fn set(&mut self, idx: usize, val: Option<u8>) {
        self.data[idx] = val;
    }
}

impl Burrow for BurrowSmall {
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
    pub const TARGET: Self = Self {
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
    pub const TARGET: Self = Self {
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

#[derive(Debug)]
pub struct Search<T> {
    heap: BTreeSet<(usize, T)>,
    settled: HashSet<T>,
    costs: HashMap<T, usize>,
    parents: HashMap<T, (usize, Option<T>)>,
    trace_path: bool,
}

impl<T> Search<T>
where
    T: Eq + Ord + Hash + Copy,
{
    pub fn init(start: T) -> Self {
        let mut search = Self {
            heap: BTreeSet::new(),
            settled: HashSet::new(),
            costs: HashMap::new(),
            parents: HashMap::new(),
            trace_path: false,
        };
        search.heap.insert((0, start));
        search.settled.insert(start);
        search.costs.insert(start, 0);
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

    pub fn push(&mut self, cost: usize, parent: T, weight: usize, adjacent: T) -> bool {
        if self.settled.contains(&adjacent) {
            return false;
        }

        if let Some(cur_cost) = self.costs.get(&adjacent) {
            if cost + weight < *cur_cost {
                self.heap.remove(&(*cur_cost, adjacent));
            } else {
                return false;
            }
        }

        self.heap.insert((cost + weight, adjacent));
        self.costs.insert(adjacent, cost + weight);
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
}

pub fn settle<B>(burrow: B) -> Option<(usize, B)>
where
    B: Burrow + Copy,
{
    let mut burrow = burrow;
    let mut weight = 0;

    for k in 0..B::LEN {
        if let Some(p) = burrow.get(k) {
            let cur_room = B::get_room(k);

            if cur_room == Some(p) {
                // already in own room -> cannot settle
                continue;
            }

            if burrow
                .get(B::HALLWAY_LEN + (p as usize) * B::ROOM_LEN)
                .is_some()
            {
                // no free space in room
                continue;
            }

            if (B::get_room_mn(p)..B::get_room_mx(p))
                .any(|k| burrow.get(k).map(|p2| p != p2).unwrap_or(false))
            {
                // room is occupied by foreigner
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
                    // all below are settled
                    burrow.move_pod(k, k_adj);
                    weight += steps * B::ENERGIES[p as usize];
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

    if weight > 0 {
        Some((weight, burrow))
    } else {
        None
    }
}

pub fn solve<B>(start: B, target: B) -> usize
where
    B: Burrow + Ord + Eq + Hash + Copy + fmt::Debug,
{
    // start normalized with everything settled
    let mut search = Search::init(start);

    while let Some((cost, burrow)) = search.pop() {
        if burrow == target {
            // target reached
            return cost;
        }

        let mut adjacent = burrow;
        let mut weight = 0;
        while let Some((weight_upd, update)) = settle(adjacent) {
            weight += weight_upd;
            adjacent = update;
        }
        if weight > 0 {
            search.push(cost, burrow, weight, adjacent);
            continue;
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
                        search.push(cost, burrow, weight, adjacent);
                        continue;
                    }

                    if !B::is_door(k_adj) && cur_room.is_some() {
                        // move from room to hallway
                        let weight = steps * B::ENERGIES[p as usize];
                        let mut adjacent = burrow;
                        adjacent.move_pod(k, k_adj);
                        if !adjacent.is_deadlock() {
                            search.push(cost, burrow, weight, adjacent);
                        }
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

pub fn parse(content: &str) -> BurrowSmall {
    BurrowSmall::parse(content)
}

pub fn solution_1(burrow: BurrowSmall) -> usize {
    solve(burrow, BurrowSmall::TARGET)
}

pub fn solution_2(burrow: BurrowSmall) -> usize {
    solve(BurrowLarge::from(burrow), BurrowLarge::TARGET)
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
        let cost = solve(start, target);
        assert_eq!(4008, cost);
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
        let cost = solve(start, target);
        assert_eq!(9011, cost);
    }
}
// end::tests[]
