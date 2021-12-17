package main

// tag::operator[]

const (
	// Numerical values, there are a lot of them this time.
	totalLengthModeBits      = 15
	totalNumPackagesModeBits = 11
)

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
	return p.subPackages
}

// Value determines the value of this package.
//nolint:funlen
func (p *Operator) Value() int {
	//nolint:gomnd
	switch p.Type() {
	case 0:
		// Packets with type ID 0 are sum packets - their value is the sum of the values of their
		// sub-packets. If they only have a single sub-packet, their value is the value of the
		// sub-packet.
		result := 0
		for _, pkg := range p.SubPackages() {
			result += pkg.Value()
		}
		return result
	case 1:
		// Packets with type ID 1 are product packets - their value is the result of multiplying
		// together the values of their sub-packets. If they only have a single sub-packet, their
		// value is the value of the sub-packet.
		//
		// Treat packages without sub-packages as having no value. This should not happen for prduct
		// packages.
		if len(p.subPackages) == 0 {
			return 0
		}
		result := 1
		for _, pkg := range p.subPackages {
			result *= pkg.Value()
		}
		return result
	case 2:
		// Packets with type ID 2 are minimum packets - their value is the minimum of the values of
		// their sub-packets.
		result := 0
		found := false
		for _, pkg := range p.subPackages {
			val := pkg.Value()
			if !found || val < result {
				result = val
				found = true
			}
		}
		return result
	case 3:
		// Packets with type ID 3 are maximum packets - their value is the maximum of the values of
		// their sub-packets.
		result := 0
		found := false
		for _, pkg := range p.subPackages {
			val := pkg.Value()
			if !found || val > result {
				result = val
				found = true
			}
		}
		return result
	case 5:
		// Packets with type ID 5 are greater than packets - their value is 1 if the value of the
		// first sub-packet is greater than the value of the second sub-packet; otherwise, their
		// value is 0.  These packets always have exactly two sub-packets.
		result := 0
		if len(p.subPackages) == 2 && p.subPackages[0].Value() > p.subPackages[1].Value() {
			result = 1
		}
		return result
	case 6:
		// Packets with type ID 6 are less than packets - their value is 1 if the value of the first
		// sub-packet is less than the value of the second sub-packet; otherwise, their value is 0.
		// These packets always have exactly two sub-packets.
		result := 0
		if len(p.subPackages) == 2 && p.subPackages[0].Value() < p.subPackages[1].Value() {
			result = 1
		}
		return result
	case 7:
		// Packets with type ID 7 are equal to packets - their value is 1 if the value of the first
		// sub-packet is equal to the value of the second sub-packet; otherwise, their value is 0.
		// These packets always have exactly two sub-packets.
		result := 0
		if len(p.subPackages) == 2 && p.subPackages[0].Value() == p.subPackages[1].Value() {
			result = 1
		}
		return result
	default:
		// Should never happen.
		return 0
	}
}

// NewOperator creates a new literal package. Version and type information have to be provided.
func NewOperator(version, typeInfo int, stream *BitStream) Operator {
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

// end::operator[]
