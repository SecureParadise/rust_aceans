package main

import "fmt"

func main() {
	language := struct {
		name   string
		isGood bool
	}{"hari", true}
	fmt.Println(language)

}
