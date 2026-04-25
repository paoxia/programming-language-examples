package func_demo

import (
	"fmt"
	"testing"
)

// ---- 多返回值 ----

// divide 多返回值函数
func divide(a, b float64) (float64, error) {
	if b == 0 {
		return 0, fmt.Errorf("division by zero")
	}
	return a / b, nil
}

// TestMultiReturn 多返回值
func TestMultiReturn(t *testing.T) {
	result, err := divide(10, 3)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("10 / 3 = %.4f\n", result)
	}

	_, err = divide(5, 0)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	}
}

// ---- 命名返回值 ----

// minMax 命名返回值
func minMax(nums []int) (min, max int) {
	min, max = nums[0], nums[0]
	for _, v := range nums[1:] {
		if v < min {
			min = v
		}
		if v > max {
			max = v
		}
	}
	return // 裸 return，自动返回命名变量
}

// TestNamedReturn 命名返回值
func TestNamedReturn(t *testing.T) {
	nums := []int{3, 1, 4, 1, 5, 9, 2, 6}
	min, max := minMax(nums)
	fmt.Printf("min=%d, max=%d\n", min, max)
}

// ---- 可变参数 ----

// sum 可变参数求和
func sum(nums ...int) int {
	total := 0
	for _, v := range nums {
		total += v
	}
	return total
}

// TestVariadicArgs 可变参数函数
func TestVariadicArgs(t *testing.T) {
	fmt.Println(sum(1, 2, 3))          // 直接传入
	fmt.Println(sum(1, 2, 3, 4, 5))    // 任意个数
	nums := []int{10, 20, 30}
	fmt.Println(sum(nums...))           // 切片展开
}

// ---- 函数作为一等公民 / 函数类型 ----

// MathFunc 函数类型定义
type MathFunc func(int, int) int

// apply 高阶函数：接收函数作为参数
func apply(a, b int, fn MathFunc) int {
	return fn(a, b)
}

// TestHigherOrder 高阶函数
func TestHigherOrder(t *testing.T) {
	add := func(a, b int) int { return a + b }
	mul := func(a, b int) int { return a * b }

	fmt.Printf("add(3,4)=%d\n", apply(3, 4, add))
	fmt.Printf("mul(3,4)=%d\n", apply(3, 4, mul))

	// 函数作为返回值
	makeAdder := func(x int) func(int) int {
		return func(y int) int {
			return x + y
		}
	}
	add5 := makeAdder(5)
	fmt.Printf("add5(10)=%d\n", add5(10))
	fmt.Printf("add5(20)=%d\n", add5(20))
}

// ---- 闭包 ----

// counter 闭包计数器：每次调用返回递增值
func counter() func() int {
	count := 0
	return func() int {
		count++
		return count
	}
}

// TestClosure 闭包
func TestClosure(t *testing.T) {
	c1 := counter()
	c2 := counter() // 独立的计数器

	fmt.Println(c1(), c1(), c1()) // 1 2 3
	fmt.Println(c2(), c2())       // 1 2（独立）
	fmt.Println(c1())             // 4（c1 继续）
}

// ---- defer ----

// TestDefer defer 执行顺序（LIFO 后进先出）
func TestDefer(t *testing.T) {
	fmt.Println("start")

	defer fmt.Println("defer 1") // 最后执行
	defer fmt.Println("defer 2") // 倒数第二
	defer fmt.Println("defer 3") // 最先执行（倒序）

	fmt.Println("end")
	// 输出顺序：start → end → defer 3 → defer 2 → defer 1
}

// TestDeferLoop defer 在循环中（注意：每次循环都会注册 defer，但函数返回时才执行）
func TestDeferLoop(t *testing.T) {
	for i := 0; i < 3; i++ {
		defer fmt.Printf("loop defer i=%d\n", i) // 参数立即求值
	}
	fmt.Println("after loop")
}

// TestDeferModifyReturn defer 修改命名返回值
func double(x int) (result int) {
	defer func() {
		result *= 2 // 修改命名返回值
	}()
	result = x
	return
}

func TestDeferModifyReturn(t *testing.T) {
	fmt.Printf("double(5)=%d\n", double(5)) // 10
}

// ---- 匿名函数 ----

// TestAnonymousFunc 匿名函数
func TestAnonymousFunc(t *testing.T) {
	// 立即调用
	result := func(a, b int) int {
		return a + b
	}(3, 4)
	fmt.Printf("anonymous func result: %d\n", result)

	// 赋值给变量
	greet := func(name string) string {
		return "Hello, " + name + "!"
	}
	fmt.Println(greet("Go"))
}
