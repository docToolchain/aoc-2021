package main

// tag::vec[]
// This file contains vector manipulation routines. All of the methods here always return a new
// vector, they never modify the original.

// DepthT describes the depth. To avoid intermingling depth and displacement, they are separate
// types.
type DepthT int

// DispT describes the depth. To avoid intermingling depth and displacement, they are separate
// types.
type DispT int

// AimT describes the aim.
type AimT int

// State describes the subs overall state, i.e. position and aim. It has depth, displacement, and
// aim.
type State struct {
	Depth DepthT
	Disp  DispT
	Aim   AimT
}

// Units contains unit vectors and name assignments.
var Units = map[string]State{
	"up": State{
		Depth: 0,
		Disp:  0,
		Aim:   -1,
	},
	"down": State{
		Depth: 0,
		Disp:  0,
		Aim:   +1,
	},
	"forward": State{
		// The depth change implicitly depends on the current aim.
		Depth: +1,
		Disp:  +1,
		Aim:   0,
	},
}

// Displace Displaces one vector by the distance specified by another. This takes the current aim
// into account. The new aim does not influence the depth change.
func (s State) Displace(delta State) State {
	result := State{
		Depth: s.Depth + delta.Depth*DepthT(s.Aim),
		Disp:  s.Disp + delta.Disp,
		Aim:   s.Aim + delta.Aim,
	}
	return result
}

// Mul multiplies each component of a vector with a number.
func (s State) Mul(factor int) State {
	result := State{
		Depth: DepthT(factor) * s.Depth,
		Disp:  DispT(factor) * s.Disp,
		Aim:   AimT(factor) * s.Aim,
	}
	return result
}

// Inv inverts a vector.
func (s State) Inv() State {
	return s.Mul(-1)
}

// Sub subtracts a vector's data from another one's.
func (s State) Sub(delta State) State {
	return s.Displace(delta.Inv())
}

// Area returns the area spanned by a vector. The aim does not matter for this compuation.
func (s State) Area() int {
	return int(s.Depth) * int(s.Disp)
}

// end::vec[]
