package main

func Part2(numbers []int) int {
	for i := 0; i < len(numbers); {
		firstIndex := numbers[i+1]
		secondIndex := numbers[i+2]
		resultIndex := numbers[i+3]

		firstValue := numbers[firstIndex]
		secondValue := numbers[secondIndex]

		var result int
		switch numbers[i] {
		case 1:
			result = firstValue + secondValue
			i += 4
		case 2:
			result = firstValue * secondValue
			i += 4
		default:
			i++
		}
		numbers[resultIndex] = result

		if result == 19690720 {
			return 100*firstValue + secondValue
		}
	}
	return -1
}
