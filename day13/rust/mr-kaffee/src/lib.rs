use std::{cmp, collections::HashSet, ops::Add};

// tag::parse[]
pub fn parse(content: &str) -> (HashSet<(usize, usize)>, Vec<(bool, usize)>) {
    let mut parts = content.split("\n\n");
    (
        parts
            .next()
            .expect("No points")
            .lines()
            .map(|line| line.split(","))
            .map(|mut point_parts| {
                (
                    point_parts
                        .next()
                        .expect("No x coordinate")
                        .parse()
                        .expect("Could not parse x"),
                    point_parts
                        .next()
                        .expect("No y coordinate")
                        .parse()
                        .expect("Could not parse y"),
                )
            })
            .collect(),
        parts
            .next()
            .expect("No fold instructions")
            .lines()
            .map(|line| line.split("="))
            .map(|mut fold_parts| {
                (
                    match fold_parts.next() {
                        Some("fold along y") => true,
                        Some("fold along x") => false,
                        _ => panic!("Unexpected fold instruction"),
                    },
                    fold_parts
                        .next()
                        .expect("No fold coordinate")
                        .parse()
                        .expect("Could not parse fold coordinate"),
                )
            })
            .collect(),
    )
}
// end::parse[]

// tag::fold[]
/// perform a single fold
pub fn fold(
    points: &HashSet<(usize, usize)>,
    horizontal: bool,
    coord: usize,
) -> HashSet<(usize, usize)> {
    points
        .iter()
        .map(|(x, y)| {
            if horizontal && y > &coord {
                (*x, 2 * coord - *y)
            } else if !horizontal && x > &coord {
                (2 * coord - *x, *y)
            } else {
                (*x, *y)
            }
        })
        .collect()
}
// end::fold[]

// tag::part1[]
/// count points after first fold operation
pub fn solution_1(points: &HashSet<(usize, usize)>, folds: &[(bool, usize)]) -> usize {
    let (horizontal, fold_coordinate) = folds.first().expect("No folds");
    fold(points, *horizontal, *fold_coordinate).len()
}
// end::part1[]

// tag::part2[]
pub fn points_to_string(points: &HashSet<(usize, usize)>) -> String {
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
                .map(|x| if points.contains(&(x, y)) { '#' } else { '.' })
                .collect::<String>()
        })
        .map(|line| line.add("\n"))
        .collect::<String>()
}

/// perform all fold operations and return result as a ``String``
pub fn solution_2(points: &HashSet<(usize, usize)>, folds: &[(bool, usize)]) -> String {
    let mut points = points.to_owned();
    for (horizontal, fold_coordinate) in folds {
        points = fold(&points, *horizontal, *fold_coordinate);
    }

    points_to_string(&points)
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
                               #...#\n\
                               #...#\n\
                               #...#\n\
                               #####\n";

    #[test]
    fn test_parse() {
        let (points, folds) = parse(&TEST_CONTENT);
        assert_eq!(HashSet::from(TEST_POINTS), points);
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
