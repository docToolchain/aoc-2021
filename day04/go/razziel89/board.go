package main

import (
	"fmt"
)

// tag::board[]

// Field is a field of a bingo board.
type Field struct {
	val    int
	marked bool
}

func fieldsFromInts(ints []int) []Field {
	fields := make([]Field, 0, len(ints))
	for _, val := range ints {
		newField := Field{
			val:    val,
			marked: false,
		}
		fields = append(fields, newField)
	}
	return fields
}

func winningSet(fields []Field) bool {
	for _, f := range fields {
		if !f.marked {
			return false
		}
	}
	return true
}

// Board is a bingo board.
type Board struct {
	fields [][]Field
	last   int
}

// IsComplete determines whether a board has as many rows as cols.
func (b Board) IsComplete() bool {
	if len(b.fields) == 0 {
		return false
	}
	// A square board is considered complete.
	return len(b.fields) == len(b.fields[0])
}

// AddRow adds a row to a board.
func (b *Board) AddRow(input []int) error {
	if len(input) == 0 {
		// Ignore empty lines as a convenience feature.
		return nil
	}
	if len(b.fields) > 0 && len(input) != len(b.fields[0]) {
		return fmt.Errorf("cannot process row of length %d, require %d", len(input), len(b.fields))
	}
	fields := fieldsFromInts(input)
	b.fields = append(b.fields, fields)
	return nil
}

// Row gets the row with the specified index. If the index is out of range, an empty slice is
// returned.
func (b Board) Row(idx int) []Field {
	if idx < 0 || idx+1 > len(b.fields) {
		return []Field{}
	}
	// This is easy.
	result := make([]Field, len(b.fields))
	_ = copy(result, b.fields[idx])
	return result
}

// Col gets the column with the specified index. If the index is out of range, an empty slice is
// returned.
func (b Board) Col(idx int) []Field {
	if idx < 0 || idx+1 > len(b.fields) {
		return []Field{}
	}
	// This is less easy.
	result := make([]Field, 0, len(b.fields))
	for _, row := range b.fields {
		result = append(result, row[idx])
	}
	return result
}

// Mark marks a number and returns whether the board had the number. All occurrences are marked.
func (b *Board) Mark(num int) bool {
	found := false
	for _, row := range b.fields {
		for _, field := range row {
			if field.val == num {
				field.marked = true
			}
		}
	}
	if found {
		b.last = num
	}
	return found
}

// Sum sums up all numbers. The value of `marked` determines whether marked or unmarked unes are
// summed up.
func (b Board) Sum(marked bool) int {
	sum := 0
	for _, row := range b.fields {
		for _, field := range row {
			if field.marked == marked {
				sum += field.val
			}
		}
	}
	return sum
}

// Score determines whether this is a winning board by returning the score. A non-winning board has
// score zero.
func (b Board) Score() int {
	for idx := 0; idx < len(b.fields); idx++ {
		if winningSet(b.Row(idx)) || winningSet(b.Col(idx)) {
			score := b.last * b.Sum(false)
			return score
		}
	}
	return 0
}

// Pretty makes a pretty string representation for this board.
func (b Board) Pretty() string {
	// Hard-code formatting helper strings.
	pre := " "
	post := " "
	sep := " "
	marker := "X"
	clear := " "
	formatter := "%-4s"
	// Actually build the representation.
	result := ""
	for _, row := range b.fields {
		result += pre
		for _, field := range row {
			fieldRep := fmt.Sprintf("%d", field.val)
			if field.marked {
				fieldRep += marker
			} else {
				fieldRep += clear
			}
			fieldRep = fmt.Sprintf(formatter, fieldRep)
			result += sep + fieldRep
		}
		result += post
		result += "\n"
	}
	return result
}

// end::board[]
