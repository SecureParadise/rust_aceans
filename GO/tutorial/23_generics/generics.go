package main

import "fmt"

/*
func printSlice(items []int) {
	for _, item := range items {
		fmt.Println(item)
	}
}
func print_string_Slice(items []string) {
	for _, item := range items {
		fmt.Println(item)
	}
}
*/

// Generic,we can assign any value instead of T but the convention is "T"
// func generic_print_slice[T interface{}](items []T) {
// func generic_print_slice[T any](items []T) {
// func generic_print_slice[T int | string | bool](items []T) {

func generic_print_slice[T comparable, V string](items []T, name V) {
	for _, item := range items {
		fmt.Println(item, name)
	}
}

// func generic_print_slice[T int | string | bool](items []T) {
// 	for _, item := range items {
// 		fmt.Println(item)
// 	}
// }

// type stack struct {
type stack[T any] struct {
	// elements []int
	elements []T
}

// func (s stack)
func main() {
	nums := []int{1, 2, 3, 4}
	// names := []string{"john", "cick", "ravi", "bad"}
	// // print_string_Slice(names)
	// fmt.Println("INT")
	generic_print_slice(nums, "yoda")
	// fmt.Println("STRING")
	// generic_print_slice(names)

	// myStack := stack[int]{
	// 	elements: []int{1, 2, 3},
	// }
	myStack_str := stack[string]{
		elements: []string{"amaresh", "thakur"},
	}
	fmt.Println(myStack_str)
}
