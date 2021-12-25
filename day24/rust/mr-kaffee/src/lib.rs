
// tag::instructions[]
#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Inp(usize),
    Add(usize, Value),
    Mul(usize, Value),
    Div(usize, Value),
    Mod(usize, Value),
    Eql(usize, Value),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    Var(usize),
    Val(isize),
}

impl Value {
    pub fn eval(&self, vars: &[isize; 4]) -> isize {
        match self {
            Value::Var(v) => vars[*v],
            Value::Val(v) => *v,
        }
    }
}

impl Instruction {
    pub fn parse(line: &str) -> Self {
        let mut parts = line.trim().split(" ");
        let name = parts.next().expect("No name in instruction");
        let first = match parts.next().expect("No first part") {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => panic!("Illegal first part in '{}'", line),
        };
        let second = parts.next().map(|second| match second {
            "w" => Value::Var(0),
            "x" => Value::Var(1),
            "y" => Value::Var(2),
            "z" => Value::Var(3),
            _ => Value::Val(second.parse().expect("Could not parse value")),
        });

        match name {
            "inp" => Self::Inp(first),
            "add" => Self::Add(first, second.expect("No second part")),
            "mul" => Self::Mul(first, second.expect("No second part")),
            "div" => Self::Div(first, second.expect("No second part")),
            "mod" => Self::Mod(first, second.expect("No second part")),
            "eql" => Self::Eql(first, second.expect("No second part")),
            _ => panic!("Illegal name in '{}'", line),
        }
    }

    pub fn eval(&self, vars: &mut [isize; 4], inputs: &mut impl Iterator<Item = isize>) -> bool {
        match self {
            Instruction::Inp(v1) => {
                if let Some(v) = inputs.next() {
                    vars[*v1] = v;
                    true
                } else {
                    false
                }
            }
            Instruction::Add(v1, v2) => {
                vars[*v1] += v2.eval(&vars);
                true
            }
            Instruction::Mul(v1, v2) => {
                vars[*v1] *= v2.eval(&vars);
                true
            }
            Instruction::Div(v1, v2) => {
                let v = v2.eval(&vars);
                if v != 0 {
                    vars[*v1] /= v;
                    true
                } else {
                    false
                }
            }
            Instruction::Mod(v1, v2) => {
                let val1 = vars[*v1];
                let val2 = v2.eval(&vars);
                if val1 >= 0 && val2 > 0 {
                    vars[*v1] = val1 % val2;
                    true
                } else {
                    false
                }
            }
            Instruction::Eql(v1, v2) => {
                vars[*v1] = (vars[*v1] == v2.eval(&vars)) as isize;
                true
            }
        }
    }
}

pub fn parse(content: &str) -> Vec<Instruction> {
    content
        .lines()
        .map(|line| Instruction::parse(line))
        .collect()
}

pub fn eval(instructions: &[Instruction], inputs: &[isize]) -> Option<isize> {
    let mut vars = [0; 4];
    let mut inputs = inputs.iter().map(|v| *v);
    for instruction in instructions {
        if !instruction.eval(&mut vars, &mut inputs) {
            return None;
        }
    }
    Some(vars[3])
}
// end::instructions[]

// tag::solution[]
pub const BOUNDS: [Option<isize>; 14] = [
    Some(5),
    None,
    None,
    None,
    Some(6),
    None,
    Some(1),
    Some(2),
    None,
    Some(8),
    Some(1),
    None,
    Some(3),
    None,
];

pub const CONSTRAINTS: [Option<fn(&[isize]) -> isize>; 14] = [
    None,
    Some(|digits| digits[12] + 6),
    Some(|digits| digits[9] + 1),
    Some(|digits| digits[4] + 3),
    None,
    Some(|digits| digits[6] + 8),
    None,
    None,
    Some(|digits| digits[7] + 7),
    None,
    None,
    Some(|digits| digits[10] + 8),
    None,
    Some(|digits| digits[0] + 4),
];

pub fn solution_1(instructions: &[Instruction]) -> isize {
    let mut inputs = BOUNDS
        .iter()
        .map(|b| if let Some(p) = b { *p } else { 0 })
        .collect::<Vec<_>>();
    for (k, c) in CONSTRAINTS.iter().enumerate() {
        if let Some(c) = c {
            inputs[k] = c(&inputs);
        }
    }

    assert_eq!(Some(0), eval(instructions, &inputs));

    inputs.iter().fold(0, |v, d| 10 * v + *d)
}

pub fn solution_2(instructions: &[Instruction]) -> isize {
    let mut inputs = BOUNDS
        .iter()
        .map(|b| if b.is_some() { 1 } else { 0 })
        .collect::<Vec<_>>();
    for (k, c) in CONSTRAINTS.iter().enumerate() {
        if let Some(c) = c {
            inputs[k] = c(&inputs);
        }
    }

    assert_eq!(Some(0), eval(instructions, &inputs));

    inputs.iter().fold(0, |v, d| 10 * v + *d)
}
// end::solution[]

// tag::tests[]
#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "inp z
        inp x
        mul z 3
        eql z x";
    const INSTRUCTIONS: &[Instruction] = &[
        Instruction::Inp(3),
        Instruction::Inp(1),
        Instruction::Mul(3, Value::Val(3)),
        Instruction::Eql(3, Value::Var(1)),
    ];

    #[test]
    fn test_parse() {
        assert_eq!(INSTRUCTIONS, parse(CONTENT));
    }
}
// end::tests[]
