package context_demo

import (
	"context" // alias
	"fmt"
	"testing"
)

func TestContext(t *testing.T) {
	ctx := context.Background()
	const key = "key1"
	value := "value1"
	ctx = context.WithValue(ctx, key, value)
	fmt.Println(ctx.Value(key))
}
