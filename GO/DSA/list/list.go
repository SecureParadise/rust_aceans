package main

import (
	"container/list"
	"fmt"
)

func main() {
	var myList list.List
	myList.PushBack(1)
	myList.PushBack(12)
	myList.PushBack(32)
	myList.PushBack(31)
	// fmt.Println(myList)
	for element := myList.Back(); element != nil; element = element.Prev() {
		fmt.Println(element.Value.(int))
	}
}
