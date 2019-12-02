package main

import "fmt"

func Part2(numbers []int) int {
	for x := 0; x < 99; x++ {
		for y := 0; y < 99; y++ {
			localNumbers := make([]int, len(numbers))
			copy(localNumbers, numbers)

			localNumbers[1] = x
			localNumbers[2] = y

			for i := 0; i < len(localNumbers); {
				firstIndex := localNumbers[i+1]
				secondIndex := localNumbers[i+2]
				resultIndex := localNumbers[i+3]

				firstValue := localNumbers[firstIndex]
				secondValue := localNumbers[secondIndex]

				var result int
				switch localNumbers[i] {
				case 1:
					result = firstValue + secondValue
					i += 4
				case 2:
					result = firstValue * secondValue
					i += 4
				default:
					i++
				}
				localNumbers[resultIndex] = result

				if result == 19690720 {
					fmt.Println(x)
					fmt.Println(y)
					return 100*x + y
				}
			}
		}
	}

	return -1
}
