//! 所有权、借用和生命周期教程

/// 所有权基础
pub fn ownership_basics() {
    println!("--- 所有权基础 ---");

    // String 类型
    let s = String::from("hello");
    println!("s = {}", s);

    // 移动 (Move)
    let s1 = String::from("hello");
    let s2 = s1; // s1 被移动到 s2
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // 错误: s1 已失效

    // 克隆 (Clone)
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 栈上数据: 实现 Copy trait 的类型
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // 没问题，i32 实现了 Copy

    // Copy trait 的类型包括:
    // - 所有整数类型
    // - 布尔类型
    // - 所有浮点数类型
    // - 字符类型
    // - 只包含 Copy 类型的元组
    let tup = (1, 2.5, true);
    let tup2 = tup;
    println!("tup2 = {:?}", tup2);
}

/// 所有权与函数
pub fn ownership_functions() {
    println!("--- 所有权与函数 ---");

    let s = String::from("hello");
    takes_ownership(s); // s 的值移动到函数里
    // println!("{}", s); // 错误: s 不再有效

    let x = 5;
    makes_copy(x); // x 是 Copy 类型，所以后面还可以用
    println!("x still valid: {}", x);

    // 返回值转移所有权
    let s1 = gives_ownership();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);

    // 通过元组返回多个值
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{}' is {}.", s5, len);
}

fn takes_ownership(some_string: String) {
    println!("Took ownership: {}", some_string);
} // some_string 被 drop，内存释放

fn makes_copy(some_integer: i32) {
    println!("Made copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

/// 借用 (Borrowing)
pub fn borrowing() {
    println!("--- 借用 ---");

    // 不可变借用
    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 可变借用
    let mut s = String::from("hello");
    change(&mut s);
    println!("Changed string: {}", s);

    // 可变借用的限制: 同一时间只能有一个可变借用
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // 错误
    println!("r1 = {}", r1);

    // 不可变借用和可变借用不能同时存在
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // 错误
    println!("r1 = {}, r2 = {}", r1, r2);

    // 引用的作用域结束后可以创建新引用
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);
    // r1 和 r2 在这里不再使用

    let r3 = &mut s;
    println!("r3 = {}", r3);
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/// 悬垂引用
pub fn dangling_reference() {
    println!("--- 悬垂引用 ---");

    // 错误示例 (注释掉，因为编译不通过)
    /*
    let reference_to_nothing = dangle();
    */

    // 正确做法: 直接返回 String
    let s = no_dangle();
    println!("s = {}", s);
}

/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s // 返回指向 s 的引用
} // s 被释放，引用无效
*/

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

/// 切片 (Slices)
pub fn slices() {
    println!("--- 切片 ---");

    // 字符串切片
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello = {}, world = {}", hello, world);

    // 语法糖
    let slice = &s[..5]; // 从开头
    let slice = &s[6..]; // 到结尾
    let slice = &s[..]; // 整个字符串
    println!("slice = {}", slice);

    // 实用函数
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("first word = {}", word);
    // s.clear(); // 错误: 因为 word 是对 s 的不可变借用，这里尝试可变借用

    // 字符串字面量就是切片
    let s = "Hello, world!"; // &str 类型
    println!("s = {}", s);

    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice = {:?}", slice);
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/// 生命周期基础
pub fn lifetimes_basics() {
    println!("--- 生命周期基础 ---");

    // 生命周期标注语法
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("longest string = {}", result);
    }

    // 生命周期注解不改变引用的寿命
    /*
    // 这个会报错
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */
}

/// 结构体中的生命周期
pub fn lifetimes_structs() {
    println!("--- 结构体生命周期 ---");

    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i = {:?}", i);

    // 方法中的生命周期
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    println!("i.level() = {}", i.level());
    let part = i.announce_and_return_part("Hello");
    println!("part = {}", part);
}

/// 生命周期省略规则
pub fn lifetimes_elision() {
    println!("--- 生命周期省略 ---");

    // 规则1: 每个引用参数都有自己的生命周期
    // fn foo<'a>(x: &'a i32) {}

    // 规则2: 如果只有一个输入生命周期，它被赋予所有输出生命周期
    // fn foo<'a>(x: &'a i32) -> &'a i32 {}

    // 规则3: 如果有多个输入生命周期，但其中一个是 &self 或 &mut self，
    //         那么 self 的生命周期被赋予所有输出生命周期

    // 省略的例子
    fn first_word_elided(s: &str) -> &str {
        &s[..]
    }

    // 完整形式
    fn first_word_full<'a>(s: &'a str) -> &'a str {
        &s[..]
    }

    let s = "hello";
    println!("first_word_elided = {}", first_word_elided(s));
    println!("first_word_full = {}", first_word_full(s));
}

/// 静态生命周期
pub fn lifetimes_static() {
    println!("--- 静态生命周期 ---");

    // 'static 生命周期持续整个程序
    let s: &'static str = "I have a static lifetime.";
    println!("static str = {}", s);

    // 字符串字面量都是 'static
    let t = "This is also static";
    println!("t = {}", t);
}

/// 泛型、trait bound 和生命周期一起用
pub fn lifetimes_combined() {
    println!("--- 综合示例 ---");

    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2.as_str(),
        "Choosing the longest!",
    );
    println!("result = {}", result);
}

/// 运行所有所有权示例
pub fn run_all() {
    ownership_basics();
    println!();
    ownership_functions();
    println!();
    borrowing();
    println!();
    dangling_reference();
    println!();
    slices();
    println!();
    lifetimes_basics();
    println!();
    lifetimes_structs();
    println!();
    lifetimes_elision();
    println!();
    lifetimes_static();
    println!();
    lifetimes_combined();
}
