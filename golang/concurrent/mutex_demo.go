package main

import (
	"fmt"
	"sync"
)

var mu sync.Mutex
var wg sync.WaitGroup
var m int = 0

func f() {
	mu.Lock()
	m++
	fmt.Println(m)
	mu.Unlock()
	wg.Done()
}

func main() {
	for i := 0; i <= 9; i++ {
		wg.Add(1)
		go f()
	}
	wg.Wait()
}
