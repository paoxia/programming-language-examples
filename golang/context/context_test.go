package context

import (
	go_context "context" // alias
	"fmt"
	"testing"
)

func TestContext(t *testing.T) {
	ctx := go_context.Background()
	const key = "key1"
	value := "value1"
	ctx = go_context.WithValue(ctx, key, value)
	fmt.Println(ctx.Value(key))
}
