package main

import (
	"fmt"
	"time"
)

func main() {
	// simple switch
	i := 5
	switch i {

	case 1:
		{
			fmt.Println("one")
		}
	case 2:
		{
			fmt.Println("two")
		}
	case 3:
		{
			fmt.Println("three")
		}
	case 4:
		{
			fmt.Println("four")
		}
	case 5:
		{
			fmt.Println("five")
		}
	default:
		{
			fmt.Println("default")
		}
	}

	switch time.Now().Weekday() {
	case time.Saturday, time.Sunday:
		{
			fmt.Println("It's weekend")
		}
	default:
		{
			fmt.Println("WORKDAY")
		}
	}

	// type switch
	whiAmI := func(i interface{}) {
		switch t := i.(type) {
		case int:
			fmt.Println("Its integer")
		case string:
			fmt.Println("String")
		case bool:
			fmt.Println("BOOLEAN")
		default:
			fmt.Println("Other", t)
		}
	}

	whiAmI("PEDA")
	whiAmI(1.2)
}
