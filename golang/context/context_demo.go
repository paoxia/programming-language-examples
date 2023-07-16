package main

import (
	"context"
	"fmt"
)

func main() {
	ctx := context.Background()
	const key = "key1"
	value := "value1"
	ctx = context.WithValue(ctx, key, value)
	fmt.Println(ctx.Value(key))
}
