package main

const (
	// Numerical values, there are a lot of them this time.
	bitsPerLiteralSet = 4
)

// Literal is an actual implementation of the package interface, literal value.
type Literal struct {
	version  int
	typeInfo int
	value    int
	// subPackages []*Literal
}

// Version provides the version number.
func (p *Literal) Version() int {
	return p.version
}

// Type provides the type information.
func (p *Literal) Type() int {
	return p.typeInfo
}

// Value provides the value information.
func (p *Literal) Value() int {
	return p.value
}

// SubPackages provides a list of sub-packages.
func (p *Literal) SubPackages() []Package {
	// Literal packages have no sub-packages. To implement the interface, we return an empty slice
	// here.
	return []Package{}
}

// NewLiteral creates a new literal package. Version and type information have to be provided.
func NewLiteral(version, typeInfo int, stream *BitStream) Literal {
	bits := []bool{}
	for notLastGroup := stream.Next(1); notLastGroup[0]; notLastGroup = stream.Next(1) {
		bits = append(bits, stream.Next(bitsPerLiteralSet)...)
	}
	bits = append(bits, stream.Next(bitsPerLiteralSet)...)
	num := BitsToInt(bits)
	return Literal{
		version:  version,
		typeInfo: typeInfo,
		value:    num,
	}
}
