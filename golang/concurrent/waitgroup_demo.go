package main

import (
	"fmt"
	"sync"
)

func print(i int) {
	fmt.Println(i)
}

func main() {
	var wg sync.WaitGroup
	for i := 0; i < 5;i++ {
		wg.Add(1)
		go func (num int)  {
			print(num)
			wg.Done()
		}(i)
	}
	// bad case
	// for i := 0; i < 5; i++ {
	// 	wg.Add(1)
	// 	go func() {
	// 		print(i)
	// 		wg.Done()
	// 	}()
	// }
	wg.Wait()

}
