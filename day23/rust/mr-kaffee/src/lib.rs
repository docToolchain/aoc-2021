#![feature(map_first_last)]
use std::{
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    rc::Rc,
};

// tag::burrow[]
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct BurrowLarge {
    data: [Option<u8>; 11 + 4 * 4],
}

#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub struct BurrowSmall {
    data: [Option<u8>; 11 + 4 * 2],
}

pub trait Burrow:
    std::ops::Index<usize, Output = Option<u8>>
    + std::ops::IndexMut<usize>
    + Eq
    + Ord
    + std::hash::Hash
    + Clone
    + std::fmt::Debug
{
    const ROOM_LEN: usize;
    const LEN: usize;
    const MAP: &'static [&'static [usize]];
    const MIN_COST: &'static [[usize; 4]];
    const TARGET: Self;
}

pub trait BurrowCommon {
    const HALLWAY_LEN: usize;
    const ENERGIES: [usize; 4];
    fn format(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    fn is_door(idx: usize) -> bool;
    fn get_room(idx: usize) -> Option<u8>;
    fn get_room_mn(room: u8) -> usize;
    fn get_room_mx(room: u8) -> usize;
    fn move_pod(&mut self, idx_from: usize, idx_to: usize);
    fn get_min_cost(&self) -> usize;
    fn is_deadlock(&self) -> bool;
}

impl<B> BurrowCommon for B
where
    B: Burrow,
{
    const HALLWAY_LEN: usize = 11;
    const ENERGIES: [usize; 4] = [1, 10, 100, 1000];

    fn format(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#############\n")?;
        write!(f, "#")?;
        for k in 0..Self::HALLWAY_LEN {
            match self[k] {
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
                match self[Self::HALLWAY_LEN + Self::ROOM_LEN * room + row] {
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
        assert_eq!(None, self[idx_to]);
        self[idx_to] = self[idx_from];
        self[idx_from] = None;
    }

    #[cfg(feature = "a-star")]
    fn get_min_cost(&self) -> usize {
        let mut cost = 0;
        for k in 0..Self::LEN {
            if let Some(p) = self[k] {
                // lookup minimum cost from table
                cost += Self::MIN_COST[k][p as usize];
                if let Some(r) = Self::get_room(k) {
                    if p == r
                        && (k + 1..Self::get_room_mx(r))
                            .any(|k| self[k].map(|p2| p2 != p).unwrap_or(false))
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

    #[cfg(not(feature = "a-star"))]
    fn get_min_cost(&self) -> usize {
        0
    }

    #[cfg(feature = "deadlock-check")]
    fn is_deadlock(&self) -> bool {
        // pods in hallway which cannot exchange positions
        if self[3] == Some(3) && self[5] == Some(0)
            || self[3] == Some(3) && self[7] == Some(0)
            || self[5] == Some(3) && self[7] == Some(0)
            || self[5] == Some(3) && self[7] == Some(1)
            || self[3] == Some(2) && self[5] == Some(0)
        {
            return true;
        }

        // pods in left most room which cannot reach any other room
        if self[1].is_some()
            && self[3] == Some(0)
            && (Self::get_room_mn(0)..Self::get_room_mx(0))
                .filter_map(|k| self[k])
                .max()
                .map_or(false, |mx| mx > 0)
        {
            return true;
        }

        // pods in rightmost room which cannto reach any other room
        if self[9].is_some()
            && self[7] == Some(3)
            && (Self::get_room_mn(3)..Self::get_room_mx(3))
                .filter_map(|k| self[k])
                .min()
                .map_or(false, |mn| mn < 3)
        {
            return true;
        }

        false
    }

    #[cfg(not(feature = "deadlock-check"))]
    fn is_deadlock(&self) -> bool {
        false
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
}

impl std::fmt::Debug for BurrowSmall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl std::fmt::Display for BurrowSmall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl std::ops::Index<usize> for BurrowSmall {
    type Output = Option<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for BurrowSmall {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl std::fmt::Debug for BurrowLarge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl std::fmt::Display for BurrowLarge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl std::ops::Index<usize> for BurrowLarge {
    type Output = Option<u8>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for BurrowLarge {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
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
    pub fn from(burrow: &BurrowSmall) -> Self {
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
    heap: BTreeSet<(usize, usize, Rc<T>)>,
    settled: HashSet<Rc<T>>,
    costs: HashMap<Rc<T>, (usize, usize)>,
    parents: HashMap<Rc<T>, (usize, usize, Option<Rc<T>>)>,
    trace_path: bool,
    hit_count: usize,
    decr_key_count: usize,
}

impl<T> Search<T>
where
    T: Eq + Ord + std::hash::Hash + Clone + std::fmt::Debug,
{
    pub fn init(start: T) -> Self {
        let mut search = Self {
            heap: BTreeSet::new(),
            settled: HashSet::new(),
            costs: HashMap::new(),
            parents: HashMap::new(),
            trace_path: cfg!(feature = "trace"),
            hit_count: 0,
            decr_key_count: 0,
        };
        let start = Rc::new(start);
        search.heap.insert((0, 0, Rc::clone(&start)));
        if search.trace_path {
            search.parents.insert(Rc::clone(&start), (0, 0, None));
        }
        search.costs.insert(start, (0, 0));
        search
    }

    pub fn pop(&mut self) -> Option<(usize, Rc<T>)> {
        if let Some((_, cost, burrow)) = self.heap.pop_first() {
            self.settled.insert(Rc::clone(&burrow));
            Some((cost, burrow))
        } else {
            None
        }
    }

    pub fn push(
        &mut self,
        parent: Rc<T>,
        cost: usize,
        adjacent: T,
        weight: usize,
        bound_rem: usize,
    ) -> bool {
        // wrap in Rc
        let adjacent = Rc::new(adjacent);

        // skip if settled
        if self.settled.contains(&adjacent) {
            return false;
        }

        let e = self.costs.entry(adjacent);
        let adj = Rc::clone(e.key()); // use same instance consistently
        let (b, c) = e.or_insert((usize::MAX, usize::MAX));
        if *b < usize::MAX {
            self.hit_count += 1;
            // if already in queue, check if decrease key required;
            // remove from queue if yes, otherwise return false (nothing to be added to queue)
            if cost + weight + bound_rem < *b {
                self.decr_key_count += 1;
                self.heap.remove(&(*b, *c, Rc::clone(&adj)));
            } else {
                return false;
            }
        }
        *b = cost + weight + bound_rem;
        *c = cost + weight;

        // keep trace
        if self.trace_path {
            self.parents
                .insert(Rc::clone(&adj), (bound_rem, weight, Some(parent)));
        }

        // insert to heap
        self.heap.insert((*b, *c, adj));

        true
    }

    pub fn get_path_to(&self, target: &T) -> VecDeque<(usize, usize, T)> {
        let mut path = VecDeque::new();

        let mut current = target.clone();
        while let Some((bound_rem, cost, parent)) = self.parents.get(&current) {
            path.push_front((*bound_rem, *cost, current.clone()));
            if let Some(parent) = parent {
                current = parent.as_ref().clone();
            } else {
                break;
            }
        }

        path
    }

    pub fn print_path_to(&self, target: &T) {
        let mut steps = 0;
        let mut sum = 0;
        for (bound_rem, weight, burrow) in self.get_path_to(target) {
            sum += weight;
            steps += 1;
            println!(
                "\n{}) {} -> {} (bound: {} -> {}) ==>\n{:?}",
                steps,
                weight,
                sum,
                bound_rem,
                sum + bound_rem,
                burrow
            );
        }
    }

    pub fn print_stats(&self) {
        println!(
            "Item Count\n- heap: {:9}\n- settled: {:6}\n- total: {:8}\nPush Stats\n- hit: {:10}\n- decr key: {:5}",
            self.heap.len(),
            self.settled.len(),
            self.costs.len(),
            self.hit_count,
            self.decr_key_count,
        );
    }
}
// end::search[]

// tag::solve[]
pub fn solve<B>(start: B) -> usize
where
    B: Burrow,
{
    let mut search = Search::init(start);

    while let Some((cost, burrow)) = search.pop() {
        if B::TARGET.eq(&burrow) {
            // target reached

            if cfg!(feature = "trace") {
                search.print_path_to(&B::TARGET);
                search.print_stats();
            }

            return cost;
        }

        for k in 0..B::LEN {
            if let Some(p) = burrow[k] {
                let cur_room = B::get_room(k);
                if cur_room == Some(p)
                    && (k + 1..B::get_room_mx(p)).all(|k_r| burrow[k_r] == Some(p))
                {
                    // position is settled in room (below are only the pods of the same type)
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
                    if burrow[k_adj].is_some() {
                        // cannot move on top of other
                        continue;
                    }

                    let adj_room = B::get_room(k_adj);
                    if adj_room == Some(p)
                        && (k_adj + 1..B::get_room_mx(p)).all(|k_r| burrow[k_r] == Some(p))
                    {
                        // move in own room
                        let weight = steps * B::ENERGIES[p as usize];
                        let mut adjacent = burrow.as_ref().clone();
                        adjacent.move_pod(k, k_adj);
                        let bound_rem = adjacent.get_min_cost();
                        search.push(Rc::clone(&burrow), cost, adjacent, weight, bound_rem);
                        // no need to continue to look for other adjacents
                        continue;
                    }

                    if k_adj < B::HALLWAY_LEN && !B::is_door(k_adj) && cur_room.is_some() {
                        // move from room to hallway
                        let weight = steps * B::ENERGIES[p as usize];
                        let mut adjacent = burrow.as_ref().clone();
                        adjacent.move_pod(k, k_adj);
                        if !adjacent.is_deadlock() {
                            let bound_rem = adjacent.get_min_cost();
                            search.push(Rc::clone(&burrow), cost, adjacent, weight, bound_rem);
                        }
                        // continue search, other positions in the hallway may be valid adjacents
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

pub fn solution_1(burrow: &BurrowSmall) -> usize {
    solve(burrow.clone())
}

pub fn solution_2(burrow: &BurrowSmall) -> usize {
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
            let act_cost = solution_1(&burrows[k]);
            assert_eq!(exp_costs[k], act_cost);
        }
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(44169, solution_2(&BURROW_1));
    }

    #[test]
    fn test_burrow_large_from() {
        assert_eq!(BURROW_2, BurrowLarge::from(&BURROW_1));
    }

    #[test]
    fn test_get_min_cost() {
        let burrow = BurrowLarge::from(&parse(
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
