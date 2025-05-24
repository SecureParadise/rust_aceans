package main

// go rutine is lightweight thread
import (
	"fmt"
	"time"
)

//	func task(id int) {
//		fmt.Println("doing task", id)
//	}
func main() {
	for i := 0; i <= 10; i++ {
		// go task(i)
		// anonymous function
		// go func(i) {
		// 	fmt.Println(i)
		// }()
		go func(i int) {
			fmt.Println(i)
		}(i)
	}

	time.Sleep(time.Second * 1)
}
