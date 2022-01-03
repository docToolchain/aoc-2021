package main

import "fmt"

// tag::ops[]

const (
	// Ops.
	opInp = 'i'
	opAdd = 'a'
	opMul = 'm'
	opDiv = 'd'
	opMod = 'o'
	opEql = 'e'
	// Registers.
	inReg = 'w'
	regW  = 'w'
	regX  = 'x'
	regY  = 'y'
	regZ  = 'z'
)

type op struct {
	act rune
	reg rune
	dat interface{}
}

// String gets a string rep.
func (o op) String() string {
	str := fmt.Sprintf("%c -> %c", o.act, o.reg)
	switch t := o.dat.(type) {
	case int:
		str += fmt.Sprintf(" : %d", t)
	case rune:
		str += fmt.Sprintf(" : %c", t)
	}
	return str
}

// Compare two ops. If the data is nil, it is excluded from the comparison
func (o op) cmp(other op) bool {
	if o.act != other.act {
		return false
	}
	if o.reg != other.reg {
		return false
	}
	if o.dat != nil && o.dat != other.dat {
		return false
	}
	return true
}

// This is an example block of operations that shows the expected order of operations:
// mul x 0
// add x z
// mod x 26
// div z 26 => 26 is called "divVal" below, the value "nil" in the expected ops signifies this.
// add x -9 => -9 is called "addX" below, the value "nil" in the expected ops signifies this.
// eql x w
// eql x 0
// mul y 0
// add y 25
// mul y x
// add y 1
// mul z y
// mul y 0
// add y w
// add y 5 => 5 is called "addY" below, the value "nil" in the expected ops signifies this.
// mul y x
// add z y

// Convert a set of ops between input operations into a neat function.
//nolint:gomnd,funlen
func getFunc(ops []op) (fn, error) {
	var f fn
	vals := []int{}
	// Sanity check the expected order of operations. Also extract the values we need.
	expectedOps := []op{
		op{act: opMul, reg: regX, dat: 0},
		op{act: opAdd, reg: regX, dat: regZ},
		op{act: opMod, reg: regX, dat: 26},
		op{act: opDiv, reg: regZ, dat: nil},
		op{act: opAdd, reg: regX, dat: nil},
		op{act: opEql, reg: regX, dat: regW},
		op{act: opEql, reg: regX, dat: 0},
		op{act: opMul, reg: regY, dat: 0},
		op{act: opAdd, reg: regY, dat: 25},
		op{act: opMul, reg: regY, dat: regX},
		op{act: opAdd, reg: regY, dat: 1},
		op{act: opMul, reg: regZ, dat: regY},
		op{act: opMul, reg: regY, dat: 0},
		op{act: opAdd, reg: regY, dat: regW},
		op{act: opAdd, reg: regY, dat: nil},
		op{act: opMul, reg: regY, dat: regX},
		op{act: opAdd, reg: regZ, dat: regY},
	}
	if len(ops) != len(expectedOps) {
		return nil, fmt.Errorf("unqeual number of ops, cannot process")
	}
	for opIdx, inOp := range ops {
		expectedOp := expectedOps[opIdx]
		if !expectedOp.cmp(inOp) {
			return nil, fmt.Errorf(
				"unexpected op '%s' at line %d, wanted '%s'",
				inOp, opIdx+1, expectedOp,
			)
		}
		if expectedOp.dat == nil {
			converted, ok := inOp.dat.(int)
			if !ok {
				return nil, fmt.Errorf("failure in integer conversion")
			}
			vals = append(vals, converted)
		}
	}
	if len(vals) != 3 {
		return nil, fmt.Errorf("cannot extract enough variables from op input")
	}

	divVal := vals[0]
	addX := vals[1]
	addY := vals[2]

	// There are basically two different kinds of functions. Ones that divide z by 26 and ones that
	// divide z by 1 as per divVal. The first kind is a bit more complex and will involve comparing
	// the value of "z mod 26 + addX" with the input digit. The second type is a lot simpler. Using
	// some math, it can easily be deduced that those functions do not use the addX value at all.
	// The value will simply be consumed by the modulo or integer division operation, I can't recall
	// which one.
	if divVal == 1 {
		f = func(s acl, dig int) acl {
			// This function does not need the addX value.
			return acl{z: s.z*26 + dig + addY}
		}
	} else {
		f = func(s acl, dig int) acl {
			var val int
			if s.z%26+addX != dig {
				val = 1
			}
			return acl{z: s.z/26*(25*val+1) + (dig+addY)*val}
		}
	}
	return f, nil
}

//nolint:nestif
func opsToFuncs(ops []op) ([]fn, error) {
	funcs := []fn{}
	// Separate ops into partitions that start with an input operation. Furthermore, sanity check
	// whether all input happens to the w register.
	partition := []op{}
	for _, o := range ops {
		if o.act == opInp {
			if o.reg != inReg {
				return []fn{}, fmt.Errorf("input to a non-w register detected")
			}
			if len(partition) > 0 {
				f, err := getFunc(partition)
				if err != nil {
					return []fn{}, err
				}
				funcs = append(funcs, f)
				partition = []op{}
			}
		} else {
			partition = append(partition, o)
		}
	}
	f, err := getFunc(partition)
	if err != nil {
		return []fn{}, err
	}
	funcs = append(funcs, f)

	return funcs, nil
}

// end::ops[]
