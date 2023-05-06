package util

import "golang.org/x/exp/constraints"

func Min[T constraints.Ordered](values ...T) T {
	var min T = values[0]

	for _, value := range values[1:] {
		if value < min {
			min = value
		}
	}

	return min
}

func Max[T constraints.Ordered](values ...T) T {
	var max T = values[0]

	for _, value := range values {
		if value > max {
			max = value
		}
	}

	return max
}

func Sum[T constraints.Integer](arr []T) T {
	var sum T

	for _, value := range arr {
		sum += value
	}

	return sum
}

func Product[T constraints.Integer](arr []T) T {
	var product T = 1

	for _, value := range arr {
		product *= value
	}

	return product
}
