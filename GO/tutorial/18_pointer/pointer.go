package main

import "fmt"

// pass by value
// func changeNum(num int) {
// pass by refrence i.e address
func changeNum(num *int) {
	*num = 5
	fmt.Println("IN CHANGEnum", *num)
}
func main() {
	num := 1
	fmt.Println("mem adderss", &num)
	changeNum(&num)
	fmt.Println("main", num)
}
