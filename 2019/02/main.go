package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, _ := os.Open("input.txt")
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	text := scanner.Text()

	stringValues := strings.Split(text, ",")
	numbers := make([]int, len(stringValues))
	for i := range stringValues {
		numbers[i], _ = strconv.Atoi(stringValues[i])
	}

	numbers[1] = 12
	numbers[2] = 2

	numbers2 := make([]int, len(numbers))
	copy(numbers2, numbers)

	fmt.Println("Part1:", Part1(numbers))
	fmt.Println("Part2:", Part2(numbers2))
}
