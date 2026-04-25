//! 高级特性教程

/// 不安全 Rust
pub fn unsafe_rust() {
    println!("--- 不安全 Rust ---");

    // 不安全的五大特性：
    // 1. 解引用裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 points to: {}", *r1);
        *r2 = 10;
        println!("r2 points to: {}", *r2);
    }
    println!("num after change: {}", num);

    // 2. 调用不安全的函数或方法
    unsafe fn dangerous() {
        println!("In dangerous function");
    }
    unsafe {
        dangerous();
    }

    // 3. 访问或修改可变静态变量
    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            // 使用原始指针访问
            let ptr = std::ptr::addr_of_mut!(COUNTER);
            *ptr += inc;
        }
    }
    add_to_count(3);
    unsafe {
        let ptr = std::ptr::addr_of!(COUNTER);
        println!("COUNTER: {}", *ptr);
    }

    // 4. 实现不安全 trait
    unsafe trait Foo {}
    unsafe impl Foo for i32 {}
    println!("Unsafe trait implemented");

    // 5. 访问 union 的字段
    #[repr(C)]
    union MyUnion {
        i: i32,
        f: f32,
    }
    let mut u = MyUnion { i: 42 };
    unsafe {
        println!("Union i: {}", u.i);
        u.f = 1.0;
        println!("Union f: {}", u.f);
    }

    println!("unsafe 不关闭借用检查，也不提供内存安全保证");
    println!();
}

/// 模式匹配进阶
pub fn patterns() {
    println!("--- 模式匹配 ---");

    // 字面量模式
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 命名变量模式
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    // 多个模式
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 范围模式
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // 解构结构体
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);
    let Point { x, y } = p; // 简写
    println!("x = {}, y = {}", x, y);

    // 匹配守卫
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // @ 绑定
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

/// 高级 trait
pub fn advanced_traits() {
    println!("--- 高级 trait ---");

    // 关联类型
    pub trait Iterator {
        type Item; // 关联类型
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
    }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    println!("Associated types define placeholders in trait definitions");

    // 泛型类型参数与默认类型
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Millimeters(u32);

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    println!("Default type parameters let you extend a trait without breaking existing code");

    // 完全限定语法
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // 父 trait
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}

    let p = Point { x: 1, y: 3 };
    p.outline_print();

    // newtype 模式
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

/// 高级类型
pub fn advanced_types() {
    println!("--- 高级类型 ---");

    // newtype 模式
    struct Millimeters(u32);
    struct Meters(u32);

    let _mm = Millimeters(10);
    let _m = Meters(1);

    // 类型别名
    type Kilometers = u32;
    let _km: Kilometers = 5;

    type Thunk = Box<dyn Fn() + Send + 'static>;
    let _f: Thunk = Box::new(|| println!("hi"));

    // never 类型 !
    fn bar() -> ! {
        panic!();
    }

    // 动态大小类型
    let s: &str = "Hello there!";
    println!("str size varies: {}", s.len());

    trait MyTrait {
        fn foo(&self);
    }
    // dyn MyTrait 是动态大小类型
}

/// 高级函数与闭包
pub fn advanced_functions() {
    println!("--- 高级函数与闭包 ---");

    // 函数指针
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    println!("list_of_strings: {:?}", list_of_strings);
    println!("list_of_strings2: {:?}", list_of_strings2);

    // 返回闭包
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    let f = returns_closure();
    println!("returns_closure: {}", f(5));
}

/// 宏
pub fn macros() {
    println!("--- 宏 ---");

    // 声明宏
    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }
    let v = vec![1, 2, 3];
    println!("vec! macro: {:?}", v);

    println!("Macros are a way of writing code that writes other code");
    println!("Procedural macros: #[derive], attribute-like, function-like");
}

/// 运行所有高级特性示例
pub fn run_all() {
    unsafe_rust();
    patterns();
    println!();
    advanced_traits();
    println!();
    advanced_types();
    println!();
    advanced_functions();
    println!();
    macros();
}
