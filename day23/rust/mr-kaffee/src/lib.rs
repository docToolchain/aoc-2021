#![feature(map_first_last)]
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fmt,
};

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub struct Burrow {
    hallway: [Option<u8>; 11],
    rooms: [[Option<u8>; 2]; 4],
}

impl fmt::Debug for Burrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#############\n")?;
        write!(f, "#")?;
        for k in 0..11 {
            match self.hallway[k] {
                Some(0) => write!(f, "A")?,
                Some(1) => write!(f, "B")?,
                Some(2) => write!(f, "C")?,
                Some(3) => write!(f, "D")?,
                _ => write!(f, ".")?,
            };
        }
        write!(f, "#\n")?;

        write!(f, "###")?;
        match self.rooms[0][0] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.rooms[1][0] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.rooms[2][0] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.rooms[3][0] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "###\n")?;

        write!(f, "  #")?;
        match self.rooms[0][1] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.rooms[1][1] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.rooms[2][1] {
            Some(0) => write!(f, "A")?,
            Some(1) => write!(f, "B")?,
            Some(2) => write!(f, "C")?,
            Some(3) => write!(f, "D")?,
            _ => write!(f, ".")?,
        };
        write!(f, "#")?;
        match self.rooms[3][1] {
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
    pub const ENERGIES: [usize; 4] = [1, 10, 100, 1000];
    pub const DOORS: [usize; 4] = [2, 4, 6, 8];

    pub fn parse(content: &str) -> Self {
        let mut lines = content.lines().skip(1);

        let mut hallway = [None; 11];
        let mut rooms = [[None; 2]; 4];

        for (k, c) in lines
            .next()
            .expect("No hallway")
            .chars()
            .skip(1)
            .enumerate()
        {
            hallway[k] = match c {
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
            for (k, c) in lines
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

                let idx = Self::DOORS
                    .iter()
                    .position(|door| &k == door)
                    .expect("Unexpected position");
                rooms[idx][row] = pod;
            }
        }

        Self { hallway, rooms }
    }

    pub fn is_target(&self) -> bool {
        self.rooms
            .iter()
            .enumerate()
            .all(|(k, room)| room[0] == Some(k as u8) && room[1] == Some(k as u8))
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

// tag::part1[]
pub fn solution_1(burrow: Burrow) -> usize {
    let mut search = Search::init(burrow);

    while let Some((cost, pods)) = search.pop() {
        if pods.is_target() {
            return cost; // target reached
        }

        for k in 0..11 {
            // "for each hallway position"
            if let Some(p) = burrow.hallway[k] {
                // move in hallway to the left
                let mut k_upd = k - 1;
                while k_upd < 11 && burrow.hallway[k_upd].is_none() {
                    if !Burrow::DOORS.contains(&k_upd) {
                        let mut adjacent = burrow.clone();
                        adjacent.hallway[k] = None;
                        adjacent.hallway[k_upd] = Some(p);
                        search.push(cost, (k - k_upd) * Burrow::ENERGIES[p as usize], adjacent);
                    } else if k_upd == Burrow::DOORS[p as usize]
                        && burrow.rooms[p as usize][0].is_none()
                    {
                        if burrow.rooms[p as usize][1] == None {
                            let mut adjacent = burrow.clone();
                            adjacent.hallway[k] = None;
                            adjacent.rooms[p as usize][1] = Some(p);
                            search.push(
                                cost,
                                (k - k_upd + 2) * Burrow::ENERGIES[p as usize],
                                adjacent,
                            );
                        } else if burrow.rooms[p as usize][1] == Some(p) {
                            let mut adjacent = burrow.clone();
                            adjacent.hallway[k] = None;
                            adjacent.rooms[p as usize][0] = Some(p);
                            search.push(
                                cost,
                                (k - k_upd + 1) * Burrow::ENERGIES[p as usize],
                                adjacent,
                            );
                        }
                    }
                    k_upd = k_upd.wrapping_sub(1);
                }

                // move in hallway to the right
                let mut k_upd = k + 1;
                while k_upd < 11 && burrow.hallway[k_upd].is_none() {
                    if !Burrow::DOORS.contains(&k_upd) {
                        let mut adjacent = burrow.clone();
                        adjacent.hallway[k] = None;
                        adjacent.hallway[k_upd] = Some(p);
                        search.push(cost, (k_upd - k) * Burrow::ENERGIES[p as usize], adjacent);
                    } else if k_upd == Burrow::DOORS[p as usize]
                        && burrow.rooms[p as usize][0].is_none()
                    {
                        if burrow.rooms[p as usize][1] == None {
                            let mut adjacent = burrow.clone();
                            adjacent.hallway[k] = None;
                            adjacent.rooms[p as usize][1] = Some(p);
                            search.push(
                                cost,
                                (k_upd - k + 2) * Burrow::ENERGIES[p as usize],
                                adjacent,
                            );
                        } else if burrow.rooms[p as usize][1] == Some(p) {
                            let mut adjacent = burrow.clone();
                            adjacent.hallway[k] = None;
                            adjacent.rooms[p as usize][0] = Some(p);
                            search.push(
                                cost,
                                (k_upd - k + 1) * Burrow::ENERGIES[p as usize],
                                adjacent,
                            );
                        }
                    }
                    k_upd += 1;
                }
            }
        } // end "for each hallway position"

        for r in 0..4 {
            // "for each room"
            if burrow.rooms[r][0].is_none() {
                if let Some(p) = burrow.rooms[r][1] {
                    if p != r as u8 {
                        // can move out of rooms[r][1]

                        let k = Burrow::DOORS[r]; // start pos on hallway

                        // move in hallway to the left
                        let mut k_upd = k - 1;
                        while k_upd < 11 && burrow.hallway[k_upd].is_none() {
                            if !Burrow::DOORS.contains(&k_upd) {
                                let mut adjacent = burrow.clone();
                                adjacent.rooms[r][1] = None;
                                adjacent.hallway[k_upd] = Some(p);
                                search.push(
                                    cost,
                                    (k - k_upd + 2) * Burrow::ENERGIES[p as usize],
                                    adjacent,
                                );
                            } else if k_upd == Burrow::DOORS[p as usize]
                                && burrow.rooms[p as usize][0].is_none()
                            {
                                if burrow.rooms[p as usize][1] == None {
                                    let mut adjacent = burrow.clone();
                                    adjacent.rooms[r][1] = None;
                                    adjacent.rooms[p as usize][1] = Some(p);
                                    search.push(
                                        cost,
                                        (k - k_upd + 4) * Burrow::ENERGIES[p as usize],
                                        adjacent,
                                    );
                                } else if burrow.rooms[p as usize][1] == Some(p) {
                                    let mut adjacent = burrow.clone();
                                    adjacent.rooms[r][1] = None;
                                    adjacent.rooms[p as usize][0] = Some(p);
                                    search.push(
                                        cost,
                                        (k - k_upd + 3) * Burrow::ENERGIES[p as usize],
                                        adjacent,
                                    );
                                }
                            }
                            k_upd = k_upd.wrapping_sub(1);
                        }

                        // move in hallway to the right
                        let mut k_upd = k + 1;
                        while k_upd < 11 && burrow.hallway[k_upd].is_none() {
                            if !Burrow::DOORS.contains(&k_upd) {
                                let mut adjacent = burrow.clone();
                                adjacent.rooms[r][1] = None;
                                adjacent.hallway[k_upd] = Some(p);
                                search.push(
                                    cost,
                                    (k_upd - k + 2) * Burrow::ENERGIES[p as usize],
                                    adjacent,
                                );
                            } else if k_upd == Burrow::DOORS[p as usize]
                                && burrow.rooms[p as usize][0].is_none()
                            {
                                if burrow.rooms[p as usize][1] == None {
                                    let mut adjacent = burrow.clone();
                                    adjacent.rooms[r][1] = None;
                                    adjacent.rooms[p as usize][1] = Some(p);
                                    search.push(
                                        cost,
                                        (k_upd - k + 4) * Burrow::ENERGIES[p as usize],
                                        adjacent,
                                    );
                                } else if burrow.rooms[p as usize][1] == Some(p) {
                                    let mut adjacent = burrow.clone();
                                    adjacent.rooms[r][1] = None;
                                    adjacent.rooms[p as usize][0] = Some(p);
                                    search.push(
                                        cost,
                                        (k_upd - k + 3) * Burrow::ENERGIES[p as usize],
                                        adjacent,
                                    );
                                }
                            }
                            k_upd += 1;
                        }
                    }
                }
            } else if burrow.rooms[r][0] != Some(r as u8) || burrow.rooms[r][1] != Some(r as u8) {
                let p = burrow.rooms[r][0].unwrap();
                // can move out of rooms[r][0]

                let k = Burrow::DOORS[r]; // start pos on hallway

                // move in hallway to the left
                let mut k_upd = k - 1;
                while k_upd < 11 && burrow.hallway[k_upd].is_none() {
                    if !Burrow::DOORS.contains(&k_upd) {
                        let mut adjacent = burrow.clone();
                        adjacent.rooms[r][0] = None;
                        adjacent.hallway[k_upd] = Some(p);
                        search.push(
                            cost,
                            (k - k_upd + 1) * Burrow::ENERGIES[p as usize],
                            adjacent,
                        );
                    } else if k_upd == Burrow::DOORS[p as usize]
                        && burrow.rooms[p as usize][0].is_none()
                        && p != r as u8
                    {
                        if burrow.rooms[p as usize][1] == None {
                            let mut adjacent = burrow.clone();
                            adjacent.rooms[r][0] = None;
                            adjacent.rooms[p as usize][1] = Some(p);
                            search.push(
                                cost,
                                (k - k_upd + 3) * Burrow::ENERGIES[p as usize],
                                adjacent,
                            );
                        } else if burrow.rooms[p as usize][1] == Some(p) {
                            let mut adjacent = burrow.clone();
                            adjacent.rooms[r][0] = None;
                            adjacent.rooms[p as usize][0] = Some(p);
                            search.push(
                                cost,
                                (k - k_upd + 2) * Burrow::ENERGIES[p as usize],
                                adjacent,
                            );
                        }
                    }
                    k_upd = k_upd.wrapping_sub(1);
                }

                // move in hallway to the right
                let mut k_upd = k + 1;
                while k_upd < 11 && burrow.hallway[k_upd].is_none() {
                    if !Burrow::DOORS.contains(&k_upd) {
                        let mut adjacent = burrow.clone();
                        adjacent.rooms[r][0] = None;
                        adjacent.hallway[k_upd] = Some(p);
                        search.push(
                            cost,
                            (k_upd - k + 1) * Burrow::ENERGIES[p as usize],
                            adjacent,
                        );
                    } else if k_upd == Burrow::DOORS[p as usize]
                        && burrow.rooms[p as usize][0].is_none()
                        && p != r as u8
                    {
                        if burrow.rooms[p as usize][1] == None {
                            let mut adjacent = burrow.clone();
                            adjacent.rooms[r][0] = None;
                            adjacent.rooms[p as usize][1] = Some(p);
                            search.push(
                                cost,
                                (k_upd - k + 3) * Burrow::ENERGIES[p as usize],
                                adjacent,
                            );
                        } else if burrow.rooms[p as usize][1] == Some(p) {
                            let mut adjacent = burrow.clone();
                            adjacent.rooms[r][0] = None;
                            adjacent.rooms[p as usize][0] = Some(p);
                            search.push(
                                cost,
                                (k_upd - k + 2) * Burrow::ENERGIES[p as usize],
                                adjacent,
                            );
                        }
                    }
                    k_upd += 1;
                }
            }
        } // end "for each room"
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

    const CONTENT: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
    const BURROW: Burrow = Burrow {
        hallway: [None; 11],
        rooms: [
            [Some(1), Some(0)],
            [Some(2), Some(3)],
            [Some(1), Some(2)],
            [Some(3), Some(0)],
        ],
    };

    #[test]
    fn test_parse() {
        assert_eq!(BURROW, parse(CONTENT));
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(12521, solution_1(BURROW));
    }
}
// end::tests[]
