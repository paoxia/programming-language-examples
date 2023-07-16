package main

import "fmt"

func input() {

}

func main() {
	ch := make(chan int, 2)
	ch <- 2
	ch <- 1

	fmt.Println(<-ch)
	fmt.Println(<-ch)
}
