package basic

import (
	"fmt"
	"testing"
)

// TestVariable 变量声明
func TestVariable(t *testing.T) {
	// 方式1: var 声明
	var a int = 10
	var b string = "hello"
	var c bool = true
	fmt.Printf("a=%d, b=%s, c=%t\n", a, b, c)

	// 方式2: 类型推导
	var d = 20
	var e = "world"
	fmt.Printf("d=%d, e=%s\n", d, e)

	// 方式3: 短变量声明（只能在函数内）
	f := 30
	g := "golang"
	fmt.Printf("f=%d, g=%s\n", f, g)

	// 多变量声明
	var h, i, j = 1, 2, 3
	k, l, m := 4, 5, 6
	fmt.Printf("h=%d, i=%d, j=%d, k=%d, l=%d, m=%d\n", h, i, j, k, l, m)

	// 零值
	var n int
	var o string
	var p bool
	fmt.Printf("n=%d, o=%q, p=%t\n", n, o, p)
}

// TestType 基本类型
func TestType(t *testing.T) {
	// 整数
	var (
		i1 int   = 123
		i2 int8  = 127
		i3 int16 = 32767
		i4 int32 = 2147483647
		i5 int64 = 9223372036854775807
		u1 uint  = 123
	)
	fmt.Printf("i1=%d, i2=%d, i3=%d, i4=%d, i5=%d, u1=%d\n", i1, i2, i3, i4, i5, u1)

	// 浮点数
	var f1 float32 = 3.14
	var f2 float64 = 3.141592653589793
	fmt.Printf("f1=%f, f2=%f\n", f1, f2)

	// 复数
	var c1 complex64 = 1 + 2i
	var c2 complex128 = 3 + 4i
	fmt.Printf("c1=%v, c2=%v\n", c1, c2)

	// 字节和rune
	var b byte = 'a'
	var r rune = '中'
	fmt.Printf("b=%c, r=%c\n", b, r)

	// 字符串
	var s string = "hello 世界"
	fmt.Printf("s=%s, length=%d\n", s, len(s))
}

// TestConstant 常量
func TestConstant(t *testing.T) {
	const pi = 3.14159
	const (
		statusOk  = 200
		statusErr = 500
	)
	fmt.Printf("pi=%f, statusOk=%d, statusErr=%d\n", pi, statusOk, statusErr)

	// iota 枚举
	const (
		zero = iota // 0
		one         // 1
		two         // 2
		three       // 3
	)
	fmt.Printf("zero=%d, one=%d, two=%d, three=%d\n", zero, one, two, three)
}
