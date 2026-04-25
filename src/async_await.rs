//! 异步编程教程

/// async fn
pub fn async_basics() {
    println!("--- async/await 基础 ---");

    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    // async 函数返回一个 Future
    async fn hello() {
        println!("Hello, async!");
    }

    // 实现 Future trait
    struct HelloFuture;

    impl Future for HelloFuture {
        type Output = ();

        fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            println!("Hello from poll!");
            Poll::Ready(())
        }
    }

    // 我们需要一个执行器来运行 Future
    // 这里用 tokio 来演示
    println!("async basics explained");
}

/// 使用 Tokio
pub fn tokio_example() {
    println!("--- Tokio ---");

    // 注意：这需要 main 函数也是 async 的

    println!("Tokio example: see the main function for usage");
}

/// async/.await
pub fn await_example() {
    println!("--- .await ---");

    // async 块
    // let future = async {
    //     println!("in async block");
    // };

    println!(".await pauses the function until the future is complete");
    println!("async functions can be called inside other async functions");
}

/// 并发运行多个 Future
pub fn concurrent_futures() {
    println!("--- 并发 Future ---");

    println!("Use tokio::join! to wait for multiple futures concurrently");
    println!("Use tokio::spawn to spawn new async tasks");
}

/// 异步 I/O
pub fn async_io() {
    println!("--- 异步 I/O ---");

    println!("tokio::fs for async file operations");
    println!("tokio::net for async networking");
    println!("tokio::io for async I/O traits");
}

/// 选择模式
pub fn select_example() {
    println!("--- select! ---");

    println!("tokio::select! waits for multiple futures concurrently");
    println!("When one completes, the others are cancelled");
}

/// 共享状态
pub fn shared_state() {
    println!("--- 共享状态 ---");

    println!("tokio::sync::Mutex for async-aware mutual exclusion");
    println!("tokio::sync::RwLock for async-aware read-write lock");
    println!("tokio::sync::watch for watching values change");
    println!("tokio::sync::broadcast for broadcasting messages");
}

/// 运行时
pub fn runtime() {
    println!("--- 运行时 ---");

    println!("Tokio runtime executes async tasks");
    println!("multi-threaded scheduler for parallelism");
    println!("current-thread scheduler for single-threaded execution");
}

/// 运行所有异步示例
pub fn run_all() {
    async_basics();
    println!();
    tokio_example();
    println!();
    await_example();
    println!();
    concurrent_futures();
    println!();
    async_io();
    println!();
    select_example();
    println!();
    shared_state();
    println!();
    runtime();
}
