use std::{cmp, collections::BTreeSet, ops::Add};

// tag::parse[]
pub fn parse(content: &str) -> (BTreeSet<(usize, usize)>, Vec<(bool, usize)>) {
    let mut parts = content.split("\n\n");
    let points = parts.next().expect("No points");
    let folds = parts.next().expect("No folds");
    assert!(parts.next().is_none(), "Excess data");

    let points = points
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            (
                parts
                    .next()
                    .expect("No x coordinate")
                    .parse()
                    .expect("Could not parse x"),
                parts
                    .next()
                    .expect("No y coordinate")
                    .parse()
                    .expect("Could not parse y"),
            )
        })
        .collect();

    let folds = folds
        .lines()
        .map(|line| {
            let mut parts = line.split("=");
            (
                match parts.next() {
                    Some("fold along y") => true,
                    Some("fold along x") => false,
                    _ => panic!("Unexpected fold instruction"),
                },
                parts
                    .next()
                    .expect("No fold coordinate")
                    .parse()
                    .expect("Could not parse fold coordinate"),
            )
        })
        .collect();

    (points, folds)
}
// end::parse[]

// tag::fold[]
/// perform a single fold
pub fn fold(
    points: &BTreeSet<(usize, usize)>,
    horizontal: bool,
    fold_coordinate: usize,
) -> BTreeSet<(usize, usize)> {
    points
        .iter()
        .map(|(x, y)| {
            if horizontal && y > &fold_coordinate {
                (*x, fold_coordinate - (*y - fold_coordinate))
            } else if !horizontal && x > &fold_coordinate {
                (fold_coordinate - (*x - fold_coordinate), *y)
            } else {
                (*x, *y)
            }
        })
        .collect()
}
// end::fold[]

// tag::part1[]
/// count points after first fold operation
pub fn solution_1(points: &BTreeSet<(usize, usize)>, folds: &[(bool, usize)]) -> usize {
    let (horizontal, fold_coordinate) = folds.first().expect("No folds");
    fold(points, *horizontal, *fold_coordinate).len()
}
// end::part1[]

// tag::part2[]
/// perform all fold operations and return result as a ``String``
pub fn solution_2(points: &BTreeSet<(usize, usize)>, folds: &[(bool, usize)]) -> String {
    let mut points = points.to_owned();
    for (horizontal, fold_coordinate) in folds {
        points = fold(&points, *horizontal, *fold_coordinate);
    }

    // calculate bounding box
    let (x_min, y_min, x_max, y_max) = points.iter().fold(
        (usize::MAX, usize::MAX, 0, 0),
        |(x_min, y_min, x_max, y_max), (x, y)| {
            (
                cmp::min(x_min, *x),
                cmp::min(y_min, *y),
                cmp::max(x_max, *x + 1),
                cmp::max(y_max, *y + 1),
            )
        },
    );

    // assemble result string
    (y_min..y_max)
        .map(|y| {
            (x_min..x_max)
                .map(|x| if points.contains(&(x, y)) { '#' } else { ' ' })
                .collect::<String>()
        })
        .map(|line| line.add("\n"))
        .collect::<String>()
}
// end::part2[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CONTENT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    const TEST_POINTS: [(usize, usize); 18] = [
        (6, 10),
        (0, 14),
        (9, 10),
        (0, 3),
        (10, 4),
        (4, 11),
        (6, 0),
        (6, 12),
        (4, 1),
        (0, 13),
        (10, 12),
        (3, 4),
        (3, 0),
        (8, 4),
        (1, 10),
        (2, 14),
        (8, 10),
        (9, 0),
    ];

    const TEST_FOLDS: &[(bool, usize)] = &[(true, 7), (false, 5)];

    const TEST_RESULT: &str = "#####\n\
                               #   #\n\
                               #   #\n\
                               #   #\n\
                               #####\n";

    #[test]
    fn test_parse() {
        let (points, folds) = parse(&TEST_CONTENT);
        assert_eq!(BTreeSet::from(TEST_POINTS), points);
        assert_eq!(TEST_FOLDS, folds);
    }

    #[test]
    fn test_solution_1() {
        let (points, folds) = parse(&TEST_CONTENT);
        assert_eq!(17, solution_1(&points, &folds));
    }

    #[test]
    fn test_solution_2() {
        let (points, folds) = parse(&TEST_CONTENT);
        assert_eq!(TEST_RESULT, solution_2(&points, &folds));
    }
}
// end::tests[]
