package main

import (
	"fmt"
	"sync"
	"time"
)

func Consumer(consumerName string, c *sync.Cond) {
	c.L.Lock()
	c.Wait()
	fmt.Println(consumerName)
	c.L.Unlock()
}

func Producer(producerName string, c *sync.Cond) {
	time.Sleep(time.Second * 2)
	c.L.Lock()
	c.L.Unlock()

	fmt.Println(producerName)
	c.Broadcast() //c.Signal()
}

func main() {
	cond := sync.NewCond(&sync.Mutex{})
	go Consumer("consumer1", cond)
	go Consumer("consumer2", cond)
	go Consumer("consumer3", cond)

	Producer("producer", cond)
	time.Sleep(time.Second * 2)
}
