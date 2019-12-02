package main

import (
	"fmt"
)

func main() {
	fmt.Println(Part1())
	fmt.Println(Part2())
}

func GetAmountOfFuelNeeded(mass int) int {
	return int(float32(mass)/3.0 - 2)
}
