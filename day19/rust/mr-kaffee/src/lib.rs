use std::collections::HashSet;

pub type Coord = (isize, isize, isize);

pub trait CoordMath {
    fn add(self, rhs: &Self) -> Self;
    fn subtract(self, rhs: &Self) -> Self;
}

impl CoordMath for Coord {
    fn add(mut self, rhs: &Self) -> Self {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self
    }

    fn subtract(mut self, rhs: &Self) -> Self {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self
    }
}

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

pub fn get_inverse_trafo(t: usize) -> usize {
    match t {
        0 => 0,   // (x, y, z) -> (x, y, z)
        1 => 1,   // (x, -y, -z) -> (x, -y, -z)
        2 => 3,   // (x, z, -y) -> (x, -z, y)
        3 => 2,   // (x, -z, y) -> (x, z, -y)
        4 => 4,   // (-x, y, -z) -> (-x, y, -z)
        5 => 5,   // (-x, -y, z) -> (-x, -y, z)
        6 => 6,   // (-x, z, y) -> (-x, y, z)
        7 => 7,   // (-x, -z, -y) -> (-x, -z, -y)
        8 => 8,   // (y, x, -z) -> (y, x, -z)
        9 => 12,  // (y, -x, z) -> (-y, x, z)
        10 => 16, // (y, z, x) -> (z, x, y)
        11 => 20, // (y, -z, -x) -> (-z, x, -y)
        12 => 9,  // (-y, x, z) -> (y, -x, z)
        13 => 13, // (-y, -x, -z) -> (-y, -x, -z)
        14 => 21, // (-y, z, -x) -> (-z, -x, y)
        15 => 17, // (-y, -z, x) -> (z, -x, -y)
        16 => 10, // (z, x, y) -> (y, z, x)
        17 => 15, // (z, -x, -y) -> (-y, -z, x)
        18 => 22, // (z, y, -x) -> (-z, y, x)
        19 => 19, // (z, -y, x) -> (z, -y, x)
        20 => 11, // (-z, x, -y) -> (y, -z, -x)
        21 => 14, // (-z, -x, y) -> (-y, z, -x)
        22 => 18, // (-z, y, x) -> (z, y, -x)
        23 => 23, // (-z, -y, -x) -> (-z, -y, -x)
        _ => panic!("Illegal transformation"),
    }
}

pub fn compare(
    beacons1: &HashSet<Coord>,
    beacons2: &HashSet<Coord>,
    threshold: usize,
) -> Option<(usize, Coord)> {
    for t in 0..24 {
        // find all beacons with the same distance as b1[k1] to b2[k2]
        for b1 in beacons1 {
            for b2 in beacons2 {
                // d = t(b2) - b1
                // => b1 = t(b2) - d
                let d = transform(*b2, t).subtract(b1);

                let count = beacons2
                    .iter()
                    .map(|b2| transform(*b2, t).subtract(&d))
                    .filter(|b2| beacons1.contains(b2))
                    .count();

                if count >= threshold {
                    return Some((t, d));
                }
            }
        }
    }

    None
}

fn settle(scanners: &[HashSet<Coord>], settled: &[bool]) -> Option<(usize, HashSet<Coord>, Coord)> {
    for s1 in scanners
        .iter()
        .enumerate()
        .filter(|(k, _)| settled[*k])
        .map(|(_, s)| s)
    {
        for (k2, s2) in scanners.iter().enumerate().filter(|(k, _)| !settled[*k]) {
            if let Some((t, d)) = compare(s1, s2, 12) {
                let upd = s2
                    .iter()
                    .map(|p| transform(*p, t).subtract(&d))
                    .collect::<HashSet<_>>();
                return Some((k2, upd, d));
            }
        }
    }

    None
}

pub fn create_map(scanners: &[HashSet<Coord>]) -> (usize, usize) {
    let mut scanners = scanners.to_owned();
    let mut settled = vec![false; scanners.len()];
    let mut coords: Vec<Option<Coord>> = vec![None; scanners.len()];
    settled[0] = true;
    coords[0] = Some((0, 0, 0));

    loop {
        if let Some((k, upd, d)) = settle(&scanners, &settled) {
            settled[k] = true;
            scanners[k] = upd;
            coords[k] = Some(d);
        } else {
            break;
        }
    }

    assert!(settled.iter().all(|f| *f));

    let mut beacons: HashSet<Coord> = HashSet::new();
    for scanner in scanners {
        beacons.extend(&scanner);
    }

    let mut max_d = 0;
    for c1 in &coords {
        for c2 in &coords {
            let d = c1.unwrap().subtract(&c2.unwrap());
            let d = (d.0.abs() + d.1.abs() + d.2.abs()) as usize;
            if d > max_d {
                max_d = d;
            }
        }
    }

    (beacons.len(), max_d)
}

//
pub fn transform(p: Coord, t: usize) -> Coord {
    let (x, y, z) = p;
    // (forward, left, up)
    match t {
        0 => (x, y, z),
        1 => (x, -y, -z),
        2 => (x, z, -y),
        3 => (x, -z, y),
        4 => (-x, y, -z),
        5 => (-x, -y, z),
        6 => (-x, z, y),
        7 => (-x, -z, -y),
        8 => (y, x, -z),
        9 => (y, -x, z),
        10 => (y, z, x),
        11 => (y, -z, -x),
        12 => (-y, x, z),
        13 => (-y, -x, -z),
        14 => (-y, z, -x),
        15 => (-y, -z, x),
        16 => (z, x, y),
        17 => (z, -x, -y),
        18 => (z, y, -x),
        19 => (z, -y, x),
        20 => (-z, x, -y),
        21 => (-z, -x, y),
        22 => (-z, y, x),
        23 => (-z, -y, -x),
        _ => panic!("Illegal transformation"),
    }
}

// tag::part1[]
pub fn solution_1_2(scanners: &[HashSet<Coord>]) -> (usize, usize) {
    create_map(scanners)
}
// end::part1[]

// tag::part2[]
pub fn solution_2(_scanners: &[HashSet<Coord>]) -> usize {
    1
}
// end::part2[]

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
    fn test_get_inverse_trafo() {
        for t in 0..24 {
            assert_eq!(
                t,
                get_inverse_trafo(get_inverse_trafo(t)),
                "Double inverse is not self for {}",
                t
            );
            assert_eq!(
                (1, 2, 3),
                transform(transform((1, 2, 3), t), get_inverse_trafo(t)),
                "Inverse trafo does not yield original for {}",
                t
            );
        }
    }

    #[test]
    fn test_compare() {
        let scanners = parse(CONTENT);
        let r = compare(&scanners[0], &scanners[1], 12);
        assert!(r.is_some(), "No transformation found");
        let (t, d) = r.unwrap();

        assert_eq!((-68, 1246, 43), d);
        assert_eq!(
            (-618, -824, -621),
            transform((686, 422, 578), t).subtract(&d)
        );
    }

    #[test]
    fn test_create_map() {
        let scanners = parse(CONTENT);
        assert_eq!((79, 3621), create_map(&scanners));
    }
}
// end::tests[]
