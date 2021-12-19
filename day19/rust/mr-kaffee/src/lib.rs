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
pub const TRAFOS: [fn(Coord) -> Coord; 24] = [
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

/// inverse transformation functions for all 24 possible transformations
pub const INV_TRAFOS: [fn(Coord) -> Coord; 24] = [
    |(x, y, z)| (x, y, z),
    |(x, y, z)| (x, -y, -z),
    |(x, y, z)| (x, -z, y),
    |(x, y, z)| (x, z, -y),
    |(x, y, z)| (-x, y, -z),
    |(x, y, z)| (-x, -y, z),
    |(x, y, z)| (-x, z, y),
    |(x, y, z)| (-x, -z, -y),
    |(x, y, z)| (y, x, -z),
    |(x, y, z)| (-y, x, z),
    |(x, y, z)| (z, x, y),
    |(x, y, z)| (-z, x, -y),
    |(x, y, z)| (y, -x, z),
    |(x, y, z)| (-y, -x, -z),
    |(x, y, z)| (-z, -x, y),
    |(x, y, z)| (z, -x, -y),
    |(x, y, z)| (y, z, x),
    |(x, y, z)| (-y, -z, x),
    |(x, y, z)| (-z, y, x),
    |(x, y, z)| (z, -y, x),
    |(x, y, z)| (y, -z, -x),
    |(x, y, z)| (-y, z, -x),
    |(x, y, z)| (z, y, -x),
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
    for line in content.lines().map(|l| l.trim()).filter(|l| l.len() > 0) {
        if line.starts_with("---") {
            if !beacons.is_empty() {
                scanners.push(beacons);
                beacons = HashSet::new();
            }
        } else {
            let mut parts = line.split(',');
            beacons.insert((
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            ));
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
/// find the transformation ``t`` for the ``beacons_check`` set, and the ``center`` position
/// of the corresponding scanner in the ``beacons_settled``'s coordinate system so that at
/// least ``threshold`` pairs of beacons (b1, b2) in the two sets exist that satisfy
/// ``t(b2) - center = b1``
///
/// return the transformation ``t`` and the ``center`` position
pub fn check_overlap(
    beacons_settled: &HashSet<Coord>,
    beacons_check: &HashSet<Coord>,
    threshold: usize,
) -> Option<(fn(Coord) -> Coord, Coord)> {
    for t_idx in 0..24 {
        // find all beacons with the same distance as b1[k1] to b2[k2]
        for b1 in beacons_settled {
            for b2 in beacons_check {
                // center = t(b2) - b1
                // => b1 = t(b2) - center
                let center = TRAFOS[t_idx](*b2).sub(b1);

                // filter for scanner range prior to lookup
                // settled beacons are in transformed coordinate (scanner is not necessarily at origin)
                // => need to check range in beacons_check via inverse transform
                // (possible optimization: use actual bounding box of beacons)
                let mut count = 0;
                for _ in beacons_settled
                    .iter()
                    .map(|b1| INV_TRAFOS[t_idx](b1.add(&center)))
                    .filter(|(x, y, z)| {
                        x >= &-1000
                            && x <= &1000
                            && y >= &-1000
                            && y <= &1000
                            && z >= &-1000
                            && z <= &1000
                    })
                    .filter(|b1| beacons_check.contains(b1))
                {
                    // don't use iter.count() to avoid checking more elements onece threshold is reached
                    count += 1;
                    if count >= threshold {
                        return Some((TRAFOS[t_idx], center));
                    }
                }
            }
        }
    }

    None // no solution found
}
// end::check_overlap[]

// tag::solution[]
/// determine number of unique beacons and max Manhatten distance between
/// any two scanners
pub fn solution_1_2(scanners: &[HashSet<Coord>]) -> (usize, usize) {
    // use my own mutable copy of the scanners
    let mut scanners = scanners.to_owned();

    // holds flags for settled scanners
    // 0 - not settled
    // 1 - settled, enqueued for furhter extension of settled scanners
    // 2 - fully processed (dequeued)
    let mut settled = vec![0; scanners.len()];
    settled[0] = 1u8; // start with first scanner settled

    // holds scanner coordinates in the coordinate system of the settled scanners.
    // Used to calculate largest distance between any two scanners.
    let mut centers = Vec::with_capacity(scanners.len());
    centers.push((0, 0, 0)); // first scanner's position (= reference)
    let mut max_dist = 0;

    // holds all settled beacons. Start with beacons in range of scanner 1
    let mut beacons: HashSet<Coord> = scanners[0].clone();

    while let Some(k1) = settled.iter().position(|s| s == &1) {
        for k2 in 0..settled.len() {
            if settled[k2] != 0 {
                continue; // already settled
            }

            if let Some((t, c)) = check_overlap(&scanners[k1], &scanners[k2], 12) {
                // update scanner to settled coordinates
                scanners[k2] = scanners[k2].iter().map(|b| t(*b).sub(&c)).collect();

                // add beacons to unique set
                beacons.extend(&scanners[k2]);

                // update settled to 1 = enqueue
                settled[k2] = 1;

                // update max distance
                for c_settled in &centers {
                    let dist = c.sub(&c_settled).abs();
                    if dist > max_dist {
                        max_dist = dist;
                    }
                }

                // add center
                centers.push(c);
            }
        }

        // update settled to 2 = fully processed
        settled[k1] = 2;
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

    #[test]
    fn test_trafos() {
        for k in 0..24 {
            assert_eq!(
                (1, 2, 3),
                TRAFOS[k](INV_TRAFOS[k]((1, 2, 3))),
                "Failed trafo after inverse at {}",
                k
            );
            assert_eq!(
                (1, 2, 3),
                INV_TRAFOS[k](TRAFOS[k]((1, 2, 3))),
                "Failed inverse after trafo at {}",
                k
            );
        }
    }
}
// end::tests[]
