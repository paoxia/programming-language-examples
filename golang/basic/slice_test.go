package basic

import (
	"fmt"
	"testing"
)

// TestSlice 切片操作
func TestSlice(t *testing.T) {
	// 声明切片
	var s1 []int
	fmt.Printf("s1=%v, len=%d, cap=%d\n", s1, len(s1), cap(s1))

	// 使用 make 创建
	s2 := make([]int, 3, 5)
	fmt.Printf("s2=%v, len=%d, cap=%d\n", s2, len(s2), cap(s2))

	// 字面量初始化
	s3 := []int{1, 2, 3, 4, 5}
	fmt.Printf("s3=%v\n", s3)

	// 切片操作
	s4 := s3[1:3]   // [2, 3]
	s5 := s3[:3]    // [1, 2, 3]
	s6 := s3[2:]    // [3, 4, 5]
	s7 := s3[:]     // [1, 2, 3, 4, 5]
	fmt.Printf("s4=%v, s5=%v, s6=%v, s7=%v\n", s4, s5, s6, s7)

	// append 追加
	s8 := append(s3, 6)
	s9 := append(s3, 7, 8, 9)
	s10 := append(s3, s4...)
	fmt.Printf("s8=%v, s9=%v, s10=%v\n", s8, s9, s10)

	// copy 复制
	dst := make([]int, 3)
	count := copy(dst, s3)
	fmt.Printf("dst=%v, count=%d\n", dst, count)

	// 遍历
	for i, v := range s3 {
		fmt.Printf("index=%d, value=%d\n", i, v)
	}
}

// TestSliceExpand 切片扩容
func TestSliceExpand(t *testing.T) {
	s := make([]int, 0, 3)
	fmt.Printf("len=%d, cap=%d, ptr=%p\n", len(s), cap(s), s)

	for i := 0; i < 10; i++ {
		s = append(s, i)
		fmt.Printf("i=%d, len=%d, cap=%d, ptr=%p\n", i, len(s), cap(s), s)
	}
}
