package main

import "fmt"

func main() {
	fmt.Println("SLICES GO")
	// dynamic array
	// most used construct in go
	// + useful methods

	// uninitilized slice is nil
	var slice_num []int
	fmt.Println(slice_num)
	fmt.Println(slice_num == nil)
	// length of slice
	fmt.Println("LENGTH")
	fmt.Println(len(slice_num))

	var nums = make([]int, 0, 5)
	// initial size 0,ie zero element,capacity 5
	// fmt.Println(nums)
	// capacity of dynamic array
	// fmt.Println(cap(nums))
	// nums
	nums = append(nums, 1)
	nums = append(nums, 2)
	nums = append(nums, 3)
	nums = append(nums, 4)
	nums = append(nums, 1)
	nums = append(nums, 2)
	nums = append(nums, 3)
	nums = append(nums, 4)
	nums = append(nums, 1)
	nums = append(nums, 2)
	nums = append(nums, 3)
	nums = append(nums, 4)
	nums = append(nums, 3)
	nums = append(nums, 4)
	nums = append(nums, 1)
	nums = append(nums, 2)
	nums = append(nums, 3)
	nums = append(nums, 4)
	nums = append(nums, 4)
	fmt.Println(nums)
	fmt.Println(cap(nums))
}
