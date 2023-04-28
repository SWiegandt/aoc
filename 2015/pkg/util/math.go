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
