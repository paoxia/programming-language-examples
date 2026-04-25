package gorm

import (
	"fmt"
	"log"
	"os"
	"testing"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

// Student db model
type Student struct {
	gorm.Model
	Name  string
	Score float64
}

// TableName return tableName
func (Student) TableName() string {
	return "student"
}

var db *gorm.DB

// init 初始化数据库连接
// 运行前请先启动 MySQL：
//
//	docker run -itd --name mysql-test -p 3306:3306 -e MYSQL_ROOT_PASSWORD=123456 mysql:5.7
func initDB() {
	var err error
	// 从环境变量读取密码，fallback 到默认值
	pwd := os.Getenv("MYSQL_ROOT_PASSWORD")
	if pwd == "" {
		pwd = "123456"
	}
	dsn := fmt.Sprintf("root:%s@tcp(127.0.0.1:3306)/school?charset=utf8mb4&parseTime=True&loc=Local", pwd)

	db, err = gorm.Open(mysql.Open(dsn), &gorm.Config{
		Logger: logger.Default.LogMode(logger.Info),
	})
	if err != nil {
		log.Fatalf("connect db error: %v", err)
	}
	db.AutoMigrate(&Student{})
}

// ---- Create ----

// TestCreate 创建记录
func TestCreate(t *testing.T) {
	initDB()

	student := Student{Name: "Alice", Score: 95.5}
	result := db.Create(&student)
	if result.Error != nil {
		t.Fatal(result.Error)
	}
	fmt.Printf("created: id=%d, name=%s\n", student.ID, student.Name)
}

// TestBatchCreate 批量创建
func TestBatchCreate(t *testing.T) {
	initDB()

	students := []Student{
		{Name: "Bob", Score: 88.0},
		{Name: "Carol", Score: 92.5},
		{Name: "Dave", Score: 76.0},
	}
	result := db.Create(&students)
	if result.Error != nil {
		t.Fatal(result.Error)
	}
	fmt.Printf("batch created: %d rows\n", result.RowsAffected)
}

// ---- Read ----

// TestFind 查询多条记录
func TestFind(t *testing.T) {
	initDB()

	var students []Student
	db.Find(&students)
	fmt.Printf("total: %d\n", len(students))
	for _, s := range students {
		fmt.Printf("  id=%d, name=%s, score=%.1f\n", s.ID, s.Name, s.Score)
	}
}

// TestFirst 查询第一条记录
func TestFirst(t *testing.T) {
	initDB()

	var student Student
	result := db.First(&student)
	if result.Error != nil {
		t.Fatal(result.Error)
	}
	fmt.Printf("first: id=%d, name=%s\n", student.ID, student.Name)
}

// TestWhere 条件查询
func TestWhere(t *testing.T) {
	initDB()

	// 查询 score >= 90 的学生
	var students []Student
	db.Where("score >= ?", 90).Find(&students)
	fmt.Printf("score>=90: %d students\n", len(students))

	// 链式条件查询
	var student Student
	db.Where("name = ?", "Alice").First(&student)
	fmt.Printf("Alice: %+v\n", student)

	// Order + Limit
	var top3 []Student
	db.Order("score desc").Limit(3).Find(&top3)
	fmt.Println("top 3 by score:")
	for _, s := range top3 {
		fmt.Printf("  %s: %.1f\n", s.Name, s.Score)
	}
}

// TestSelect 指定查询字段
func TestSelect(t *testing.T) {
	initDB()

	var students []Student
	db.Select("id, name").Find(&students)
	for _, s := range students {
		fmt.Printf("id=%d, name=%s\n", s.ID, s.Name)
	}
}

// ---- Update ----

// TestUpdate 更新单个字段
func TestUpdate(t *testing.T) {
	initDB()

	// 先找到要更新的记录
	var student Student
	db.Where("name = ?", "Alice").First(&student)

	// 更新单个字段
	db.Model(&student).Update("Score", 98.5)
	fmt.Printf("after update: score=%.1f\n", student.Score)

	// 更新多个字段
	db.Model(&student).Updates(Student{Name: "Alice_V2", Score: 99.0})
	fmt.Printf("after updates: name=%s, score=%.1f\n", student.Name, student.Score)

	// 使用 map 更新（可以更新零值字段）
	db.Model(&student).Updates(map[string]interface{}{"score": 100.0})
}

// ---- Delete ----

// TestDelete 软删除（GORM Model 中有 DeletedAt 字段）
func TestDelete(t *testing.T) {
	initDB()

	var student Student
	db.Where("name = ?", "Dave").First(&student)
	if student.ID == 0 {
		fmt.Println("Dave not found, skip delete")
		return
	}

	// 软删除：设置 DeletedAt，记录仍在数据库中
	db.Delete(&student)
	fmt.Printf("soft deleted: id=%d\n", student.ID)

	// 验证：普通 Find 找不到被软删除的记录
	var check Student
	result := db.Where("name = ?", "Dave").First(&check)
	fmt.Printf("find after soft delete: found=%v\n", result.Error == nil)

	// 包含软删除记录的查询
	db.Unscoped().Where("name = ?", "Dave").First(&check)
	fmt.Printf("unscoped find: name=%s, deleted=%v\n", check.Name, check.DeletedAt.Valid)

	// 硬删除
	db.Unscoped().Delete(&student)
	fmt.Println("hard deleted")
}

// ---- Transaction ----

// TestTransaction 事务：原子性 CRUD
func TestTransaction(t *testing.T) {
	initDB()

	err := db.Transaction(func(tx *gorm.DB) error {
		// 在事务中创建两条记录
		s1 := Student{Name: "TxStudent1", Score: 85.0}
		if err := tx.Create(&s1).Error; err != nil {
			return err // 返回错误会触发回滚
		}

		s2 := Student{Name: "TxStudent2", Score: 90.0}
		if err := tx.Create(&s2).Error; err != nil {
			return err
		}

		// 模拟业务规则：如果分数超过 95 则不允许创建（演示回滚场景）
		// return fmt.Errorf("business rule violation, rollback")

		fmt.Printf("transaction: created s1.id=%d, s2.id=%d\n", s1.ID, s2.ID)
		return nil // 返回 nil 提交事务
	})

	if err != nil {
		fmt.Printf("transaction failed: %v\n", err)
	} else {
		fmt.Println("transaction committed")
	}
}

// ---- 原始 SQL ----

// TestRawSQL 执行原始 SQL
func TestRawSQL(t *testing.T) {
	initDB()

	// Raw 查询
	var students []Student
	db.Raw("SELECT * FROM student WHERE score > ? ORDER BY score DESC LIMIT ?", 80, 5).Scan(&students)
	fmt.Printf("raw query result: %d rows\n", len(students))

	// Exec 执行
	db.Exec("UPDATE student SET score = score + 1 WHERE name = ?", "Bob")
	fmt.Println("exec done")
}

// TestGormBenchmark 百万条数据写入压测（谨慎运行）
func TestGormBenchmark(t *testing.T) {
	t.Skip("skip benchmark by default, remove t.Skip() to run")
	initDB()

	count := 1000000
	for i := 0; i < count; i++ {
		name := fmt.Sprintf("test:%v", i)
		db.Create(&Student{Name: name, Score: float64(i % 100)})
	}
}
