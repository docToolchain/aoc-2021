// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
)

const (
	// Our algorithm does not end by itself since we can never really be sure that we have found the
	// highest y-velocity that will still result in a hit. Thus, we end after this many velocities
	// at most. Empirically, this has been proven to be enough.
	maxNumVelocities = 1000000
	buffer           = 100
	drag             = -1
	gravity          = -1
)

// tag::solution[]

// Generate valid velocity vectors up to a maximum x value and a minimum y value.
func generateVelocities(xMax, yMin int) <-chan Vec {
	channel := make(chan Vec, buffer)

	go func() {
		numVelocities := 0
		// Start with a Manhattan distance of 1 and successively create all values in the allowed
		// range.
		distance := 1
		for {
			// For all smaller x, explore only y==+distance and y==-distance.
			for x := 0; x < distance; x++ {
				channel <- Vec{x, distance}
				numVelocities++
				if -distance >= yMin {
					channel <- Vec{x, -distance}
					numVelocities++
				}
			}
			if distance <= xMax {
				// For x==distance, explore all y in [-distance, +distance].
				for y := -distance; y <= distance; y++ {
					channel <- Vec{distance, y}
					numVelocities++
				}
			}
			distance++
			// It is not guaranteed that we have found the solution, but we did ^^. So this hack is
			// fine.
			if numVelocities > maxNumVelocities {
				break
			}
		}
		close(channel)
	}()

	return channel
}

func generateTrajectory(input <-chan Vec, drag, gravity int, area Area) <-chan int {
	channel := make(chan int, buffer)

	go func() {
		// Obtain one velocity vector and follow its trajectory.
		for orgVel := range input {
			vel := orgVel
			if vel.x < 0 {
				log.Fatal("negative x velocity detected")
			}
			pos := Vec{0, 0}
			trackedHeight := pos.y
			inside := area.Inside(pos)
			invalid := area.Invalid(pos)
			step := 0
			traj := []Vec{pos}
			for !inside && !invalid {
				// Explore the trajectory based on it.
				// We explore as long as our x position is not higher than the max and our y
				// position is not lower than the min.

				// Generate the next step in the trajectory.
				pos = pos.Add(vel)
				step++
				traj = append(traj, pos)
				// Update the velocity. We will never have negative x velocities.
				if vel.x > 0 {
					vel.x += drag
				}
				vel.y += gravity

				if pos.y > trackedHeight {
					trackedHeight = pos.y
				}

				// Check whether we are still on track to possibly hit the area.
				inside = area.Inside(pos)
				invalid = area.Invalid(pos)
			}
			// Don't return invalid data.
			if inside && !invalid {
				channel <- trackedHeight
			}
		}
		close(channel)
	}()

	return channel
}

func main() {
	areas, err := ReadLinesAsAreas()
	if err != nil {
		log.Fatal(err.Error())
	}
	if len(areas) != 1 {
		log.Fatal("only one area expected")
	}
	area := areas[0]
	if area.xMax < 0 || area.xMin < 0 {
		log.Fatal("areas with negative x values not supported")
	}
	velocities := generateVelocities(area.xMax, area.yMin)
	trajectories := generateTrajectory(velocities, drag, gravity, area)

	count := 1
	maxHeight := <-trajectories
	for height := range trajectories {
		count++
		if height > maxHeight {
			maxHeight = height
		}
		fmt.Println("Max valid height is", maxHeight, "and valid trajectories found are", count)
	}
}

// end::solution[]
