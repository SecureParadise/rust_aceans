package main

import (
	"container/list"
	"fmt"
)

func main() {
	var strList list.List
	strList.PushBack("ram")
	strList.PushBack("laxman")
	strList.PushBack("Bharat")
	strList.PushBack("Satrudhan")

	for i := strList.Front(); i != nil; i = i.Next() {
		fmt.Println(i.Value.(string))
	}
}
