use std::{
    cmp::{self, Ordering},
    collections::HashMap,
};

//tag::counters[]
/// to allow using the same code with counts stored in a vector or in a hash map, the interface is modeled as a trait
pub trait VentsCount {
    /// increment count at given coordinate
    fn increment(&mut self, coord: (isize, isize));
    /// count number of coordinates with count > 1
    fn count_dangerous(&self) -> usize;
}

impl VentsCount for HashMap<(isize, isize), usize> {
    fn increment(&mut self, coord: (isize, isize)) {
        // get entry, create and initialize with value 0 if not yet present
        let count = self.entry(coord).or_insert(0);
        // increment
        *count += 1;
    }

    fn count_dangerous(&self) -> usize {
        self.values().filter(|count| **count > 1).count()
    }
}

/// structure to count vents based on a vector
pub struct VecVentsCount {
    counts: Vec<usize>,
    width: isize,
    x_min: isize,
    y_min: isize,
}

impl VecVentsCount {
    /// create new ``VecVentsCount`` for bounding box
    pub fn new((x_min, y_min, x_max, y_max): (isize, isize, isize, isize)) -> Self {
        let width = x_max - x_min;
        let counts = vec![0usize; (width * (y_max - y_min)) as usize];
        VecVentsCount {
            counts,
            width,
            x_min,
            y_min,
        }
    }
}

impl VentsCount for VecVentsCount {
    fn increment(&mut self, (x, y): (isize, isize)) {
        self.counts[(x - self.x_min + self.width * (y - self.y_min)) as usize] += 1;
    }

    fn count_dangerous(&self) -> usize {
        self.counts.iter().filter(|count| **count > 1).count()
    }
}
//end::counters[]

//tag::parse[]
/// parse lines ``"x1,y1 -> x2,y2"`` to tuples ``(x1, y1, x2, y2)``
pub fn parse(content: &str) -> Vec<(isize, isize, isize, isize)> {
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

            (x1, y1, x2, y2)
        })
        .collect()
}
//end::parse[]

//tag::bbox[]
/// get bounding box over lines
///
/// returns ``(x_min, y_min, x_max, y_max)`` so that all points ``(x, y)`` on
/// any line satisfy ``x_min <= x < x_max`` and ``y_min <= y < y_max``
pub fn get_bbox(lines: &[(isize, isize, isize, isize)]) -> (isize, isize, isize, isize) {
    lines.iter().fold(
        (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
        |(x_min, y_min, x_max, y_max), (x1, y1, x2, y2)| {
            (
                cmp::min(cmp::min(x_min, *x1), *x2),
                cmp::min(cmp::min(y_min, *y1), *y2),
                cmp::max(cmp::max(x_max, *x1 + 1), *x2 + 1),
                cmp::max(cmp::max(y_max, *y1 + 1), *y2 + 1),
            )
        },
    )
}
//end::bbox[]

//tag::count_overlaps[]
/// this function uses the struct [VecVentsCount] by default. This behavior can be changed to using ``HashMap`` based
/// counting using the feature ``hash_counters``
pub fn count_overlaps(lines: &[(isize, isize, isize, isize)], incl_diagonal: bool) -> usize {
    let mut counts: Box<dyn VentsCount> = if cfg!(feature = "hash_counters") {
        Box::new(HashMap::new())
    } else {
        Box::new(VecVentsCount::new(get_bbox(lines)))
    };

    for (x1, y1, x2, y2) in lines {
        if incl_diagonal || x1 == x2 || y1 == y2 {
            let dx = match x1.cmp(&x2) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                Ordering::Less => 1,
            };
            let dy = match y1.cmp(&y2) {
                Ordering::Equal => 0,
                Ordering::Greater => -1,
                Ordering::Less => 1,
            };
            let len = cmp::max((x2 - x1) * dx, (y2 - y1) * dy) + 1;
            for k in 0..len {
                counts.increment((x1 + k * dx, y1 + k * dy));
            }
        }
    }

    counts.count_dangerous()
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

    const LINES: &'static [(isize, isize, isize, isize)] = &[
        (0, 9, 5, 9),
        (8, 0, 0, 8),
        (9, 4, 3, 4),
        (2, 2, 2, 1),
        (7, 0, 7, 4),
        (6, 4, 2, 0),
        (0, 9, 2, 9),
        (3, 4, 1, 4),
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
