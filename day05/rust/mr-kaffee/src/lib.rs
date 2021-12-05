use std::cmp;

//tag::parse[]
/// parse lines ``"x1,y1 -> x2,y2"`` to toples ``(x1, y1, x2, y2)``
///
/// the lines are normalized so that ``x1 <= x2`` and ``x1 == x2`` implies ``y1 <= y2``
pub fn parse(content: &str) -> Vec<(usize, usize, usize, usize)> {
    content
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let mut xy1 = parts.next().expect("No first point").split(',');
            let x1 = xy1
                .next()
                .expect("No x1")
                .parse()
                .expect("Could not parse x1");
            let y1 = xy1
                .next()
                .expect("No y1")
                .parse()
                .expect("Could not parse y1");
            let mut xy2 = parts.next().expect("NO second point").split(',');
            let x2 = xy2
                .next()
                .expect("No x2")
                .parse()
                .expect("Could not parse x2");
            let y2 = xy2
                .next()
                .expect("No y2")
                .parse()
                .expect("Could not parse y2");

            if x2 < x1 || (x1 == x2) && (y2 < y1) {
                // reverse order
                // ensure that x1 <= x2 and x1 == x2 implies y1 <= y2
                (x2, y2, x1, y1)
            } else {
                (x1, y1, x2, y2)
            }
        })
        .collect()
}
//end::parse[]

//tag::bbox[]
/// get bounding box over lines
///
/// returns ``(x_min, y_min, x_max, y_max)`` so that all points ``(x, y)`` on
/// any line satisfy ``x_min <= x < x_max`` and ``y_min <= y < y_max``
pub fn get_bbox(lines: &[(usize, usize, usize, usize)]) -> (usize, usize, usize, usize) {
    lines.iter().fold(
        (usize::MAX, usize::MAX, usize::MIN, usize::MIN),
        |(x_min, y_min, x_max, y_max), (x1, y1, x2, y2)| {
            (
                cmp::min(x_min, *x1),
                cmp::min(cmp::min(y_min, *y1), *y2),
                cmp::max(x_max, *x2 + 1),
                cmp::max(cmp::max(y_max, *y1 + 1), *y2 + 1),
            )
        },
    )
}
//end::bbox[]

//tag::count_overlaps[]
pub fn count_overlaps(lines: &[(usize, usize, usize, usize)], incl_diagonal: bool) -> usize {
    let (x_min, y_min, x_max, y_max) = get_bbox(lines);
    let width = x_max - x_min;
    let mut counts = vec![0usize; width * (y_max - y_min)];

    for (x1, y1, x2, y2) in lines {
        if x1 == x2 {
            // vertical line
            for y in *y1..=*y2 {
                counts[*x1 - x_min + width * (y - y_min)] += 1;
            }
        } else if y1 == y2 {
            // horizontal line
            for x in *x1..=*x2 {
                counts[x - x_min + width * (*y1 - y_min)] += 1;
            }
        } else if incl_diagonal {
            // check line is actually diagonal
            if x2 - x1 != cmp::max(y1, y2) - cmp::min(y1, y2) {
                panic!("Not a diagnoal line: {},{} -> {},{}", x1, y1, x2, y2);
            }

            if y2 < y1 {
                // bottom left to top right (x and y progress in different directions)
                for k in 0..x2 - x1 + 1 {
                    counts[x1 + k - x_min + width * (y1 - k - y_min)] += 1;
                }
            } else {
                // top left to bottom right
                for k in 0..x2 - x1 + 1 {
                    counts[x1 + k - x_min + width * (y1 + k - y_min)] += 1;
                }
            }
        }
    }

    counts.iter().filter(|count| **count > 1).count()
}
//end::count_overlaps[]

//tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    const LINES: &'static [(usize, usize, usize, usize)] = &[
        (0, 9, 5, 9),
        (0, 8, 8, 0),
        (3, 4, 9, 4),
        (2, 1, 2, 2),
        (7, 0, 7, 4),
        (2, 0, 6, 4),
        (0, 9, 2, 9),
        (1, 4, 3, 4),
        (0, 0, 8, 8),
        (5, 5, 8, 2),
    ];

    #[test]
    fn test_parse() {
        assert_eq!(LINES, parse(CONTENT));
    }

    #[test]
    fn test_get_bbox() {
        assert_eq!((0, 0, 10, 10), get_bbox(LINES));
    }

    #[test]
    fn test_count_overlaps_straight() {
        assert_eq!(5, count_overlaps(LINES, false));
    }

    #[test]
    fn test_count_overlaps() {
        assert_eq!(12, count_overlaps(LINES, true));
    }
}
//end::tests[]