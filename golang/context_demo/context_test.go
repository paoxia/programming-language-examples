package context_demo

import (
	"context"
	"fmt"
	"testing"
	"time"
)

// TestContextValue WithValue：在 context 中传递键值对
func TestContextValue(t *testing.T) {
	ctx := context.Background()

	// 推荐使用自定义类型作为 key，避免碰撞
	type contextKey string
	const userKey contextKey = "user"
	const traceKey contextKey = "traceID"

	ctx = context.WithValue(ctx, userKey, "Alice")
	ctx = context.WithValue(ctx, traceKey, "abc-123")

	fmt.Println("user:", ctx.Value(userKey))
	fmt.Println("traceID:", ctx.Value(traceKey))
}

// TestContextCancel WithCancel：主动取消
func TestContextCancel(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel() // 确保资源释放

	// 启动子 goroutine，监听取消信号
	done := make(chan struct{})
	go func() {
		defer close(done)
		for {
			select {
			case <-ctx.Done():
				fmt.Println("goroutine: context cancelled:", ctx.Err())
				return
			default:
				fmt.Println("goroutine: working...")
				time.Sleep(100 * time.Millisecond)
			}
		}
	}()

	// 主线程等 300ms 后取消
	time.Sleep(300 * time.Millisecond)
	cancel()
	<-done // 等待 goroutine 退出
	fmt.Println("main: done")
}

// TestContextTimeout WithTimeout：超时自动取消
func TestContextTimeout(t *testing.T) {
	// 设置 500ms 超时
	ctx, cancel := context.WithTimeout(context.Background(), 500*time.Millisecond)
	defer cancel()

	done := make(chan struct{})
	go func() {
		defer close(done)
		select {
		case <-time.After(2 * time.Second): // 模拟耗时操作（2s > 超时500ms）
			fmt.Println("goroutine: task completed")
		case <-ctx.Done():
			fmt.Println("goroutine: timeout/cancelled:", ctx.Err())
		}
	}()

	<-done
	fmt.Println("main: context deadline:", ctx.Err())
}

// TestContextDeadline WithDeadline：指定截止时间
func TestContextDeadline(t *testing.T) {
	deadline := time.Now().Add(400 * time.Millisecond)
	ctx, cancel := context.WithDeadline(context.Background(), deadline)
	defer cancel()

	fmt.Println("deadline:", deadline.Format("15:04:05.000"))

	select {
	case <-time.After(1 * time.Second):
		fmt.Println("task done")
	case <-ctx.Done():
		fmt.Println("deadline exceeded:", ctx.Err())
	}
}

// TestContextChain context 链式传播：父 context 取消会传播到子 context
func TestContextChain(t *testing.T) {
	parent, parentCancel := context.WithCancel(context.Background())

	child, childCancel := context.WithCancel(parent)
	defer childCancel()

	grandchild, grandchildCancel := context.WithTimeout(child, 5*time.Second)
	defer grandchildCancel()

	// 取消父 context，子孙均会收到取消信号
	parentCancel()

	select {
	case <-grandchild.Done():
		fmt.Println("grandchild done:", grandchild.Err()) // context canceled
	case <-time.After(1 * time.Second):
		fmt.Println("timeout waiting")
	}
}
