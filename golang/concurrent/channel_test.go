package concurrent

import (
	"fmt"
	"testing"
)

func input() {

}

func TestChannel(t *testing.T) {
	ch := make(chan int, 2)
	ch <- 2
	ch <- 1

	fmt.Println(<-ch)
	fmt.Println(<-ch)
}
