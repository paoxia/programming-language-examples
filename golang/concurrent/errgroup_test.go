package concurrent

import (
	"errors"
	"fmt"
	"testing"

	"golang.org/x/sync/errgroup"
)

func TestErrGroup(t *testing.T) {
	var eg errgroup.Group

	eg.Go(func() error {
		return errors.New("error")
	})

	eg.Go(func() error {
		return nil
	})

	if err:= eg.Wait(); err != nil {
		fmt.Println(err.Error())
	}
	
}