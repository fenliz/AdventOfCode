package main

import (
	"bufio"
	"os"
	"strconv"
)

func Part2() int {
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	totalFuel := 0
	for scanner.Scan() {
		line := scanner.Text()
		mass, _ := strconv.Atoi(line)

		for {
			fuel := GetAmountOfFuelNeeded(mass)
			if fuel <= 0 {
				break
			}
			totalFuel = totalFuel + fuel
			mass = fuel
		}

	}
	return totalFuel
}
