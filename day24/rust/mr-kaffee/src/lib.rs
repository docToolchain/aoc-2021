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
pub fn get_constraints(instructions: &[Instruction]) -> Vec<(usize, usize, isize)> {
    let block_starts = instructions
        .iter()
        .enumerate()
        .filter(|(_, i)| i == &&Instruction::Inp(0))
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    assert_eq!(14, block_starts.len());

    // z holds elements (n, o) where n is an input number and o as an offset
    // the actual value of the z variable will be
    // z.iter().fold(0, |z, (n, o)| z * 26 + input[n] + o);
    let mut z: Vec<(usize, isize)> = Vec::with_capacity(14);
    // constraints holds elements (n1, n2, o) and represents a constraint
    // input[n1] == input[n2] + o, which needs to be satisfied for a valid serial number
    let mut constraints = Vec::with_capacity(7);
    for (n, k) in block_starts.iter().enumerate() {
        // take last digit of z
        // a single 'div z a' instruction is expected per block where a is either
        // 26 or 1. In case of a == 26, the last element from z is removed
        let d = if instructions[*k..]
            .iter()
            .filter_map(|i| {
                if let Instruction::Div(3, Value::Val(div)) = i {
                    Some(*div)
                } else {
                    None
                }
            })
            .next()
            .expect("No div")
            == 26
        {
            // z is divided by 26, pop element
            z.pop()
        } else {
            // z is divided by 1, peek element
            z.last().map(|d| *d)
        };

        // a single instruction 'add x a' where a is a literal value is expected in a block
        // the value a is used to build the condition
        let check_off = instructions[*k..]
            .iter()
            .filter_map(|i| {
                if let Instruction::Add(1, Value::Val(off)) = i {
                    Some(*off)
                } else {
                    None
                }
            })
            .next()
            .expect("No check offset");
        // the actual condition offset is the offset from the last element from z + the condition offset
        let off = d.map(|(_, off)| off + check_off).unwrap_or(check_off);

        if off > -9 && off < 9 {
            // if the offset is greater than -9 and less than 9, this results in a constraint comparing
            // two inputs input[n] = input[n1] + off. This is only valid, if there was a last element in z
            let (n1, _) = d.expect("Condition but no digit to compare");
            // normalize constraint to positive offsets
            constraints.push(if off > 0 { (n, n1, off) } else { (n1, n, -off) });

            // if constraint is satisfied, no additional element will be added to z
        } else {
            // the condition check outcome is independent of the input values, add next element to z
            // there will be one instruction 'add y w' followed by an instruction 'add y a' where
            // a is a literal value representing an offset
            let k_add = instructions[*k..]
                .iter()
                .position(|i| i == &Instruction::Add(2, Value::Var(0)))
                .expect("input is not copied to y");
            if let Instruction::Add(2, Value::Val(off)) = instructions[*k + k_add + 1] {
                z.push((n, off));
            } else {
                panic!("No offset added to digit {}", n);
            }
        }
    }

    // expect 7 constraints and z empty (equal to 0)
    assert_eq!(7, constraints.len());
    assert!(z.is_empty());

    // sort for easier comparison
    constraints.sort_unstable();

    constraints
}

pub fn solution_1(instructions: &[Instruction]) -> isize {
    let constraints = get_constraints(instructions);
    let mut inputs = vec![0; 14];
    for (_, n2, o) in &constraints {
        inputs[*n2] = 9 - o;
    }
    for (n1, n2, o) in &constraints {
        inputs[*n1] = inputs[*n2] + o;
    }

    assert_eq!(Some(0), eval(instructions, &inputs));

    inputs.iter().fold(0, |v, d| 10 * v + *d)
}

pub fn solution_2(instructions: &[Instruction]) -> isize {
    let constraints = get_constraints(instructions);
    let mut inputs = vec![0; 14];
    for (_, n2, _) in &constraints {
        inputs[*n2] = 1;
    }
    for (n1, n2, o) in &constraints {
        inputs[*n1] = inputs[*n2] + o;
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
