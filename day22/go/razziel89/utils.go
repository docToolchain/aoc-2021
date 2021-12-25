package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

const (
	cuboidFormat = "%s x=%d..%d,y=%d..%d,z=%d..%d"
	parsedVals   = 7
	onStr        = "on"
	offStr       = "off"
)

var reader = bufio.NewReader(os.Stdin)

// Function readLine reads one line from stdin via a global reader instance.
func readLine() (string, error) {
	return reader.ReadString('\n')
}

// tag::utils[]

// ReadLinesAsCuboids reads lines in as cuboids.
func ReadLinesAsCuboids() ([]Cuboid, []bool, error) {
	result := []Cuboid{}
	switches := []bool{}
	for {
		line, err := readLine()
		if err == io.EOF {
			// Success case, no more input to read.
			return result, switches, nil
		}
		if err != nil {
			return []Cuboid{}, []bool{}, err
		}
		line = strings.TrimSpace(line)
		//nolint:nestif
		var onOff string
		var x1, x2, y1, y2, z1, z2 int
		count, err := fmt.Sscanf(line, cuboidFormat, &onOff, &x1, &x2, &y1, &y2, &z1, &z2)
		if err != nil {
			return []Cuboid{}, []bool{}, err
		}
		if count != parsedVals {
			return []Cuboid{}, []bool{}, fmt.Errorf("parsed wrong number of values")
		}
		if onOff != onStr && onOff != offStr {
			return []Cuboid{}, []bool{}, fmt.Errorf("icorrect on-off string")
		}
		cub := NewCuboid(x1, x2+1, y1, y2+1, z1, z2+1)
		result = append(result, cub)
		switches = append(switches, onOff == onStr)
	}
}

// end::utils[]
