package main

// tag::package[]

const (
	// Numerical values, there are a lot of them this time.
	versionBits  = 3
	typeInfoBits = 3
	// Type information.
	literalType = 4
)

// Package describes a package.
type Package interface {
	Version() int
	Type() int
	SubPackages() []Package
	Value() int
}

// FromBitStream obtains a package from a bit stream. No padding on the stream is done. Will create
// operators or literals as mandated by the type information.
func FromBitStream(stream *BitStream) Package {
	version := BitsToInt(stream.Next(versionBits))
	typeInfo := BitsToInt(stream.Next(typeInfoBits))
	var pkg Package
	if typeInfo == literalType {
		lit := NewLiteral(version, typeInfo, stream)
		pkg = &lit
	} else {
		op := NewOperator(version, typeInfo, stream)
		pkg = &op
	}
	return pkg
}

// end::package[]
