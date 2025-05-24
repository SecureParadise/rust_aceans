package main

import "fmt"

func main() {
	nums := []int{6, 7, 8}
	sum := 0
	// for _, num := range nums {
	for index, num := range nums {
		sum = sum + num
		// fmt.Println("index", index, "Sum", sum) //-- valid
		// fmt.Println("index {index} Sum {sum}") //--Not vatld
		fmt.Printf("index %d Sum %d\n", index, sum)
	}
	range_with_map := map[string]string{"fname": "JoHN", "lanme": "don"}
	fmt.Println("KEY VALUE")
	for anyKay, keyValue := range range_with_map {
		fmt.Println(anyKay, keyValue)
	}
	fmt.Println("KEY")
	for anyKay := range range_with_map {
		fmt.Println(anyKay)
	}
	fmt.Println("KEY")
	for keyValue := range range_with_map {
		fmt.Println(keyValue)
	}
	// unicode code point rune
	//  i-> index
	//  c -> unocode value
	for i, c := range "golang" {
		fmt.Println(i, c)
		fmt.Println(i, string(c))
	}
}
