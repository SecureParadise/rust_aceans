package main

import (
	"fmt"
	// "reflect"
	// "time"
)

//	func IncrementPassByPointer(ptr *int) {
//		fmt.Println("Value of ptr", ptr)
//		// fmt.Println("Addr of i :", &i)
//		(*(ptr))++
//	}
type student struct {
	rollNo int
	Name   string
}

func check() {
	bidiyarthi := student{1, "rama"}
	botal := &bidiyarthi
	fmt.Println("botal", botal)
	fmt.Println("botal.rollNo", botal.rollNo)
	fmt.Println("botal.Name", botal.Name)
	// fmt.Println(reflect.TypeOf(bidiyarthi))
}

func main() {
	check()
	// i := 10
	// fmt.Println("Value of i :", i)
	// fmt.Println("Addr of i :", &i)

	// IncrementPassByPointer(&i)
	// fmt.Println("Value of i :", i)
	// fmt.Println("Addr of i :", &i)

}
