package basic

import (
	"errors"
	"fmt"
	"testing"
)

// TestError 错误处理
func TestError(t *testing.T) {
	// 创建错误
	err1 := errors.New("something went wrong")
	fmt.Println(err1)

	err2 := fmt.Errorf("error code: %d", 404)
	fmt.Println(err2)

	// 自定义错误类型
	err3 := &MyError{
		Code: 500,
		Msg:  "internal server error",
	}
	fmt.Println(err3)

	// 使用错误
	result, err := Divide(10, 2)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Result: %d\n", result)
	}

	result, err = Divide(10, 0)
	if err != nil {
		fmt.Printf("Error: %v\n", err)
	} else {
		fmt.Printf("Result: %d\n", result)
	}
}

// MyError 自定义错误
type MyError struct {
	Code int
	Msg  string
}

func (e *MyError) Error() string {
	return fmt.Sprintf("code=%d, msg=%s", e.Code, e.Msg)
}

// Divide 除法函数
func Divide(a, b int) (int, error) {
	if b == 0 {
		return 0, errors.New("division by zero")
	}
	return a / b, nil
}

// TestPanic panic
func TestPanic(t *testing.T) {
	// panic
	// panic("oops")

	// recover
	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("Recovered: %v\n", r)
		}
	}()

	DoSomething(true)
	fmt.Println("Continuing...")
}

func DoSomething(shouldPanic bool) {
	if shouldPanic {
		panic("something bad happened")
	}
	fmt.Println("Doing something...")
}
