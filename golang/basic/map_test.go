package basic

import (
	"fmt"
	"testing"
)

// TestMap 普通 map 基础操作
func TestMap(t *testing.T) {
	// 声明 map（nil map，不可写入）
	var m1 map[string]int
	fmt.Printf("m1=%v, m1==nil: %v\n", m1, m1 == nil)

	// 使用 make 创建
	m2 := make(map[string]int)
	m2["apple"] = 1
	m2["banana"] = 2
	m2["cherry"] = 3
	fmt.Printf("m2=%v\n", m2)

	// 字面量初始化
	m3 := map[string]int{
		"one":   1,
		"two":   2,
		"three": 3,
	}
	fmt.Printf("m3=%v\n", m3)

	// 读取：存在的 key
	val := m3["one"]
	fmt.Printf("m3[\"one\"]=%d\n", val)

	// 读取：不存在的 key 返回零值，通过 ok 判断是否存在
	val2, ok := m3["four"]
	fmt.Printf("m3[\"four\"]=%d, ok=%v\n", val2, ok)

	// 更新
	m3["one"] = 100
	fmt.Printf("after update: m3[\"one\"]=%d\n", m3["one"])

	// 删除
	delete(m3, "two")
	fmt.Printf("after delete: m3=%v\n", m3)

	// 遍历（顺序不固定）
	for k, v := range m3 {
		fmt.Printf("key=%s, value=%d\n", k, v)
	}

	// 获取长度
	fmt.Printf("len(m3)=%d\n", len(m3))
}

// TestMapNested 嵌套 map
func TestMapNested(t *testing.T) {
	nested := map[string]map[string]int{
		"class1": {"alice": 90, "bob": 85},
		"class2": {"carol": 92, "dave": 78},
	}
	fmt.Printf("class1.alice=%d\n", nested["class1"]["alice"])
	fmt.Printf("class2.carol=%d\n", nested["class2"]["carol"])
}

// TestSyncMap sync.Map 并发安全 map
func TestSyncMap(t *testing.T) {
	// sync.Map 示例已迁移至 concurrent/pool_test.go 中的 TestSyncMap
	// 此处演示普通 map 的并发注意事项（并发读写会 panic，需用 sync.Map 或加锁）
	fmt.Println("普通 map 并发读写不安全，请使用 sync.Map（见 concurrent/pool_test.go）或加 Mutex")
}
