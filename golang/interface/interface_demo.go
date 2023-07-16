package main

import "fmt"


func test(data interface{}) {
	switch data.(type) {
	case int:
		fmt.Println(data.(int))
	case float64:
		fmt.Println(data.(float64))
	case *int:
		fmt.Println(*data.(*int))
	}

}


func main() {
	test(123)
	test(1234.1)
	i := 234
	test(&i)
}