package main

func Part1(numbers []int) int {
Loop:
	for i := 0; i < len(numbers); i += 4 {
		firstIndex := numbers[i+1]
		secondIndex := numbers[i+2]
		resultIndex := numbers[i+3]

		firstValue := numbers[firstIndex]
		secondValue := numbers[secondIndex]

		var result int
		switch numbers[i] {
		case 1:
			result = firstValue + secondValue
		case 2:
			result = firstValue * secondValue
		default:
			break Loop
		}
		numbers[resultIndex] = result
	}

	return numbers[0]
}
