// tag::instructions[]
/// Value type
pub type Val = i32;
/// Memory of variables
pub type Memory = [Val; 4];

/// Variable
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Var {
    W,
    X,
    Y,
    Z,
}

impl std::ops::Index<Var> for Memory {
    type Output = Val;

    fn index(&self, index: Var) -> &Self::Output {
        match index {
            Var::W => &self[0],
            Var::X => &self[1],
            Var::Y => &self[2],
            Var::Z => &self[3],
        }
    }
}

impl std::ops::IndexMut<Var> for Memory {
    fn index_mut(&mut self, index: Var) -> &mut Self::Output {
        match index {
            Var::W => &mut self[0],
            Var::X => &mut self[1],
            Var::Y => &mut self[2],
            Var::Z => &mut self[3],
        }
    }
}

impl std::str::FromStr for Var {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Var::W),
            "x" => Ok(Var::X),
            "y" => Ok(Var::Y),
            "z" => Ok(Var::Z),
            _ => Err(()),
        }
    }
}

/// Second operand (Variable or Literal)
#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Var(Var),
    Val(Val),
}

impl Op {
    /// evaluate operand
    pub fn eval(&self, vars: &Memory) -> Val {
        match self {
            Op::Var(v) => vars[*v],
            Op::Val(v) => *v,
        }
    }
}

impl std::str::FromStr for Op {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(v) = s.parse() {
            Ok(Op::Var(v))
        } else {
            s.parse().map(|v| Op::Val(v))
        }
    }
}

/// instruction
#[derive(Debug, PartialEq, Eq)]
pub enum Inst {
    Inp(Var),
    Add(Var, Op),
    Mul(Var, Op),
    Div(Var, Op),
    Mod(Var, Op),
    Eql(Var, Op),
}

impl Inst {
    /// execute instruction
    pub fn execute(&self, vars: &mut Memory, inputs: &mut impl Iterator<Item = Val>) {
        match self {
            Inst::Inp(v1) => vars[*v1] = inputs.next().expect("No input"),
            Inst::Add(v1, v2) => vars[*v1] += v2.eval(vars),
            Inst::Mul(v1, v2) => vars[*v1] *= v2.eval(vars),
            Inst::Div(v1, v2) => vars[*v1] /= v2.eval(vars),
            Inst::Mod(v1, v2) => vars[*v1] = vars[*v1] % v2.eval(vars),
            Inst::Eql(v1, v2) => vars[*v1] = (vars[*v1] == v2.eval(vars)) as Val,
        }
    }
}

impl std::str::FromStr for Inst {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(" ");

        let name = parts
            .next()
            .ok_or_else(|| format!("'{}' contains no name part", s))?;
        let first = parts
            .next()
            .ok_or_else(|| format!("'{}' contains no first operand", s))?
            .parse()
            .map_err(|()| format!("First part of '{}' is not a variable", s))?;
        let second = parts.next().map_or(Ok(None), |s| s.parse().map(Some))?;
        let second = || second.ok_or_else(|| format!("'{}' contains no second operand", s));

        match name {
            "inp" => Ok(Self::Inp(first)),
            "add" => Ok(Self::Add(first, second()?)),
            "mul" => Ok(Self::Mul(first, second()?)),
            "div" => Ok(Self::Div(first, second()?)),
            "mod" => Ok(Self::Mod(first, second()?)),
            "eql" => Ok(Self::Eql(first, second()?)),
            _ => Err(format!("'{}' is not a valid instruction name", name))?,
        }
    }
}

/// parse lines to instructions
pub fn parse(content: &str) -> Vec<Inst> {
    content.lines().map(|line| line.parse().unwrap()).collect()
}

/// execute instructions
pub fn execute(instructions: &[Inst], inputs: &[Val]) -> Val {
    let mut vars = Memory::default();
    let mut inputs = inputs.iter().copied();
    for instruction in instructions {
        instruction.execute(&mut vars, &mut inputs)
    }
    vars[Var::Z]
}
// end::instructions[]

// tag::solution[]
pub const DIGIT_MIN: Val = 1;
pub const DIGIT_MAX: Val = 9;

pub const INPUT_LEN: usize = 14;

