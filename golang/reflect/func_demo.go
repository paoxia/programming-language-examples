package main

import (
	"fmt"
	"reflect"
)

func add(x, y int) int {
	return x + y
}


func main() {
	f := reflect.ValueOf(add)
	params := []reflect.Value{reflect.ValueOf(1), reflect.ValueOf(2)}
	res := f.Call(params)
    fmt.Println(res[0].Int())
}