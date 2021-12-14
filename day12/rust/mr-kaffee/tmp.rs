use std::collections::VecDeque;

/// map of caves
pub struct CaveMap<'a> {
    pub caves: Vec<&'a str>,
    pub adjacents: Vec<Vec<usize>>,
    pub start: usize,
    pub end: usize,
}

impl<'a> CaveMap<'a> {
    // tag::parse[]
    /// parse input to map
    pub fn parse(input: &'a str) -> Self {
        let mut caves = Vec::new();
        let mut adjacents = Vec::new();
        let mut start = 0;
        let mut end = 0;

        for line in input.lines() {
            let mut parts = line.split('-').map(|part| {
                caves
                    .iter()
                    .position(|cave| *cave == part)
                    .unwrap_or_else(|| {
                        caves.push(part);
                        adjacents.push(Vec::new());
                        // keep ID to start or end
                        match part {
                            "start" => start = caves.len() - 1,
                            "end" => end = caves.len() - 1,
                            _ => {}
                        }
                        caves.len() - 1
                    })
            });

            let lhs = parts.next().expect("No LHS");
            let rhs = parts.next().expect("No RHS");
            assert!(parts.next().is_none(), "More than two parts");
            adjacents[lhs].push(rhs);
            adjacents[rhs].push(lhs);
        }

        CaveMap {
            caves,
            adjacents,
            start,
            end,
        }
    }
    // end::parse[]

    /// get index of cave identified by name
    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.caves.iter().position(|cave| *cave == name)
    }

    /// get iterator over adjacents for name
    pub fn get_adjacents(&self, name: &str) -> Option<impl Iterator<Item = &str>> {
        self.index_of(name)
            .map(|idx| self.adjacents[idx].iter().map(|adj| self.caves[*adj]))
    }

    /// check whether cave with given id is small
    pub fn is_small(&self, id: usize) -> bool {
        self.caves[id].chars().next().unwrap().is_ascii_lowercase()
    }
}

pub fn parse(input: &'static str) -> CaveMap {
    CaveMap::parse(input)
}

/// determine whether a path contains a cave
fn contains(paths: &[(usize, Option<usize>)], path_idx: usize, cave_idx: usize) -> bool {
    let mut path_idx = path_idx;
    loop {
        let (idx, parent) = paths[path_idx];
        if idx == cave_idx {
            return true;
        }

        if let Some(next_idx) = parent {
            path_idx = next_idx;
        } else {
            return false;
        }
    }
}

// tag::get_path_count[]
/// count number of distinct pathes using map
///
/// the flag ``no_duplicate_small`` controls whether small caves may be visited more than
/// once. If the flag is set to ``true``, this is not allowed at all. If it is set to false,
/// it is allowed at most once.
pub fn get_path_count(map: &CaveMap, no_duplicate_small: bool) -> usize {
    // store all path elements as a cave idx and an index to the parent element (= index to this vec)
    let mut paths: Vec<(usize, Option<usize>)> = Vec::new();

    // count of unique paths
    let mut path_count = 0;

    let mut queue = VecDeque::new();
    paths.push((map.start, None));
    queue.push_back((paths.len() - 1, no_duplicate_small));

    while let Some((path_idx, no_duplicate_small)) = queue.pop_front() {
        let cave = paths[path_idx].0;
        for &adjacent in &map.adjacents[cave] {
            if adjacent == map.start {
                // never go back to start
                continue;
            } else if adjacent == map.end {
                // new path to "end" found
                path_count += 1;
                continue;
            }

            let duplicate_small = map.is_small(adjacent) && contains(&paths, path_idx, adjacent);
            if no_duplicate_small && duplicate_small {
                // skip lower case already on path, if no duplicate small caves are allowed
                continue;
            }

            // extend path by adjacent
            paths.push((adjacent, Some(path_idx)));

            // add extended path to queue, if a duplicate small was added, no further duplicate smalls are allowed
            queue.push_back((paths.len() - 1, no_duplicate_small || duplicate_small));
        }
    }

    path_count
}
// end::get_path_count[]

// tag::part1[]
pub fn solution_1(map: &CaveMap) -> usize {
    get_path_count(map, true)
}
// end::part1[]

// tag::part2[]
pub fn solution_2(map: &CaveMap) -> usize {
    get_path_count(map, false)
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use super::*;

    const TEST_INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    fn test_data() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
        BTreeMap::from([
            ("start", BTreeSet::from(["A", "b"])),
            ("A", BTreeSet::from(["start", "c", "b", "end"])),
            ("b", BTreeSet::from(["start", "A", "d", "end"])),
            ("c", BTreeSet::from(["A"])),
            ("d", BTreeSet::from(["b"])),
            ("end", BTreeSet::from(["A", "b"])),
        ])
    }

    #[test]
    fn test_parse() {
        let map = parse(TEST_INPUT);
        for (name, adjacents) in test_data() {
            assert_eq!(
                Some(adjacents),
                map.get_adjacents(name).map(|it| it.collect())
            );
        }
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(10, solution_1(&parse(&TEST_INPUT)))
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(36, solution_2(&parse(&TEST_INPUT)))
    }
}
// end::tests[]
