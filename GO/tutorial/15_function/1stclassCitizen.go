package main

import "fmt"

func processIt(fn func(a int) int) {
	fn(1)
}

func takenothing() func(_a int) int {
	return func(_a int) int {
		return 4
	}
}

func main() {
	fn := func(a int) int {
		return 2
	}
	fmt.Println(processIt(fn))
	leliya := takenothing(10)
	fmt.Println(leliya)
}
