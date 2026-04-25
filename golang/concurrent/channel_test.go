package concurrent

import (
	"fmt"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

// ---- 无缓冲 channel ----

// TestUnbufferedChannel 无缓冲 channel：发送和接收必须同步
func TestUnbufferedChannel(t *testing.T) {
	ch := make(chan int) // 无缓冲

	go func() {
		fmt.Println("sender: sending 42")
		ch <- 42 // 阻塞，直到有接收方
		fmt.Println("sender: sent")
	}()

	time.Sleep(100 * time.Millisecond)
	val := <-ch // 接收
	fmt.Println("receiver: got", val)
}

// TestBufferedChannel 有缓冲 channel（已有基础示例，此处展示更多用法）
func TestBufferedChannel(t *testing.T) {
	ch := make(chan int, 2)
	ch <- 2
	ch <- 1

	fmt.Println(<-ch) // 2（FIFO）
	fmt.Println(<-ch) // 1
}

// ---- close + range channel ----

// TestCloseRange 关闭 channel 后用 range 消费所有数据
func TestCloseRange(t *testing.T) {
	ch := make(chan int, 5)

	// 生产者：发完关闭
	go func() {
		for i := 1; i <= 5; i++ {
			ch <- i
		}
		close(ch) // 关闭后 range 会退出
	}()

	// 消费者：range 自动检测 close
	for v := range ch {
		fmt.Println("received:", v)
	}
	fmt.Println("channel closed, done")
}

// TestCheckClosed 通过 ok 判断 channel 是否已关闭
func TestCheckClosed(t *testing.T) {
	ch := make(chan int, 3)
	ch <- 10
	ch <- 20
	close(ch)

	for {
		val, ok := <-ch
		if !ok {
			fmt.Println("channel closed")
			break
		}
		fmt.Println("val:", val)
	}
}

// ---- 单向 channel ----

// producer 只写 channel（单向）
func producer(out chan<- int) {
	for i := 0; i < 3; i++ {
		out <- i
		time.Sleep(50 * time.Millisecond)
	}
	close(out)
}

// consumer 只读 channel（单向）
func consumer(in <-chan int) {
	for v := range in {
		fmt.Println("consume:", v)
	}
}

// TestDirectionalChannel 单向 channel
func TestDirectionalChannel(t *testing.T) {
	ch := make(chan int, 5)
	go producer(ch) // 双向 channel 可隐式转换为单向
	consumer(ch)
}

// ---- select 多路复用 ----

// TestSelect select：监听多个 channel
func TestSelect(t *testing.T) {
	ch1 := make(chan string, 1)
	ch2 := make(chan string, 1)

	go func() {
		time.Sleep(200 * time.Millisecond)
		ch1 <- "one"
	}()
	go func() {
		time.Sleep(100 * time.Millisecond)
		ch2 <- "two"
	}()

	// ch2 先就绪
	for i := 0; i < 2; i++ {
		select {
		case msg := <-ch1:
			fmt.Println("received from ch1:", msg)
		case msg := <-ch2:
			fmt.Println("received from ch2:", msg)
		}
	}
}

// TestSelectDefault select default 非阻塞
func TestSelectDefault(t *testing.T) {
	ch := make(chan int, 1)

	select {
	case v := <-ch:
		fmt.Println("received:", v)
	default:
		fmt.Println("no value ready (non-blocking)")
	}

	ch <- 99
	select {
	case v := <-ch:
		fmt.Println("received:", v)
	default:
		fmt.Println("no value ready")
	}
}

// TestSelectTimeout select + timeout 模式
func TestSelectTimeout(t *testing.T) {
	ch := make(chan int)

	go func() {
		time.Sleep(600 * time.Millisecond)
		ch <- 1
	}()

	select {
	case v := <-ch:
		fmt.Println("received:", v)
	case <-time.After(300 * time.Millisecond):
		fmt.Println("timeout! no response within 300ms")
	}
}

// ---- RWMutex 读写锁 ----

// TestRWMutex 读写锁：多读单写
func TestRWMutex(t *testing.T) {
	var rwMu sync.RWMutex
	var wg sync.WaitGroup
	cache := map[string]string{"key": "value0"}

	// 5 个并发读
	for i := 0; i < 5; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			rwMu.RLock()
			defer rwMu.RUnlock()
			fmt.Printf("reader %d: %s\n", id, cache["key"])
		}(i)
	}

	// 1 个写（会等待所有读完成）
	wg.Add(1)
	go func() {
		defer wg.Done()
		rwMu.Lock()
		defer rwMu.Unlock()
		cache["key"] = "value_updated"
		fmt.Println("writer: updated")
	}()

	wg.Wait()
}

// ---- atomic 原子操作 ----

// TestAtomic 原子操作：无锁并发计数
func TestAtomic(t *testing.T) {
	var counter int64
	var wg sync.WaitGroup

	for i := 0; i < 1000; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			atomic.AddInt64(&counter, 1)
		}()
	}

	wg.Wait()
	fmt.Println("counter:", atomic.LoadInt64(&counter)) // 应为 1000
}

// TestAtomicCompareSwap CAS（Compare And Swap）原子操作
func TestAtomicCompareSwap(t *testing.T) {
	var val int32 = 10

	// CAS：如果当前值是 10，则替换为 20
	swapped := atomic.CompareAndSwapInt32(&val, 10, 20)
	fmt.Printf("CAS(10->20) swapped=%v, val=%d\n", swapped, val) // true, 20

	// 再次 CAS：当前值是 20，尝试用 10 替换（失败）
	swapped = atomic.CompareAndSwapInt32(&val, 10, 30)
	fmt.Printf("CAS(10->30) swapped=%v, val=%d\n", swapped, val) // false, 20
}