pub fn get_constraints(instructions: &[Inst]) -> Vec<(usize, usize, Val)> {
    let block_starts = instructions
        .iter()
        .enumerate()
        .filter(|(_, i)| i == &&Inst::Inp(Var::W))
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    assert_eq!(INPUT_LEN, block_starts.len());

    // determine module used to build stack
    let module = instructions
        .iter()
        .filter_map(|i| {
            if let Inst::Mod(Var::X, Op::Val(m)) = i {
                Some(*m)
            } else {
                None
            }
        })
        .next()
        .expect("No module");

    // stack of elements (n, o) where n is an input number and o an offset
    // the actual value of the z variable will be
    // stack.iter().fold(0, |z, (n, o)| z * module + input[n] + o);
    let mut stack: Vec<(usize, Val)> = Vec::with_capacity(INPUT_LEN);

    // constraints holds elements (n1, n2, o) and represents a constraint
    // input[n1] == input[n2] + o, which needs to be satisfied for a valid serial number
    let mut constraints = Vec::with_capacity(INPUT_LEN / 2);

    // iterate over blocks
    for (n1, k) in block_starts.iter().enumerate() {
        // take last element of stack
        // a single 'div z a' instruction is expected per block where a is either
        // the module or 1. In case of a == module, the last element from z is removed
        let element = if instructions[*k..]
            .iter()
            .filter_map(|i| {
                if let Inst::Div(Var::Z, Op::Val(div)) = i {
                    Some(*div)
                } else {
                    None
                }
            })
            .next()
            .expect("No div")
            == module
        {
            // z is divided by 26, pop element
            stack.pop()
        } else {
            // z is divided by 1, peek element
            stack.last().map(|e| *e)
        };

        // a single instruction 'add x a' where a is a literal value is expected in a block
        // the value a is used to build the condition
        let off = instructions[*k..]
            .iter()
            .filter_map(|i| {
                if let Inst::Add(Var::X, Op::Val(o)) = i {
                    Some(*o)
                } else {
                    None
                }
            })
            .next()
            .expect("No check offset");

        // the actual condition offset is the offset from the last element from stack + the
        // condition offset
        let off = element.map(|(_, o)| off + o).unwrap_or(off);

        if off > -DIGIT_MAX && off < DIGIT_MAX {
            // if the offset is greater than -9 and less than 9, this results in a constraint comparing
            // two inputs input[n] = input[n1] + off. This is only valid, if there was a last element on
            // stack
            let (n2, _) = element.expect("Condition but no input digit to compare");

            // normalize constraint to positive offsets
            constraints.push(if off > 0 {
                (n1, n2, off)
            } else {
                (n2, n1, -off)
            });

            // if constraint is satisfied, no additional element will be added to stack
        } else {
            // the condition check outcome is independent of the input values, add next element to stack
            // there will be one instruction 'add y w' followed by an instruction 'add y a' where
            // a is a literal value representing an offset
            let k_add = instructions[*k..]
                .iter()
                .position(|i| i == &Inst::Add(Var::Y, Op::Var(Var::W)))
                .expect("input is not copied to y");

            if let Inst::Add(Var::Y, Op::Val(off)) = instructions[*k + k_add + 1] {
                assert!(off >= 0 && off < module - DIGIT_MAX, "Input does not fit");
                stack.push((n1, off));
            } else {
                panic!("No offset added to digit {}", n1);
            }
        }
    }

    // expect 7 constraints and stack empty (z equal to 0)
    assert_eq!(INPUT_LEN / 2, constraints.len());
    assert!(stack.is_empty());

    constraints
}

pub fn solution_1(instructions: &[Inst]) -> u64 {
    let inputs = &mut [0; INPUT_LEN];
    for (n1, n2, o) in get_constraints(instructions) {
        inputs[n1] = DIGIT_MAX;
        inputs[n2] = DIGIT_MAX - o;
    }

    assert_eq!(0, execute(instructions, inputs));

    inputs.iter().fold(0, |v, d| 10 * v + *d as u64)
}

pub fn solution_2(instructions: &[Inst]) -> u64 {
    let inputs = &mut [0; INPUT_LEN];
    for (n1, n2, o) in get_constraints(instructions) {
        inputs[n1] = DIGIT_MIN + o;
        inputs[n2] = DIGIT_MIN;
    }

    assert_eq!(0, execute(instructions, inputs));

    inputs.iter().fold(0, |v, d| 10 * v + *d as u64)
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
    const INSTRUCTIONS: &[Inst] = &[
        Inst::Inp(Var::Z),
        Inst::Inp(Var::X),
        Inst::Mul(Var::Z, Op::Val(3)),
        Inst::Eql(Var::Z, Op::Var(Var::X)),
    ];

    #[test]
    fn test_parse() {
        assert_eq!(INSTRUCTIONS, parse(CONTENT));
    }
}
// end::tests[]
