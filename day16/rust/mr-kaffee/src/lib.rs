#[derive(Debug, PartialEq, Eq, Clone)]
// tag::packets[]
pub struct Packets {
    data: Vec<u8>,
    byte: usize,
    bit: u8,
}

impl Packets {
    pub fn from(bytes: &[u8]) -> Self {
        Packets {
            data: Vec::from(bytes),
            byte: 0,
            bit: 0,
        }
    }

    pub fn read_number(&mut self, len: usize) -> usize {
        let mut v = 0usize;
        for _ in 0..len {
            v = (v << 1) | ((self.data[self.byte] >> (3 - self.bit)) & 1) as usize;
            self.bit += 1;
            if self.bit > 3 {
                self.bit = 0;
                self.byte += 1;
            }
        }

        v
    }

    pub fn pos(&self) -> usize {
        4 * self.byte + self.bit as usize
    }
}
// end::packets[]

// tag::parse[]
///
/// ```
/// # use mr_kaffee_2021_16::*;
/// assert_eq!(Packets::from(&[0, 1, 10]), parse("01A"));
/// ```
pub fn parse(content: &str) -> Packets {
    let data = content
        .trim()
        .chars()
        .map(|c| match c {
            '0'..='9' => c as u8 - '0' as u8,
            'A'..='F' => c as u8 - 'A' as u8 + 10,
            _ => panic!("Illegal character: {}", c),
        })
        .collect::<Vec<_>>();
    Packets::from(&data)
}
// end::parse[]


// tag::sum_versions[]
pub fn sum_versions(packets: &mut Packets) -> usize {
    let mut sum = 0;

    let version = packets.read_number(3);
    let type_id = packets.read_number(3);

    if type_id == 4 {
        let mut value = 0;
        loop {
            let f = packets.read_number(1);
            let v = packets.read_number(4);

            value = value << 4 | v;

            if f == 0 {
                break;
            }
        }
    } else {
        let l = packets.read_number(1);
        let n = packets.read_number(if l == 1 { 11 } else { 15 });

        if l == 1 {
            for _ in 0..n {
                sum += sum_versions(packets);
            }
        } else {
            let target = packets.pos() + n;
            while packets.pos() < target {
                sum += sum_versions(packets);
            }
        }
    }

    version + sum
}
// end::sum_versions[]

// tag::process_packet[]
pub fn process_packet(packets: &mut Packets) -> usize {
    let mut value = 0;

    packets.read_number(3);
    let type_id = packets.read_number(3);

    if type_id == 4 {
        loop {
            let f = packets.read_number(1);
            let v = packets.read_number(4);

            value = value << 4 | v;

            if f == 0 {
                break;
            }
        }
    } else {
        let l = packets.read_number(1);
        let n = packets.read_number(if l == 1 { 11 } else { 15 });
        let mut values = Vec::new();
        if l == 1 {
            for _ in 0..n {
                values.push(process_packet(packets));
            }
        } else {
            let target = packets.pos() + n;
            while packets.pos() < target {
                values.push(process_packet(packets));
            }
        }

        value = match type_id {
            0 => values.iter().sum::<usize>(),
            1 => values.iter().product::<usize>(),
            2 => *values.iter().min().expect("No min"),
            3 => *values.iter().max().expect("No max"),
            5 => (values[0] > values[1]) as usize,
            6 => (values[0] < values[1]) as usize,
            7 => (values[0] == values[1]) as usize,
            _ => panic!("Illegal type ID: {}", type_id),
        }
    }

    value
}
// end::process_packet[]

pub fn solution_1(packets: &Packets) -> usize {
    sum_versions(&mut packets.clone())
}

pub fn solution_2(packets: &Packets) -> usize {
    process_packet(&mut packets.clone())
}

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT_1: &str = "8A004A801A8002F478";
    const CONTENT_2: &str = "620080001611562C8802118E34";
    const CONTENT_3: &str = "C0015000016115A2E0802F182340";
    const CONTENT_4: &str = "A0016C880162017C3686B18A3D4780";

    const DATA_0: &[u8] = &[0xD, 0x2, 0xF, 0xE, 0x2, 0x8];
    const DATA_2: &[u8] = &[
        6, 2, 0, 0, 8, 0, 0, 0, 1, 6, 1, 1, 5, 6, 2, 0xC, 8, 8, 0, 2, 1, 1, 8, 0xE, 3, 4,
    ];

    const SUM_0: usize = 6;
    const SUM_1: usize = 16;
    const SUM_2: usize = 12;
    const SUM_3: usize = 23;
    const SUM_4: usize = 31;

    #[test]
    fn test_parse() {
        assert_eq!(Packets::from(DATA_2), parse(CONTENT_2));
    }

    #[test]
    fn test_solution_1() {
        assert_eq!(SUM_0, solution_1(&mut Packets::from(DATA_0)));
        assert_eq!(SUM_1, solution_1(&mut parse(CONTENT_1)));
        assert_eq!(SUM_2, solution_1(&mut Packets::from(DATA_2)));
        assert_eq!(SUM_3, solution_1(&mut parse(CONTENT_3)));
        assert_eq!(SUM_4, solution_1(&mut parse(CONTENT_4)));
    }

    #[test]
    fn test_solution_2() {
        assert_eq!(3, solution_2(&mut parse("C200B40A82")));
        assert_eq!(7, solution_2(&mut parse("880086C3E88112")));
        assert_eq!(1, solution_2(&mut parse("9C0141080250320F1802104A08")));
    }
}
// end::tests[]
