package main

import "fmt"

func add(a int, b int) int {
	return a + b
}
func sub(a, b int) int {
	return a - b
}

func getLang() (string, string, string) {
	return "golang", "js", "c"
}

func main() {
	addRes := add(2, 3)
	fmt.Println(addRes)

	subRes := sub(2, 3)
	fmt.Println(subRes)
	// _ to supress value and error
	// function are 1at class citizen
	lang1, lang2, _ := getLang()
	fmt.Println(lang1, lang2)
	fmt.Println(getLang())
}
