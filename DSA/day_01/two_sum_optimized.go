package main

import "fmt"

// question link:= 		https://leetcode.com/problems/two-sum/description/
// solution
func twoSum(nums []int, target int) []int {
	// create a map
	customMapTable := make(map[int]int)
	// iterate through array
	for i := 0; i < len(nums); i++ {
		first := nums[i]
		second := target - first
		// index,found := customMapTable[second];
		// index is the value and key is element in array nums
		// it will return value_of_key_second and if key second is present then return true in found ie found=true else false i.e found = false
		index, found := customMapTable[second]
		if found {
			return []int{i, index}
		}
		customMapTable[first] = i
	}
	return []int{}
}

func main() {
	nums := []int{2, 7, 11, 15}
	target := 18
	fmt.Println(twoSum(nums, target))
}
