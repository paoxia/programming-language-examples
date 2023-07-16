package gorm

import (
	"fmt"
	"log"
	"testing"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
)

// Student db model
type Student struct {
	gorm.Model
	ID   int64
	Name string
}

// TableName return tableName
func (Student) TableName() string {
	return "student"
}

var db *gorm.DB

func Init() {
	var err error
	dsn := "root:123456@tcp(127.0.0.1:3306)/school?charset=utf8mb4&parseTime=True&loc=Local"
	db, err = gorm.Open(mysql.Open(dsn), &gorm.Config{})
	if err != nil {
		log.Println("connect db error")
	}
	if db == nil {
		fmt.Println("nil 1")
	}
	db.AutoMigrate(&Student{})
}

// CreateStudent 创建
func CreateStudents() error {
	test := Student{Name: "test"}
	if db == nil {
		fmt.Println("nil")
	}
	return db.Create(&test).Error
}

func CreateStudent(i int) error {
	name := fmt.Sprintf("test:%v", i)
	test := Student{Name: name}
	if db == nil {
		fmt.Println("nil")
	}
	return db.Create(&test).Error
}

func TestGorm(t *testing.T) {
	// Create
	Init()
	//CreateStudent()
	count := 1000000
	for i := 0; i < count; i++ {
		CreateStudent(i)
	}
}
