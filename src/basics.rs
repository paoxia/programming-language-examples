//! Rust 基础语法教程模块

/// 变量和可变性
pub fn variables_mutability() {
    println!("--- 变量和可变性 ---");

    // 不可变变量
    let x = 5;
    println!("不可变变量 x = {}", x);

    // 可变变量
    let mut y = 5;
    println!("可变变量 y = {}", y);
    y = 10;
    println!("修改后 y = {}", y);

    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("常量 MAX_POINTS = {}", MAX_POINTS);

    // 变量遮蔽 (Shadowing)
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("变量遮蔽后 z = {}", z);

    // 遮蔽可以改变类型
    let spaces = "   ";
    let spaces = spaces.len();
    println!("遮蔽改变类型后 spaces = {}", spaces);
}

/// 数据类型
pub fn data_types() {
    println!("--- 数据类型 ---");

    // 整数类型
    let _u8: u8 = 255;
    let _i8: i8 = -128;
    let _u32: u32 = 0;
    let _i32: i32 = -1_000_000;
    let _isize: isize = 42;
    let _usize: usize = 100;

    // 数字字面量
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    println!("数字字面量: {}, {}, {}, {}, {}", decimal, hex, octal, binary, byte);

    // 浮点类型
    let f32: f32 = 3.0;
    let f64: f64 = 3.141592653589793;
    println!("浮点: {} (f32), {} (f64)", f32, f64);

    // 数值运算
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 向零截断
    let remainder = 43 % 5;
    println!("运算: sum={}, diff={}, product={}, quotient={}, truncated={}, rem={}",
             sum, difference, product, quotient, truncated, remainder);

    // 布尔类型
    let t = true;
    let f: bool = false;
    println!("布尔: t={}, f={}", t, f);

    // 字符类型
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("字符: {}, {}, {}", c, z, heart_eyed_cat);

    // 元组
    let tup: (i32, f64, char) = (500, 6.4, 'z');
    println!("元组: {:?}", tup);
    let (a, b, c) = tup; // 解构
    println!("解构元组: a={}, b={}, c={}", a, b, c);
    println!("元组索引: tup.0={}, tup.1={}, tup.2={}", tup.0, tup.1, tup.2);

    // 数组
    let arr = [1, 2, 3, 4, 5];
    println!("数组: {:?}", arr);
    let first = arr[0];
    let second = arr[1];
    println!("数组元素: first={}, second={}", first, second);

    // 数组初始化
    let zeros = [0; 5];
    println!("零数组: {:?}", zeros);
}

/// 函数
pub fn functions() {
    println!("--- 函数 ---");

    // 基本函数
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("add(5, 3) = {}", add(5, 3));

    // 无返回值函数
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
    greet("Rustacean");

    // 表达式 vs 语句
    let y = {
        let x = 3;
        x + 1 // 没有分号，这是表达式
    };
    println!("y = {}", y);

    // 早期返回
    fn early_return(x: i32) -> i32 {
        if x < 0 {
            return 0;
        }
        x * 2
    }
    println!("early_return(-5) = {}", early_return(-5));
    println!("early_return(10) = {}", early_return(10));
}

/// 控制流
pub fn control_flow() {
    println!("--- 控制流 ---");

    // if 表达式
    let number = 7;
    if number < 5 {
        println!("小于5");
    } else if number == 5 {
        println!("等于5");
    } else {
        println!("大于5");
    }

    // if 作为表达式赋值
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);

    // loop 循环
    println!("loop 循环:");
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 3 {
            break count * 2; // break 可以返回值
        }
        println!("  count = {}", count);
    };
    println!("loop 结果 = {}", result);

    // 循环标签
    println!("嵌套循环与标签:");
    let mut count = 0;
    'outer: loop {
        let mut remaining = 3;
        loop {
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            println!("  count={}, remaining={}", count, remaining);
            remaining -= 1;
        }
        count += 1;
    }

    // while 循环
    println!("while 循环:");
    let mut number = 3;
    while number != 0 {
        println!("  {}", number);
        number -= 1;
    }
    println!("  发射!");

    // for 循环遍历数组
    println!("for 循环:");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("  值为: {}", element);
    }

    // for 循环遍历范围
    println!("for 范围循环:");
    for number in 1..4 {
        println!("  {}", number);
    }
    println!("反向:");
    for number in (1..4).rev() {
        println!("  {}", number);
    }
}

/// 运行所有基础示例
pub fn run_all() {
    variables_mutability();
    println!();
    data_types();
    println!();
    functions();
    println!();
    control_flow();
}
