package main

// go rutine is lightweight thread
import (
	"fmt"
	"sync"
)

func task(id int, w *sync.WaitGroup) {
	defer w.Done()
	fmt.Println("doing task", id)
}
func main() {
	var waitgroup_wg sync.WaitGroup

	for i := 0; i <= 10; i++ {
		waitgroup_wg.Add(1)
		go task(i, &waitgroup_wg)

	}

	waitgroup_wg.Wait()
}
