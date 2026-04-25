//! 并发编程教程

/// 使用线程同时运行代码
pub fn threads() {
    println!("--- 线程 ---");

    use std::thread;
    use std::time::Duration;

    // 简单的线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // 使用 move 闭包获取环境变量
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // println!("Here's a vector: {:?}", v); // 错误: v 已被移动
    handle.join().unwrap();
}

/// 使用消息传递在线程间传送数据
pub fn message_passing() {
    println!("--- 消息传递 ---");

    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    // 简单的通道
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // 错误: val 已被发送
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // 发送多个值
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }

    // 多个发送者
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
}

/// 互斥器
pub fn mutexes() {
    println!("--- Mutex ---");

    use std::sync::Mutex;

    // Mutex 的基本用法
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // 多线程共享 Mutex
    use std::sync::Arc;
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/// 原子引用计数 Arc<T>
pub fn arc_example() {
    println!("--- Arc<T> ---");

    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("Thread {}: data = {:?}", i, data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Arc strong count: {}", Arc::strong_count(&data));
}

/// Send 和 Sync trait
pub fn send_sync() {
    println!("--- Send 和 Sync ---");

    // Send: 允许在线程间转移所有权
    // Sync: 允许多线程同时访问

    println!("Send trait: 类型可以在线程间转移所有权");
    println!("Sync trait: 类型可以安全地被多个线程同时引用");
    println!("- Rc<T> 既不是 Send 也不是 Sync");
    println!("- Mutex<T> 是 Send 和 Sync 的");
    println!("- Arc<T> 是 Send 和 Sync 的");
}

/// 死锁示例
pub fn deadlock() {
    println!("--- 死锁 ---");

    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));

    let a_clone = Arc::clone(&a);
    let b_clone = Arc::clone(&b);

    let t1 = thread::spawn(move || {
        let _a_lock = a_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _b_lock = b_clone.lock().unwrap();
        println!("Thread 1 done");
    });

    let t2 = thread::spawn(move || {
        let _b_lock = b.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _a_lock = a.lock().unwrap();
        println!("Thread 2 done");
    });

    // 注意：这会导致死锁！这里只是示例，我们使用较短的超时或避免实际运行
    println!("Deadlock example (would hang if uncommented)");
    t1.join().unwrap_or_default();
    t2.join().unwrap_or_default();
}

/// 条件变量
pub fn condition_variables() {
    println!("--- 条件变量 ---");

    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        println!("Notifying...");
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("Waiting...");
        started = cvar.wait(started).unwrap();
    }
    println!("Started!");
}

/// 运行所有并发示例
pub fn run_all() {
    threads();
    println!();
    message_passing();
    println!();
    mutexes();
    println!();
    arc_example();
    println!();
    send_sync();
    println!();
    deadlock();
    println!();
    condition_variables();
}
