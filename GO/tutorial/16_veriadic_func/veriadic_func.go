package main

import "fmt"

// func sum(nums ...int) int {
func sum(nums ...int) int {
	total := 0
	for _, num := range nums {
		total = total + num

	}
	return total
}
func main() {
	fmt.Println(1, 2, 3, 4, 5, "hello")
	num_slice := []int{1, 2, 3, 4, 5}
	// result := sum(1, 2, 3, 4, 5)
	result := sum(num_slice...)
	fmt.Println(result)
}
