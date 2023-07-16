package main

import "fmt"

func test() {
	defer func() {
        if err := recover(); err != nil {
            fmt.Println("work failed:", err)
        }
    }()
	panic("test")
}

func main() {
	count := 2
	for i := 0; i < count; i++ {
		fmt.Printf("before:%v",i)
		test()
		fmt.Printf("after:%v",i)
	}

}
