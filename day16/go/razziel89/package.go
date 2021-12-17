package main

const (
	// Numerical values, there are a lot of them this time.
	versionBits              = 3
	typeInfoBits             = 3
	bitsPerLiteralSet        = 4
	totalLengthModeBits      = 15
	totalNumPackagesModeBits = 11
	// Type information.
	literalType = 4
)

// Package describes a package.
type Package interface {
	Version() int
	Type() int
	SubPackages() []Package
}

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
	// result := make([]Package, 0, len(p.subPackages))
	// for _, pkg := range p.subPackages {
	// 	result = append(result, pkg)
	// }
	// return result
	// Literal packages have no sub-packages.
	return []Package{}
}

func newLiteral(version, typeInfo int, stream *BitStream) Literal {
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

// Operator is an actual implementation of the package interface, operator value.
type Operator struct {
	version     int
	typeInfo    int
	subPackages []Package
}

// Version provides the version number.
func (p *Operator) Version() int {
	return p.version
}

// Type provides the type information.
func (p *Operator) Type() int {
	return p.typeInfo
}

// SubPackages provides a list of sub-packages.
func (p *Operator) SubPackages() []Package {
	result := make([]Package, 0, len(p.subPackages))
	//nolint:gosimple
	for _, pkg := range p.subPackages {
		result = append(result, pkg)
	}
	return result
}

func newOperator(version, typeInfo int, stream *BitStream) Operator {
	subPackages := []Package{}
	modeBit := stream.Next(1)
	if !modeBit[0] {
		// Total length mode.
		totalLength := BitsToInt(stream.Next(totalLengthModeBits))
		shortStream := stream.LimitedStream(totalLength)
		for !shortStream.Done() {
			// No padding in between according to the task.
			pkg := FromBitStream(&shortStream)
			subPackages = append(subPackages, pkg)
		}
	} else {
		// Number of packages mode.
		numPkgs := BitsToInt(stream.Next(totalNumPackagesModeBits))
		for count := 0; count < numPkgs; count++ {
			// No padding in between according to the task.
			pkg := FromBitStream(stream)
			subPackages = append(subPackages, pkg)
		}
	}
	return Operator{
		version:     version,
		typeInfo:    typeInfo,
		subPackages: subPackages,
	}
}

// FromBitStream obtains a package from a bit stream. No padding on the stream is done.
func FromBitStream(stream *BitStream) Package {
	version := BitsToInt(stream.Next(versionBits))
	typeInfo := BitsToInt(stream.Next(typeInfoBits))
	var pkg Package
	if typeInfo == literalType {
		lit := newLiteral(version, typeInfo, stream)
		pkg = &lit
	} else {
		op := newOperator(version, typeInfo, stream)
		pkg = &op
	}
	return pkg
}
