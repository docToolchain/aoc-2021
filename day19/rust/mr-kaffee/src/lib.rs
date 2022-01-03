use std::{
    cmp,
    collections::{HashMap, HashSet},
    str::FromStr,
};

// tag::coordinate[]
/// type alias for 3D coordinate
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Coord(isize, isize, isize);

impl Coord {
    pub fn add(mut self, rhs: &Self) -> Self {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
        self
    }

    pub fn sub(mut self, rhs: &Self) -> Self {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
        self
    }

    pub fn abs(&self) -> usize {
        (self.0.abs() + self.1.abs() + self.2.abs()) as usize
    }
}

impl FromStr for Coord {
    type Err = Box<dyn std::error::Error>;

    /// parse 3D coordinate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split(',').map(|v| v.parse());
        let x = vals
            .next()
            .ok_or_else(|| format!("Illegal coordinate: {}; no x value", s))??;
        let y = vals
            .next()
            .ok_or_else(|| format!("Illegal coordinate: {}; no y value", s))??;
        let z = vals
            .next()
            .ok_or_else(|| format!("Illegal coordinate: {}; no z value", s))??;
        Ok(Coord(x, y, z))
    }
}

impl std::fmt::Display for Coord {
    /// print 3D coordinate
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}
// end::coordinate[]

/// range of scanner
const RANGE: std::ops::RangeInclusive<isize> = -1000..=1000;

// tag::trafos[]
/// transformation functions for all 24 possible transformations
pub const TRAFOS: [fn(Coord) -> Coord; 24] = [
    |Coord(x, y, z)| Coord(x, y, z),
    |Coord(x, y, z)| Coord(x, -y, -z),
    |Coord(x, y, z)| Coord(x, z, -y),
    |Coord(x, y, z)| Coord(x, -z, y),
    |Coord(x, y, z)| Coord(-x, y, -z),
    |Coord(x, y, z)| Coord(-x, -y, z),
    |Coord(x, y, z)| Coord(-x, z, y),
    |Coord(x, y, z)| Coord(-x, -z, -y),
    |Coord(x, y, z)| Coord(y, x, -z),
    |Coord(x, y, z)| Coord(y, -x, z),
    |Coord(x, y, z)| Coord(y, z, x),
    |Coord(x, y, z)| Coord(y, -z, -x),
    |Coord(x, y, z)| Coord(-y, x, z),
    |Coord(x, y, z)| Coord(-y, -x, -z),
    |Coord(x, y, z)| Coord(-y, z, -x),
    |Coord(x, y, z)| Coord(-y, -z, x),
    |Coord(x, y, z)| Coord(z, x, y),
    |Coord(x, y, z)| Coord(z, -x, -y),
    |Coord(x, y, z)| Coord(z, y, -x),
    |Coord(x, y, z)| Coord(z, -y, x),
    |Coord(x, y, z)| Coord(-z, x, -y),
    |Coord(x, y, z)| Coord(-z, -x, y),
    |Coord(x, y, z)| Coord(-z, y, x),
    |Coord(x, y, z)| Coord(-z, -y, -x),
];
// end::trafos[]

// tag::scanner[]
#[derive(Debug, Clone)]
pub struct Scanner {
    id: usize,
    pos: Coord,
    beacons: Vec<Coord>,
    dists: Vec<HashMap<usize, Vec<usize>>>,
}

impl Scanner {
    const THRESHOLD: usize = 12;

    pub fn len(&self) -> usize {
        self.beacons.len()
    }

    pub fn transform(&mut self, t: fn(Coord) -> Coord, o: &Coord) {
        self.pos = t(self.pos).add(o);
        for k in 0..self.len() {
            self.beacons[k] = t(self.beacons[k]).add(o);
        }
    }

    // tag::sanity[]
    /// sanity check for overlapping scanner regions
    ///
    /// for all beacons seen by self / other, check whether they are also seen by other / self
    /// if and only if they are in range of other / self
    ///
    /// # Panics
    /// if sanity check fails
    pub fn sanity_check(&self, other: &Self, t: fn(Coord) -> Coord, o: &Coord) {
        // create a copy of other in self's coordinates
        let mut other = other.to_owned();
        other.transform(t, o);

        // loop over beacons seen by self
        for b1 in &self.beacons {
            let d = other.pos.sub(b1);
            let in_range = RANGE.contains(&d.0) && RANGE.contains(&d.1) && RANGE.contains(&d.2);
            let contained = other.beacons.contains(b1);
            if in_range != contained {
                panic!(
                    "{} is in range of 2nd scanner {} at {}, but not contained in set or vice versa",
                    b1, other.id, other.pos
                );
            }
        }

        // loop over beacons seen by other
        for b2 in &other.beacons {
            let d = self.pos.sub(b2);
            let in_range = RANGE.contains(&d.0) && RANGE.contains(&d.1) && RANGE.contains(&d.2);
            let contained = self.beacons.contains(b2);
            if in_range != contained {
                panic!(
                    "{} is in range of 1st scanner {} at {}, but not contained in set or vice versa",
                    b2, self.id, self.pos
                );
            }
        }
    }
    // end::sanity[]

