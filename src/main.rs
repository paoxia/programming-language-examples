// Rust 语法全面示例

// 1. 变量和数据类型
fn variables_and_types() {
    // 不可变变量（默认）
    let x = 5;
    println!("x = {}", x);

    // 可变变量
    let mut y = 5;
    y = 10;
    println!("y = {}", y);

    // 显式类型标注
    let z: i32 = 15;
    println!("z = {}", z);

    // 基本数据类型
    let a: u8 = 255;
    let b: f64 = 3.14;
    let c: bool = true;
    let d: char = 'R';
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);

    // 复合数据类型
    // 元组
    let tuple: (i32, f64, bool) = (10, 2.5, false);
    println!("Tuple: {:?}", tuple);
    println!("First element: {}", tuple.0);

    // 数组
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", array);
    println!("Third element: {}", array[2]);
}

// 2. 控制流
fn control_flow() {
    // if 表达式
    let number = 7;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // if-let 表达式
    let optional = Some(42);
    if let Some(x) = optional {
        println!("Found: {}", x);
    }

    // loop 循环
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
        println!("Loop iteration: {}", counter);
    }

    // while 循环
    let mut num = 5;
    while num > 0 {
        println!("While loop: {}", num);
        num -= 1;
    }

    // for 循环
    let numbers = [1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("For loop (array): {}", number);
    }

    // for 循环与范围
    for i in 1..6 {
        println!("For loop (range): {}", i);
    }
}

// 3. 函数
fn functions() {
    // 基本函数
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let result = add(5, 3);
    println!("Add result: {}", result);

    // 无返回值函数
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    greet("Rust");

    // 表达式风格函数（无分号）
    fn multiply(x: i32, y: i32) -> i32 {
        x * y
    }

    println!("Multiply result: {}", multiply(4, 6));
}

// 4. 结构体
struct Person {
    name: String,
    age: u32,
}

// 结构体实现
impl Person {
    // 关联函数（静态方法）
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    // 实例方法
    fn greet(&self) {
        println!("Hello, my name is {} and I'm {} years old.", self.name, self.age);
    }

    // 修改实例的方法
    fn celebrate_birthday(&mut self) {
        self.age += 1;
        println!("Happy birthday! Now I'm {} years old.", self.age);
    }
}

// 5. 枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 枚举实现
impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
        }
    }
}

// 6. 模式匹配
fn pattern_matching() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }

    let optional = Some(7);
    match optional {
        Some(x) => println!("Found: {}", x),
        None => println!("Nothing found"),
    }

    // 解构元组
    let pair = (2, 3);
    match pair {
        (x, y) => println!("Pair: ({}, {})", x, y),
    }
}

// 7. 错误处理
fn error_handling() {
    // Result 类型
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    match divide(10.0, 2.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    match divide(5.0, 0.0) {
        Ok(result) => println!("Division result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    // 使用 ? 运算符
    fn calculate() -> Result<f64, String> {
        let result = divide(20.0, 4.0)?;
        Ok(result * 2.0)
    }

    if let Ok(value) = calculate() {
        println!("Calculate result: {}", value);
    }
}

// 8. 泛型
fn generics() {
    // 泛型函数
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = [1, 5, 3, 9, 2];
    println!("Largest number: {}", largest(&numbers));

    let strings = ["apple", "banana", "orange"];
    println!("Largest string: {}", largest(&strings));

    // 泛型结构体
    struct Point<T> {
        x: T,
        y: T,
    }

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.5, y: 3.7 };
    println!("Int point: x={}, y={}", int_point.x, int_point.y);
    println!("Float point: x={}, y={}", float_point.x, float_point.y);
}

// 9. 特质（Traits）
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    retweet_count: u32,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {} ({} retweets)", self.username, self.content, self.retweet_count)
    }
}

// 特质作为参数
fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

// 10. 所有权和借用
fn ownership() {
    // 所有权转移
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权转移给 s2
    // println!("{}", s1); // 这行会编译错误，因为 s1 已经没有所有权

    // 借用
    let s3 = String::from("hello");
    let len = calculate_length(&s3); // 不可变借用
    println!("Length of '{}' is {}.", s3, len);

    // 可变借用
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("Modified string: {}", s4);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// 11. 闭包
fn closures() {
    // 基本闭包
    let add_one = |x| x + 1;
    println!("Closure result: {}", add_one(5));

    // 带类型标注的闭包
    let multiply: fn(i32, i32) -> i32 = |x, y| x * y;
    println!("Multiply closure: {}", multiply(3, 4));

    // 捕获环境变量
    let factor = 2;
    let multiply_by_factor = move |x| x * factor;
    println!("Multiply by factor: {}", multiply_by_factor(5));
}

// 12. 模块和包
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    mod internal {
        pub fn double(x: i32) -> i32 {
            x * 2
        }
    }

    pub fn double_public(x: i32) -> i32 {
        internal::double(x)
    }
}

// 13. 异步编程（基本示例）
async fn async_example() {
    println!("Async function started");
    // 模拟异步操作
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Async function completed");
}

// 14. 智能指针
fn smart_pointers() {
    // Box<T>
    let b = Box::new(5);
    println!("Box contains: {}", *b);

    // Rc<T> (引用计数)
    use std::rc::Rc;
    let a = Rc::new(4);
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    println!("Reference count: {}", Rc::strong_count(&a));

    // RefCell<T> (内部可变性)
    use std::cell::RefCell;
    let mut_value = RefCell::new(5);
    *mut_value.borrow_mut() = 10;
    println!("RefCell value: {}", *mut_value.borrow());
}

// 15. 并发编程
fn concurrency() {
    use std::thread;
    use std::sync::mpsc;
    use std::time::Duration;

    // 通道通信
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["Hello", "from", "the", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}

// 主函数
fn main() {
    println!("=== Rust 语法全面示例 ===\n");

    println!("1. 变量和数据类型");
    variables_and_types();
    println!();

    println!("2. 控制流");
    control_flow();
    println!();

    println!("3. 函数");
    functions();
    println!();

    println!("4. 结构体");
    let mut person = Person::new(String::from("Alice"), 30);
    person.greet();
    person.celebrate_birthday();
    println!();

    println!("5. 枚举");
    let quit_msg = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("Hello Rust"));
    let color_msg = Message::ChangeColor(255, 0, 0);
    quit_msg.process();
    move_msg.process();
    write_msg.process();
    color_msg.process();
    println!();

    println!("6. 模式匹配");
    pattern_matching();
    println!();

    println!("7. 错误处理");
    error_handling();
    println!();

    println!("8. 泛型");
    generics();
    println!();

    println!("9. 特质（Traits）");
    let article = Article {
        title: String::from("Rust Introduction"),
        author: String::from("John Doe"),
        content: String::from("Rust is a systems programming language"),
    };
    let tweet = Tweet {
        username: String::from("@rustlang"),
        content: String::from("Rust 1.60 released!"),
        retweet_count: 100,
    };
    notify(&article);
    notify(&tweet);
    println!();

    println!("10. 所有权和借用");
    ownership();
    println!();

    println!("11. 闭包");
    closures();
    println!();

    println!("12. 模块和包");
    println!("Math add: {}", math::add(5, 3));
    println!("Math subtract: {}", math::subtract(10, 4));
    println!("Math double: {}", math::double_public(6));
    println!();

    println!("13. 智能指针");
    smart_pointers();
    println!();

    println!("14. 并发编程");
    concurrency();
    println!();

    println!("=== 示例结束 ===");
}

