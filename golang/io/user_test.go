package io

import (
	"fmt"
	"testing"
)

// seem must use func main
func TestUser(t *testing.T) {
	var num int
	fmt.Scanln(&num)
	fmt.Println(num)
}
