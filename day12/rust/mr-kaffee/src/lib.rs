use std::collections::VecDeque;

// tag::cavemap[]
/// Map of Caves
///
/// Each cave has a unique ID: the position of it's name in the [CaveMap::caves] field.
/// The adjacents of the cave are stored in the list at the same position in the [CaveMap::adjacents] field.
///
/// The IDs of the start and end cave are stored in the fields [CaveMap::start] and [CaveMap::end].
pub struct CaveMap<'a> {
    pub caves: Vec<&'a str>,
    pub adjacents: Vec<Vec<usize>>,
    pub start: usize,
    pub end: usize,
}

impl<'a> CaveMap<'a> {
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

            let lhs_id = parts.next().expect("No LHS");
            let rhs_id = parts.next().expect("No RHS");
            assert!(parts.next().is_none(), "More than two parts");
            adjacents[lhs_id].push(rhs_id);
            adjacents[rhs_id].push(lhs_id);
        }

        CaveMap {
            caves,
            adjacents,
            start,
            end,
        }
    }

    /// check whether cave with given ID is small
    pub fn is_small(&self, id: usize) -> bool {
        self.caves[id].chars().next().unwrap().is_ascii_lowercase()
    }
}
// end::cavemap[]

pub fn parse(input: &'static str) -> CaveMap {
    CaveMap::parse(input)
}

// tag::contains[]
/// determine whether a path contains a cave
fn contains(paths: &[(usize, Option<usize>)], ptr: usize, id: usize) -> bool {
    let mut ptr = ptr;
    loop {
        let (cave_id, opt_ptr) = paths[ptr];
        if cave_id == id {
            return true;
        }

        if let Some(next_ptr) = opt_ptr {
            ptr = next_ptr;
        } else {
            return false;
        }
    }
}
// end::contains[]

// tag::get_path_count[]
/// count number of distinct pathes using map
///
/// the flag ``no_duplicate_small`` controls whether small caves may be visited more than
/// once. If the flag is set to ``true``, this is not allowed at all. If it is set to false,
/// it is allowed at most once.
pub fn get_path_count(map: &CaveMap, no_duplicate_small: bool) -> usize {
    // store all path elements as a cave ID and a parent pointer (= index to this vec) wrapped in an Option
    let mut paths: Vec<(usize, Option<usize>)> = Vec::new();

    // count of unique paths
    let mut path_count = 0;

    let mut queue = VecDeque::new();
    paths.push((map.start, None));
    queue.push_back((paths.len() - 1, no_duplicate_small));

    while let Some((ptr, no_duplicate_small)) = queue.pop_front() {
        let cave_id = paths[ptr].0;
        for &id in &map.adjacents[cave_id] {
            if id == map.start {
                // never go back to start
                continue;
            } else if id == map.end {
                // new path to "end" found
                path_count += 1;
                continue;
            }

            let duplicate_small = map.is_small(id) && contains(&paths, ptr, id);
            if no_duplicate_small && duplicate_small {
                // skip small cave already on path, if no duplicate small caves are allowed
                continue;
            }

            // push new path element to paths vec
            paths.push((id, Some(ptr)));

            // add pointer to path element to queue
            // if a duplicate small was added, no further duplicate smalls are allowed
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

    fn get_test_data() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
        BTreeMap::from([
            ("start", BTreeSet::from(["A", "b"])),
            ("A", BTreeSet::from(["start", "c", "b", "end"])),
            ("b", BTreeSet::from(["start", "A", "d", "end"])),
            ("c", BTreeSet::from(["A"])),
            ("d", BTreeSet::from(["b"])),
            ("end", BTreeSet::from(["A", "b"])),
        ])
    }

    fn get_adjacents<'a>(map: &'a CaveMap, name: &str) -> Option<impl Iterator<Item = &'a str>> {
        map.caves
            .iter()
            .position(|cave| *cave == name)
            .map(|idx| map.adjacents[idx].iter().map(|adj| map.caves[*adj]))
    }

    #[test]
    fn test_parse() {
        let map = parse(TEST_INPUT);
        for (name, adjacents) in get_test_data() {
            assert_eq!(
                Some(adjacents),
                get_adjacents(&map, name).map(|it| it.collect())
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
