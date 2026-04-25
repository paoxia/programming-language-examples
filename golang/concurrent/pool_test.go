package concurrent

import (
	"fmt"
	"sync"
	"testing"
	"time"
)

// TestWorkerPool 协程池
func TestWorkerPool(t *testing.T) {
	jobs := make(chan int, 100)
	results := make(chan int, 100)

	// 启动 3 个 worker
	for w := 1; w <= 3; w++ {
		go Worker(w, jobs, results)
	}

	// 发送 5 个任务
	for j := 1; j <= 5; j++ {
		jobs <- j
	}
	close(jobs)

	// 收集结果
	for a := 1; a <= 5; a++ {
		fmt.Printf("Result: %d\n", <-results)
	}
}

// Worker 工作协程
func Worker(id int, jobs <-chan int, results chan<- int) {
	for j := range jobs {
		fmt.Printf("Worker %d started job %d\n", id, j)
		time.Sleep(time.Second)
		fmt.Printf("Worker %d finished job %d\n", id, j)
		results <- j * 2
	}
}

// TestSemaphore 信号量
func TestSemaphore(t *testing.T) {
	sem := make(chan struct{}, 2) // 最多2个并发
	var wg sync.WaitGroup

	for i := 1; i <= 5; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			sem <- struct{}{} // 获取信号量
			defer func() { <-sem }()

			fmt.Printf("Goroutine %d is running\n", id)
			time.Sleep(2 * time.Second)
			fmt.Printf("Goroutine %d is done\n", id)
		}(i)
	}

	wg.Wait()
}

// TestOnce sync.Once
func TestOnce(t *testing.T) {
	var once sync.Once
	var wg sync.WaitGroup

	for i := 1; i <= 5; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			once.Do(Initialize)
		}()
	}

	wg.Wait()
}

// Initialize 初始化函数，只执行一次
func Initialize() {
	fmt.Println("Initialized!")
}

// TestMap sync.Map
func TestSyncMap(t *testing.T) {
	var m sync.Map

	// 写入
	m.Store("key1", "value1")
	m.Store("key2", "value2")

	// 读取
	if v, ok := m.Load("key1"); ok {
		fmt.Printf("key1=%s\n", v)
	}

	// 遍历
	m.Range(func(key, value interface{}) bool {
		fmt.Printf("key=%v, value=%v\n", key, value)
		return true
	})

	// 删除
	m.Delete("key1")
	if _, ok := m.Load("key1"); !ok {
		fmt.Println("key1 not found")
	}
}
