package net

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

// TestHttpClient HTTP 客户端 GET 请求
func TestHttpClient(t *testing.T) {
	client := &http.Client{
		Timeout: 10 * time.Second,
	}

	resp, err := client.Get("https://httpbin.org/get")
	if err != nil {
		fmt.Printf("Get error: %v\n", err)
		return
	}
	defer resp.Body.Close()

	fmt.Printf("Status: %s\n", resp.Status)
	fmt.Printf("Content-Type: %s\n", resp.Header.Get("Content-Type"))

	body, _ := io.ReadAll(resp.Body)
	fmt.Printf("Body: %s\n", string(body))
}

// TestHttpPost HTTP 客户端 POST 请求（修复：使用真实 body）
func TestHttpPost(t *testing.T) {
	client := &http.Client{Timeout: 10 * time.Second}

	// 构造请求体
	payload := map[string]interface{}{
		"name": "张三",
		"age":  18,
	}
	jsonData, err := json.Marshal(payload)
	if err != nil {
		fmt.Printf("Marshal error: %v\n", err)
		return
	}

	resp, err := client.Post(
		"https://httpbin.org/post",
		"application/json",
		bytes.NewBuffer(jsonData), // 修复：使用真实 body
	)
	if err != nil {
		fmt.Printf("Post error: %v\n", err)
		return
	}
	defer resp.Body.Close()

	fmt.Printf("Status: %s\n", resp.Status)
	body, _ := io.ReadAll(resp.Body)
	fmt.Printf("Body: %s\n", string(body))
}

// TestHttpCustomRequest 自定义 Request（带 Header）
func TestHttpCustomRequest(t *testing.T) {
	client := &http.Client{Timeout: 10 * time.Second}

	payload := map[string]string{"key": "value"}
	jsonData, _ := json.Marshal(payload)

	req, err := http.NewRequest("POST", "https://httpbin.org/post", bytes.NewBuffer(jsonData))
	if err != nil {
		fmt.Printf("NewRequest error: %v\n", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer token-123")
	req.Header.Set("X-Request-ID", "req-001")

	resp, err := client.Do(req)
	if err != nil {
		fmt.Printf("Do error: %v\n", err)
		return
	}
	defer resp.Body.Close()

	fmt.Printf("Status: %s\n", resp.Status)
}

// TestHttpServer HTTP 服务端 + httptest（不阻塞测试）
func TestHttpServer(t *testing.T) {
	// 使用 httptest.NewServer 在随机端口启动测试服务器
	mux := http.NewServeMux()

	mux.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello, %s!", r.URL.Path[1:])
	})

	mux.HandleFunc("/api/user", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"name": "张三",
			"age":  18,
		})
	})

	server := httptest.NewServer(mux)
	defer server.Close()

	fmt.Println("Test server running at:", server.URL)

	// 测试 /api/user
	resp, err := http.Get(server.URL + "/api/user")
	if err != nil {
		t.Fatal(err)
	}
	defer resp.Body.Close()

	body, _ := io.ReadAll(resp.Body)
	fmt.Printf("Response: %s\n", string(body))

	// 测试路径参数
	resp2, err := http.Get(server.URL + "/World")
	if err != nil {
		t.Fatalf("GET /World failed: %v", err)
	}
	defer resp2.Body.Close()
	body2, _ := io.ReadAll(resp2.Body)
	fmt.Printf("Response: %s\n", string(body2))
}

// TestHttpMiddleware 中间件模式（Handler 链式组合）
func TestHttpMiddleware(t *testing.T) {
	// 日志中间件
	logging := func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			fmt.Printf("[LOG] %s %s\n", r.Method, r.URL.Path)
			next.ServeHTTP(w, r)
		})
	}

	// 认证中间件
	auth := func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			token := r.Header.Get("Authorization")
			if token == "" {
				http.Error(w, "Unauthorized", http.StatusUnauthorized)
				return
			}
			next.ServeHTTP(w, r)
		})
	}

	// 业务 Handler
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		fmt.Fprintln(w, "OK")
	})

	// 链式组合：logging -> auth -> handler
	chain := logging(auth(handler))
	server := httptest.NewServer(chain)
	defer server.Close()

	client := &http.Client{}

	// 无 token → 401
	resp, _ := client.Get(server.URL + "/test")
	fmt.Printf("no token: %s\n", resp.Status)

	// 有 token → 200
	req, _ := http.NewRequest("GET", server.URL+"/test", nil)
	req.Header.Set("Authorization", "Bearer valid-token")
	resp2, _ := client.Do(req)
	fmt.Printf("with token: %s\n", resp2.Status)
}

// TestHttpQueryParams 查询参数解析
func TestHttpQueryParams(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		query := r.URL.Query()
		name := query.Get("name")
		age := query.Get("age")
		fmt.Fprintf(w, "name=%s, age=%s", name, age)
	}))
	defer server.Close()

	resp, err := http.Get(server.URL + "/search?name=张三&age=18")
	if err != nil {
		t.Fatalf("GET /search failed: %v", err)
	}
	defer resp.Body.Close()
	body, _ := io.ReadAll(resp.Body)
	fmt.Println("Query params result:", string(body))
}
