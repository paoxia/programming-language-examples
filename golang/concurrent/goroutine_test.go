package concurrent

import (
	"fmt"
	"testing"
)

func test() {
	fmt.Println("test")
}

func TestGoroutine(t *testing.T) {

	go test()

}
