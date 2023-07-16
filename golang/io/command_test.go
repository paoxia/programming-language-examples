package io

import (
	"fmt"
	"os"
	"strings"
	"testing"
)

// go test -run TestCommand -v -args "test"
// usually use in func main
func TestCommand(t *testing.T) {
	if len(os.Args) > 1 {
		fmt.Println(strings.Join(os.Args[1:], ","))
	}
	fmt.Println(strings.Join(os.Args[5:], ","))
	fmt.Println(os.Args)
}
