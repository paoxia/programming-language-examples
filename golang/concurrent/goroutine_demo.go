package main

import (
	"fmt"
)

func test() {
	fmt.Println("test")
}

func main() {

	go test()
	
}