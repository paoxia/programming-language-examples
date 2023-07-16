package concurrent

import (
	"fmt"
	"sync"
	"testing"
)

func print(i int) {
	fmt.Println(i)
}

func TestWaitGroup(t *testing.T) {
	var wg sync.WaitGroup
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(num int) {
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
