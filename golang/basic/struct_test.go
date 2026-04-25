package basic

import (
	"fmt"
	"testing"
)

// Person 结构体
type Person struct {
	Name string
	Age  int
}

// Student 嵌套结构体
type Student struct {
	Person
	School string
	Score  float64
}

// TestStruct 结构体
func TestStruct(t *testing.T) {
	// 方式1: 字段名:值
	p1 := Person{
		Name: "张三",
		Age:  18,
	}
	fmt.Printf("p1=%v\n", p1)

	// 方式2: 按顺序赋值
	p2 := Person{"李四", 20}
	fmt.Printf("p2=%v\n", p2)

	// 方式3: new
	p3 := new(Person)
	p3.Name = "王五"
	p3.Age = 22
	fmt.Printf("p3=%v\n", p3)

	// 访问字段
	fmt.Printf("p1.Name=%s, p1.Age=%d\n", p1.Name, p1.Age)
}

// TestNestedStruct 嵌套结构体
func TestNestedStruct(t *testing.T) {
	// 嵌套结构体初始化
	s := Student{
		Person: Person{
			Name: "小明",
			Age:  16,
		},
		School: "第一中学",
		Score:  95.5,
	}
	fmt.Printf("s=%v\n", s)

	// 提升字段访问（直接访问嵌入结构体的字段）
	fmt.Printf("Name=%s, Age=%d, School=%s\n", s.Name, s.Age, s.School)
}

// TestStructMethod 结构体方法
func TestStructMethod(t *testing.T) {
	p := Person{"张三", 18}

	// 值接收者
	p.SayHello()
	fmt.Printf("After SayHello: p.Age=%d\n", p.Age)

	// 指针接收者
	p.HappyBirthday()
	fmt.Printf("After HappyBirthday: p.Age=%d\n", p.Age)
}

// SayHello 值接收者方法
func (p Person) SayHello() {
	fmt.Printf("Hello, I'm %s, %d years old\n", p.Name, p.Age)
	p.Age++ // 不会改变原对象
}

// HappyBirthday 指针接收者方法
func (p *Person) HappyBirthday() {
	p.Age++
	fmt.Printf("Happy birthday %s! Now you're %d years old\n", p.Name, p.Age)
}
