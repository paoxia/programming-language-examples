package encoding

import (
	"encoding/json"
	"fmt"
	"testing"
)

// User 用户结构体
type User struct {
	Name     string `json:"name"`
	Age      int    `json:"age"`
	Email    string `json:"email,omitempty"`
	Password string `json:"-"` // 忽略该字段
}

// TestJsonMarshal JSON序列化
func TestJsonMarshal(t *testing.T) {
	user := User{
		Name:     "张三",
		Age:      18,
		Email:    "zhangsan@example.com",
		Password: "123456",
	}

	// 序列化为JSON
	data, err := json.Marshal(user)
	if err != nil {
		fmt.Printf("Marshal error: %v\n", err)
		return
	}
	fmt.Printf("JSON: %s\n", data)

	// 格式化输出
	prettyData, err := json.MarshalIndent(user, "", "  ")
	if err != nil {
		fmt.Printf("MarshalIndent error: %v\n", err)
		return
	}
	fmt.Printf("Pretty JSON:\n%s\n", prettyData)
}

// TestJsonUnmarshal JSON反序列化
func TestJsonUnmarshal(t *testing.T) {
	jsonStr := `{"name":"李四","age":20,"email":"lisi@example.com"}`

	var user User
	err := json.Unmarshal([]byte(jsonStr), &user)
	if err != nil {
		fmt.Printf("Unmarshal error: %v\n", err)
		return
	}
	fmt.Printf("User: %+v\n", user)
}

// TestJsonMap JSON和map
func TestJsonMap(t *testing.T) {
	// map -> JSON
	m := map[string]interface{}{
		"name":    "王五",
		"age":     22,
		"enabled": true,
		"tags":    []string{"a", "b", "c"},
	}
	data, _ := json.MarshalIndent(m, "", "  ")
	fmt.Printf("Map JSON:\n%s\n", data)

	// JSON -> map
	var m2 map[string]interface{}
	json.Unmarshal(data, &m2)
	fmt.Printf("Unmarshaled map: %v\n", m2)
}
