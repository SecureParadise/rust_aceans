package main

import (
	"fmt"
)

func twoSum(nums []int, target int) []int {
	for i := 0; i < len(nums); i++ {
		first := nums[i]
		for j := i + 1; j < len(nums); j++ {
			second := nums[j]
			sum := first + second
			if sum == target {
				return []int{i, j}
			}

		}
	}
	return []int{}
}

func main() {
	nums := []int{2, 7, 11, 15}
	target := 18
	fmt.Println(twoSum(nums, target))
}
