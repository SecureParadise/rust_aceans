package main

import "fmt"

func main() {
	m := make(map[string]string)
	m["name"] = "goolu"
	fmt.Println(m["name"])
	fmt.Println(len(m))

	delete(m, "price")
	clear(m)
	naksha := map[string]int{"price": 40, "phones": 3}
	fmt.Println("phones naksha", naksha)

	value_of_key_phones, Ok := naksha["phones"]

	fmt.Println(value_of_key_phones)
	if Ok {
		fmt.Println("all ok")
	} else {
		fmt.Println("not pk")
	}
}
