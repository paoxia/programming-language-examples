package interface_demo

import (
	"fmt"
	"math"
	"testing"
)

// ---- 接口定义与实现 ----

// Shape 接口定义
type Shape interface {
	Area() float64
	Perimeter() float64
}

// Circle 圆形
type Circle struct {
	Radius float64
}

func (c Circle) Area() float64 {
	return math.Pi * c.Radius * c.Radius
}

func (c Circle) Perimeter() float64 {
	return 2 * math.Pi * c.Radius
}

// Rectangle 矩形
type Rectangle struct {
	Width, Height float64
}

func (r Rectangle) Area() float64 {
	return r.Width * r.Height
}

func (r Rectangle) Perimeter() float64 {
	return 2 * (r.Width + r.Height)
}

// printShape 接受接口参数
func printShape(s Shape) {
	fmt.Printf("Area=%.2f, Perimeter=%.2f\n", s.Area(), s.Perimeter())
}

// TestInterfaceBasic 接口基础：定义、实现、多态
func TestInterfaceBasic(t *testing.T) {
	c := Circle{Radius: 5}
	r := Rectangle{Width: 4, Height: 6}

	printShape(c)
	printShape(r)

	// 接口切片：多态使用
	shapes := []Shape{c, r, Circle{Radius: 3}}
	for _, s := range shapes {
		fmt.Printf("%T -> Area=%.2f\n", s, s.Area())
	}
}

// ---- 接口组合 ----

// Stringer 打印接口
type Stringer interface {
	String() string
}

// Saver 持久化接口
type Saver interface {
	Save() error
}

// Describable 组合接口（接口嵌入）
type Describable interface {
	Shape    // 嵌入 Shape 接口
	Stringer // 嵌入 Stringer 接口
}

// Square 正方形，实现 Describable 要求的所有方法
type Square struct {
	Side float64
}

func (s Square) Area() float64      { return s.Side * s.Side }
func (s Square) Perimeter() float64 { return 4 * s.Side }
func (s Square) String() string     { return fmt.Sprintf("Square(side=%.2f)", s.Side) }

// TestInterfaceComposition 接口组合
func TestInterfaceComposition(t *testing.T) {
	sq := Square{Side: 5}

	// 可以赋值给组合接口
	var d Describable = sq
	fmt.Println(d.String())
	fmt.Printf("Area=%.2f\n", d.Area())

	// 也可赋值给子接口
	var s Shape = sq
	var str Stringer = sq
	fmt.Printf("Shape Area=%.2f, Stringer=%s\n", s.Area(), str.String())
}

// ---- 类型断言 ----

// TestTypeAssertion 类型断言
func TestTypeAssertion(t *testing.T) {
	var s Shape = Circle{Radius: 3}

	// 安全类型断言
	if c, ok := s.(Circle); ok {
		fmt.Printf("It's a Circle with radius=%.2f\n", c.Radius)
	}

	// type switch
	shapes := []Shape{Circle{5}, Rectangle{3, 4}, Square{6}}
	for _, shape := range shapes {
		switch v := shape.(type) {
		case Circle:
			fmt.Printf("Circle radius=%.2f\n", v.Radius)
		case Rectangle:
			fmt.Printf("Rectangle %gx%g\n", v.Width, v.Height)
		case Square:
			fmt.Printf("Square side=%.2f\n", v.Side)
		default:
			fmt.Printf("Unknown shape: %T\n", v)
		}
	}
}

// ---- 空接口 interface{} ----

// TestEmptyInterface 空接口（接收任意类型）
func TestEmptyInterface(t *testing.T) {
	var any interface{}

	any = 42
	fmt.Printf("int: %v (%T)\n", any, any)

	any = "hello"
	fmt.Printf("string: %v (%T)\n", any, any)

	any = []int{1, 2, 3}
	fmt.Printf("slice: %v (%T)\n", any, any)
}

// ---- nil interface 陷阱 ----

// TestNilInterface nil interface 与 nil 指针的区别
func TestNilInterface(t *testing.T) {
	var c *Circle       // nil 指针
	var s Shape = c     // interface 包含类型信息，不等于 nil！

	fmt.Printf("c == nil: %v\n", c == nil)   // true
	fmt.Printf("s == nil: %v\n", s == nil)   // false（接口非 nil，但内部值为 nil）

	var s2 Shape // 未赋值的接口，真正的 nil interface
	fmt.Printf("s2 == nil: %v\n", s2 == nil) // true
}
