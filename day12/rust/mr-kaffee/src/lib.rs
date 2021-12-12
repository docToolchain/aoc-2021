use std::collections::{BTreeMap, BTreeSet, VecDeque};

// tag::parse[]
/// Parse the input in a map from nodes to sets of neighbor nodes
pub fn parse<'a>(input: &'a str) -> BTreeMap<&'a str, BTreeSet<&'a str>> {
    let mut map = BTreeMap::new();

    for line in input.lines() {
        let mut parts = line.split('-');

        let lhs = parts.next().expect("No LHS");
        let rhs = parts.next().expect("No RHS");
        assert!(parts.next().is_none(), "More than two parts");

        map.entry(lhs).or_insert(BTreeSet::new()).insert(rhs);
        map.entry(rhs).or_insert(BTreeSet::new()).insert(lhs);
    }
    map
}
// end::parse[]

// tag::get_path_count[]
/// count number of distinct pathes using map
///
/// the flag ``no_duplicate_small`` controls whether small caves may be visited more than
/// once. If the flag is set to ``true``, this is not allowed at all. If it is set to false,
/// it is allowed at most once.
pub fn get_path_count(map: &BTreeMap<&str, BTreeSet<&str>>, no_duplicate_small: bool) -> usize {
    let mut path_count = 0;

    let mut queue = VecDeque::new();
    queue.push_back((vec!["start"], no_duplicate_small));

    while let Some((path, no_duplicate_small)) = queue.pop_front() {
        let cave = path.last().expect("Incomplete path is empty");
        for adjacent in map.get(cave).expect("Node missing in map") {
            match *adjacent {
                "start" => continue, // never go back to "start"
                "end" => { // new path to "end" found
                    path_count += 1;
                    continue;
                }
                _ => {} // no special treatment
            }

            let duplicate_small = adjacent
                .chars()
                .next()
                .expect("Empty node")
                .is_ascii_lowercase()
                && path.contains(adjacent);
            if no_duplicate_small && duplicate_small {
                // skip lower case already on path, if no duplicate small caves are allowed
                continue;
            }

            // extend path by adjacent
            let mut path_extended = path.clone();
            path_extended.push(adjacent);

            // add extended path to queue, if a duplicate small was added, no further duplicate smalls are allowed
            queue.push_back((path_extended, no_duplicate_small || duplicate_small));
        }
    }

    path_count
}
// end::get_path_count[]

// tag::part1[]
pub fn solution_1(map: &BTreeMap<&str, BTreeSet<&str>>) -> usize {
    get_path_count(map, true)
}
// end::part1[]

// tag::part2[]
pub fn solution_2(map: &BTreeMap<&str, BTreeSet<&str>>) -> usize {
    get_path_count(map, false)
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
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
        assert_eq!(test_data(), parse(TEST_INPUT));
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(10, solution_1(&test_data()))
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(36, solution_2(&test_data()))
    }
}
// end::tests[]
