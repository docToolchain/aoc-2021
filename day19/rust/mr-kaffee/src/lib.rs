use std::collections::HashSet;

// tag::coordinate[]
/// type alias for 3D coordinate
pub type Coord = (isize, isize, isize);

/// basic math operations for coordinates
pub trait CoordMath {
    fn add(self, rhs: &Self) -> Self;
    fn sub(self, rhs: &Self) -> Self;
    fn abs(&self) -> usize;
}

impl CoordMath for Coord {
    fn add(mut self, rhs: &Self) -> Self {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self
    }

    fn sub(mut self, rhs: &Self) -> Self {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self
    }

    fn abs(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}
// end::coordinate[]

// tag::trafos[]
/// transformation functions for all 24 possible transformations
const TRAFOS: [fn(Coord) -> Coord; 24] = [
    |(x, y, z)| (x, y, z),
    |(x, y, z)| (x, -y, -z),
    |(x, y, z)| (x, z, -y),
    |(x, y, z)| (x, -z, y),
    |(x, y, z)| (-x, y, -z),
    |(x, y, z)| (-x, -y, z),
    |(x, y, z)| (-x, z, y),
    |(x, y, z)| (-x, -z, -y),
    |(x, y, z)| (y, x, -z),
    |(x, y, z)| (y, -x, z),
    |(x, y, z)| (y, z, x),
    |(x, y, z)| (y, -z, -x),
    |(x, y, z)| (-y, x, z),
    |(x, y, z)| (-y, -x, -z),
    |(x, y, z)| (-y, z, -x),
    |(x, y, z)| (-y, -z, x),
    |(x, y, z)| (z, x, y),
    |(x, y, z)| (z, -x, -y),
    |(x, y, z)| (z, y, -x),
    |(x, y, z)| (z, -y, x),
    |(x, y, z)| (-z, x, -y),
    |(x, y, z)| (-z, -x, y),
    |(x, y, z)| (-z, y, x),
    |(x, y, z)| (-z, -y, -x),
];
// end::trafos[]

// tag::parse[]
/// parse the input into a vector of scanners
///
/// each scanner is a hash set of coordinates in the scanner's local
/// coordinate system
pub fn parse(content: &str) -> Vec<HashSet<Coord>> {
    let mut scanners = Vec::new();

    let mut beacons = HashSet::new();
    for line in content.lines() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        } else if line.starts_with("---") {
            if !beacons.is_empty() {
                scanners.push(beacons);
                beacons = HashSet::new();
            }
        } else {
            let mut parts = line
                .trim()
                .split(',')
                .map(|s| s.parse().expect("Could not parse."));
            let p = (
                parts.next().expect("No x"),
                parts.next().expect("No y"),
                parts.next().expect("No z"),
            );
            assert!(parts.next().is_none(), "More than three coordinates");
            beacons.insert(p);
        }
    }
    if !beacons.is_empty() {
        scanners.push(beacons);
    }

    scanners
}
// end::parse[]

// tag::check_overlap[]
/// check whether two sets of beacons overlap
///
/// find the transformation ``t`` for the second set, and a distance ``d`` of the references,
/// so that at least ``threshold`` pairs of beacons (b1, b2) in the two sets exist that satisfy
/// ``t(b2) - d = b1``
pub fn check_overlap(
    beacons1: &HashSet<Coord>,
    beacons2: &HashSet<Coord>,
    threshold: usize,
) -> Option<(fn(Coord) -> Coord, Coord)> {
    for t in TRAFOS {
        // find all beacons with the same distance as b1[k1] to b2[k2]
        for b1 in beacons1 {
            for b2 in beacons2 {
                // d = t(b2) - b1
                // => b1 = t(b2) - d
                let d = t(*b2).sub(b1);

                let mut count = 0;
                for _ in beacons2
                    .iter()
                    .map(|b2| t(*b2).sub(&d))
                    .filter(|b2| beacons1.contains(b2))
                {
                    // don't use iter.count() to avoid checking more elements onece threshold is reached
                    count += 1;
                    if count >= threshold {
                        return Some((t, d));
                    }
                }
            }
        }
    }

    None
}
// end::check_overlap[]

// tag::solution[]
/// check whether any unsettled scanner can be settled given the settled beacons
///
/// if so, return the index of that scanner, the beacons in the coordinates of the settled beacons, and the reference distance
pub fn settle_any(
    beacons: &HashSet<Coord>,
    scanners: &[HashSet<Coord>],
    settled: &[bool],
) -> Option<(usize, Vec<Coord>, Coord)> {
    for (k2, s2) in scanners.iter().enumerate().filter(|(k, _)| !settled[*k]) {
        if let Some((t, d)) = check_overlap(&beacons, s2, 12) {
            return Some((k2, s2.iter().map(|p| t(*p).sub(&d)).collect(), d));
        }
    }

    None
}

/// determine number of unique beacons
pub fn solution_1_2(scanners: &[HashSet<Coord>]) -> (usize, usize) {
    // holds flags for settled scanners
    let mut settled = vec![false; scanners.len()];
    settled[0] = true; // start with first scanner settled

    // holds distances from all scanners to first scanner.
    // Used to calculate largest distance between any two scanners.
    let mut deltas = Vec::with_capacity(scanners.len());
    deltas.push((0, 0, 0));
    let mut max_dist = 0;

    // holds all settled beacons. Start with beacons in range of scanner 1
    let mut beacons: HashSet<Coord> = scanners[0].clone();

    // while any scanner has been settled
    while let Some((k, iter, d)) = settle_any(&beacons, &scanners, &settled) {
        // update flag
        settled[k] = true;
        // add beacons to settled beacons
        beacons.extend(iter);

        // update max distance
        for d0 in &deltas {
            let dist = d.sub(&d0).abs();
            if dist > max_dist {
                max_dist = dist;
            }
        }

        // save distance to first scanner
        deltas.push(d);
    }

    // return number of beacons and largest distance between any two scanners
    (beacons.len(), max_dist)
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = include_str!("../test.txt");

    #[test]
    fn test_parse() {
        let scanners = parse(CONTENT);
        assert_eq!(5, scanners.len());
        assert!(scanners[2].contains(&(-644, 584, -595)));
        assert!(scanners[4].contains(&(30, -46, -14)));
    }

    #[test]
    fn test_compare() {
        let scanners = parse(CONTENT);
        let r = check_overlap(&scanners[0], &scanners[1], 12);
        assert!(r.is_some(), "No transformation found");
        let (t, d) = r.unwrap();

        assert_eq!((-68, 1246, 43), d);
        assert_eq!((-618, -824, -621), t((686, 422, 578)).sub(&d));
    }

    #[test]
    fn test_solution_1_2() {
        let scanners = parse(CONTENT);
        assert_eq!((79, 3621), solution_1_2(&scanners));
    }
}
// end::tests[]
