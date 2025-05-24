package main

import "fmt"

func main() {
	var nums [4]int
	// add element to array
	nums[0] = 1
	nums[1] = 2
	nums[2] = 3
	nums[3] = 4
	fmt.Println(nums)
	fmt.Println(nums[2])
	// array length
	// fmt.Println(len(nums))

	// initilze array
	floatArray := [5]float32{1.2, 2.2, 3.2, 4.2, 10.2}
	fmt.Println(floatArray)
	//2d array
	// array2d := [3][3]int{{3, 4, 9}, {5, 6, 7}}
	var array2d [3][3]int
	fmt.Println(array2d)
}
