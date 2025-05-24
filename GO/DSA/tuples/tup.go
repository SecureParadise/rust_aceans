package main

import "fmt"

func powerseries(a int) (int, int) {
	return a * a, a * a * a
}
func main() {
	powerseries(3)
	fmt.Println(powerseries(3))
}
