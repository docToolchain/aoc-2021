use std::{
    cmp,
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

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

/// range of scanner
const RANGE: RangeInclusive<isize> = -1000..=1000;

// tag::parse[]
/// parse the input into a vector of scanners
///
/// each scanner is a hash set of coordinates in the scanner's local
/// coordinate system and a hashmap of pairwise distances as keys and number of
/// occurance of this distance as value
pub fn parse(content: &str) -> Vec<(HashSet<Coord>, Coord, HashMap<usize, usize>)> {
    let mut scanners: Vec<(HashSet<Coord>, Coord, HashMap<usize, usize>)> = Vec::new();

    let mut beacons = HashSet::new();
    let mut distances = HashMap::new();
    for line in content.lines().map(|l| l.trim()).filter(|l| l.len() > 0) {
        if line.starts_with("---") {
            if !beacons.is_empty() {
                scanners.push((beacons, (0, 0, 0), distances));
                beacons = HashSet::new();
                distances = HashMap::new();
            }
        } else {
            let mut parts = line.split(',');
            let b_new = (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );
            for b_old in &beacons {
                // update pairwise distances
                *distances.entry(b_old.sub(&b_new).abs()).or_insert(0) += 1;
            }
            beacons.insert(b_new);
        }
    }
    if !beacons.is_empty() {
        scanners.push((beacons, (0, 0, 0), distances));
    }

    scanners
}
// end::parse[]

// tag::sanity[]
/// sanity check for overlapping scanner regions
///
/// for all beacons seen by scanner 1 / 2, check whether they are also seen by scanner 2 / 1
/// if and only if they are in range of scanner 2 / 1
/// 
/// # Panics
/// if sanity check fails
pub fn sanity_check(
    beacons1: &HashSet<Coord>,
    center1: &Coord,
    beacons2: &HashSet<Coord>,
    center2: &Coord,
    t_idx: usize,
    d: Coord,
) {
    // => b1 = t(b2) + d
    // => b2 = t^-1(b1 - d)

    // transform center2 in 1's coordinates
    let center2 = TRAFOS[t_idx](*center2).add(&d);

    for b1 in beacons1 {
        // distance vec
        let (dx, dy, dz) = center2.sub(b1);

        let in_range = RANGE.contains(&dx) && RANGE.contains(&dy) && RANGE.contains(&dz);
        // transform in 2's coordinates for check
        let contained = beacons2.contains(&INV_TRAFOS[t_idx](b1.sub(&d)));
        if in_range != contained {
            panic!(
                "{:?} is in range of 2nd scanner at {:?}, but not contained in set",
                b1, center2
            );
        }
    }

    // loop over beacons seen by scanner 2 in 1's coordinates
    for b2 in beacons2.iter().map(|b2| TRAFOS[t_idx](*b2).add(&d)) {
        // distance vec
        let (dx, dy, dz) = center1.sub(&b2);

        let in_range = RANGE.contains(&dx) && RANGE.contains(&dy) && RANGE.contains(&dz);
        let contained = beacons1.contains(&b2);
        if in_range != contained {
            panic!(
                "{:?} is in range of 1st scanner at {:?}, but not contained in set",
                b2, center1
            );
        }
    }
}
// end::sanity[]

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
    (beacons_settled, _center_settled, distances_settled): &(
        HashSet<Coord>,
        Coord,
        HashMap<usize, usize>,
    ),
    (beacons_check, _center_check, distances_check): &(
        HashSet<Coord>,
        Coord,
        HashMap<usize, usize>,
    ),
    threshold: usize,
) -> Option<(fn(Coord) -> Coord, Coord)> {
    // first check pairwise distances (idea taken from Moritz Gronbach, https://github.com/mogron)
    let mut common_distances = 0;
    for (distance, count) in distances_settled {
        common_distances += cmp::min(distances_check.get(distance).unwrap_or(&0), count);
        if common_distances >= threshold * (threshold - 1) / 2 {
            break;
        }
    }
    if common_distances < threshold * (threshold - 1) / 2 {
        return None; // not enough common distances, no overlap possible
    }

    for t_idx in 0..24 {
        // find all beacons with the same distance as b1[k1] to b2[k2]
        for b1 in beacons_settled {
            for b2 in beacons_check {
                // d = b1 - t(b2)
                // => b1 = t(b2) + d
                // => b2 = t^-1(b1 - d)
                let d = b1.sub(&TRAFOS[t_idx](*b2));

                // filter for scanner range prior to lookup
                // settled beacons are in transformed coordinate (scanner is not necessarily at origin)
                // => need to check range in beacons_check via inverse transform
                // (possible optimization: use actual bounding box of beacons)
                let mut count = 0;
                for _ in beacons_settled
                    .iter()
                    .map(|b1| INV_TRAFOS[t_idx](b1.sub(&d)))
                    .filter(|(x, y, z)| RANGE.contains(x) && RANGE.contains(y) && RANGE.contains(z))
                    .filter(|b1| beacons_check.contains(b1))
                {
                    // don't use iter.count() to avoid checking more elements once threshold is reached
                    count += 1;
                    if count >= threshold {
                        if cfg!(feature = "sanity-check") {
                            sanity_check(
                                beacons_settled,
                                _center_settled,
                                beacons_check,
                                _center_check,
                                t_idx,
                                d,
                            );
                        }
                        return Some((TRAFOS[t_idx], d));
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
pub fn solution_1_2(scanners: &[(HashSet<Coord>, Coord, HashMap<usize, usize>)]) -> (usize, usize) {
    // use my own mutable copy of the scanners
    let mut scanners = scanners.to_owned();

    // holds flags for settled scanners
    // 0 - not settled
    // 1 - settled, enqueued for furhter extension of settled scanners
    // 2 - fully processed (dequeued)
    let mut settled = vec![0; scanners.len()];
    settled[0] = 1u8; // start with first scanner settled

    let mut transformed = Vec::with_capacity(scanners.len());
    transformed.push(0usize);

    // Used to calculate largest distance between any two scanners.
    let mut max_dist = 0;

    // holds all settled beacons. Start with beacons in range of scanner 1
    let mut beacons: HashSet<Coord> = scanners[0].0.clone();

    while let Some(k1) = settled.iter().position(|s| s == &1) {
        for k2 in 0..settled.len() {
            if settled[k2] != 0 {
                continue; // already settled
            }

            if let Some((t, d)) = check_overlap(&scanners[k1], &scanners[k2], 12) {
                // update scanner to settled coordinates
                scanners[k2].0 = scanners[k2].0.iter().map(|b| t(*b).add(&d)).collect();
                scanners[k2].1 = t(scanners[k2].1).add(&d);

                // add beacons to unique set
                beacons.extend(&scanners[k2].0);

                // update settled to 1 = enqueue
                settled[k2] = 1;

                // update max distance
                for k_t in &transformed {
                    let dist = scanners[k2].1.sub(&scanners[*k_t].1).abs();
                    if dist > max_dist {
                        max_dist = dist;
                    }
                }

                transformed.push(k2);
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
        assert!(scanners[2].0.contains(&(-644, 584, -595)));
        assert!(scanners[4].0.contains(&(30, -46, -14)));
    }

    #[test]
    fn test_compare() {
        let scanners = parse(CONTENT);
        let r = check_overlap(&scanners[0], &scanners[1], 12);
        assert!(r.is_some(), "No transformation found");
        let (t, d) = r.unwrap();

        assert_eq!((68, -1246, -43), d);
        assert_eq!((-618, -824, -621), t((686, 422, 578)).add(&d));
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