    // tag::check_overlap[]
    /// check whether ``self`` overlaps with ``other``
    ///
    /// find the transformation ``t`` and the offset ``o`` such that the coordinate system
    /// of ``other`` is transformed into the coordinate system of ``self`` as ``t(c) + o``,
    /// i.e., after calling ``other.transform(t, &o)``, the same beacons have the same
    /// coordinates in both scanners.
    pub fn check_overlap(&self, other: &Self) -> Option<(fn(Coord) -> Coord, Coord)> {
        for i1 in 0..=self.len() - Self::THRESHOLD {
            for j1 in 0..=other.len() - Self::THRESHOLD {
                let mut cnt = 0;
                let mut pairs = Vec::new();

                for (d, is) in &self.dists[i1] {
                    if let Some(js) = other.dists[j1].get(d) {
                        cnt += cmp::min(is.len(), js.len());
                        for i2 in is {
                            for j2 in js {
                                pairs.push((*i2, *j2));
                            }
                        }
                    }
                }

                if cnt < Self::THRESHOLD {
                    continue;
                }

                // beacon i1 from self and beacon j1 from other have at least Self::THRESHOLD
                // distances to other beacons in common
                for t in TRAFOS {
                    for k0 in 0..=pairs.len() - Self::THRESHOLD {
                        let o = self.beacons[pairs[k0].0].sub(&t(other.beacons[pairs[k0].1]));
                        let cnt = pairs
                            .iter()
                            .filter(|(i, j)| self.beacons[*i] == t(other.beacons[*j]).add(&o))
                            .count();
                        if cnt >= Self::THRESHOLD {
                            if cfg!(feature = "sanity-check") {
                                self.sanity_check(other, t, &o);
                            }
                            return Some((t, o));
                        }
                    }
                }
            }
        }

        None
    }
    // end::check_overlap[]
}

impl FromStr for Scanner {
    type Err = Box<dyn std::error::Error>;

    // tag::parse[]
    /// parse scanner
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().map(|l| l.trim()).filter(|l| l.len() > 0);

        let trimchars: &[char] = &[' ', '-'];
        let head = lines.next().ok_or("Empty content")?.trim_matches(trimchars);
        let id = head
            .strip_prefix("scanner ")
            .ok_or_else(|| format!("Illegal header: {}", head))?
            .parse()?;

        let pos = Coord(0, 0, 0);

        let mut beacons = Vec::new();
        for line in lines {
            beacons.push(line.parse::<Coord>()?);
        }

        let mut dists = Vec::with_capacity(beacons.len());
        for k1 in 0..beacons.len() {
            let mut map = HashMap::new();
            for k2 in 0..beacons.len() {
                let d = beacons[k2].sub(&beacons[k1]).abs();
                (*map.entry(d).or_insert(Vec::new())).push(k2);
            }
            dists.push(map);
        }

        Ok(Scanner {
            id,
            pos,
            beacons,
            dists,
        })
    }
    // end::parse[]
}

impl std::fmt::Display for Scanner {
    /// print scanner (without offset)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- scanner {} ---", self.id)?;
        for k in 0..self.len() {
            self.beacons[k].sub(&self.pos).fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
// end::scanner[]

pub fn parse(content: &str) -> Vec<Scanner> {
    content.split("\n--").map(|s| s.parse().unwrap()).collect()
}

// tag::solution[]
pub fn solution_1_2(scanners: &[Scanner]) -> (usize, usize) {
    // use my own mutable copy of the scanners
    let mut scanners = scanners.to_owned();

    // state
    let mut stack = Vec::with_capacity(scanners.len());
    let mut settled = vec![false; scanners.len()];
    let mut transformed = Vec::with_capacity(scanners.len());
    let mut beacons: HashSet<Coord> = HashSet::new();

    // init (start with scanner 0)
    stack.push(0);
    settled[0] = true;
    transformed.push(0);
    beacons.extend(&scanners[0].beacons);

    let mut max_dist = 0;

    while let Some(k1) = stack.pop() {
        for k2 in 0..scanners.len() {
            if settled[k2] {
                continue;
            }

            if let Some((t, o)) = scanners[k1].check_overlap(&scanners[k2]) {
                // update scanner to settled coordinates
                scanners[k2].transform(t, &o);

                // update max distance
                for k_t in &transformed {
                    let dist = scanners[k2].pos.sub(&scanners[*k_t].pos).abs();
                    if dist > max_dist {
                        max_dist = dist;
                    }
                }

                // push k2 to stack, set settled, add to list of transformed and extend unique beacons
                stack.push(k2);
                settled[k2] = true;
                transformed.push(k2);
                beacons.extend(&scanners[k2].beacons);
            }
        }
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
        assert!(scanners[2].beacons.contains(&Coord(-644, 584, -595)));
        assert!(scanners[4].beacons.contains(&Coord(30, -46, -14)));
    }

    #[test]
    fn test_parse_format() {
        let scanners = parse(CONTENT);
        let text = scanners
            .iter()
            .map(|s| format!("{}\n", s))
            .collect::<String>();
        assert_eq!(CONTENT.trim(), text.trim());
    }

    #[test]
    fn test_check_overlap() {
        let scanners = parse(CONTENT);
        let r = scanners[0].check_overlap(&scanners[1]);
        assert!(r.is_some(), "No transformation found");
        let (t, d) = r.unwrap();

        assert_eq!(Coord(68, -1246, -43), d);
        assert_eq!(Coord(-618, -824, -621), t(Coord(686, 422, 578)).add(&d));
    }

    #[test]
    fn test_solution_1_2() {
        let scanners = parse(CONTENT);
        assert_eq!((79, 3621), solution_1_2(&scanners));
    }
}
// end::tests[]
