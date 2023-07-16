package main

import (
	"fmt"
	"reflect"
)

type Person interface {
	SetAge(age int)
	SetName(name string)
}

type Student struct {
	age    int
	name   string
	school string
}

func (s *Student) SetAge(age int) {
	s.age = age
}

func (s *Student) SetName(name string) {
	s.name = name
}

func ConstructPerson(p Person) Person {

	pPtr := reflect.ValueOf(&p).Elem()

	fmt.Println(pPtr.Elem())
	fmt.Println(pPtr.Elem().Type())
	fmt.Println(pPtr.Elem().Type().Elem())
	pPtr.Set(reflect.New(pPtr.Elem().Type().Elem()))
	return p
}

func main() {
	// fmt.Println(reflect.TypeOf(3))
	// stu := Student{age: 1, name: "yu"}
	// fmt.Println(reflect.TypeOf(stu))

	var s *Student
	//fmt.Println(reflect.TypeOf(s).Elem())
	student := ConstructPerson(s).(*Student)
	fmt.Println(student)
}
