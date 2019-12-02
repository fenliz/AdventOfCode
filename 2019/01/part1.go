package main

import (
	"bufio"
	"os"
	"strconv"
)

func Part1() int {
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)

	totalFuel := 0
	for scanner.Scan() {
		line := scanner.Text()
		mass, _ := strconv.Atoi(line)

		fuel := GetAmountOfFuelNeeded(mass)
		totalFuel = totalFuel + fuel
	}
	return totalFuel
}
