package main

import "fmt"

func main() {
	i := 1
	for i <= 3 {
		fmt.Println(i)
		i = i + 1
	}
	// classic for
	fmt.Println("CLASSIC START")
	for j := 0; j < 4; j++ {
		fmt.Println(j)
	}
	fmt.Println("CLASSIC END")
	// range
	for i := range 4 {
		fmt.Println(i)
	}
}

// go have only for loo
