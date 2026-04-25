//! 结构体、枚举和模式匹配教程

/// 结构体定义和实例化
pub fn struct_definition() {
    println!("--- 结构体定义 ---");

    // 普通结构体
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // 创建实例
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("user1 = {:?}", user1);

    // 修改字段
    user1.email = String::from("anotheremail@example.com");
    println!("user1.email = {}", user1.email);

    // 字段初始化简写
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // 同名字段简写
            email,
            sign_in_count: 1,
        }
    }
    let user2 = build_user(
        String::from("user2@example.com"),
        String::from("user2"),
    );
    println!("user2 = {:?}", user2);

    // 结构体更新语法
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2 // 其余字段来自 user2
    };
    println!("user3 = {:?}", user3);
    // 注意: user2 的 username String 被移动到 user3 了，user2 不再可用
}

/// 元组结构体
pub fn tuple_structs() {
    println!("--- 元组结构体 ---");

    #[derive(Debug)]
    struct Color(i32, i32, i32);

    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black = {:?}", black);
    println!("origin = {:?}", origin);

    // 访问元组结构体字段
    println!("black.0 = {}", black.0);
    println!("origin.1 = {}", origin.1);

    // 解构
    let Color(r, g, b) = black;
    println!("r={}, g={}, b={}", r, g, b);
}

/// 类单元结构体
pub fn unit_structs() {
    println!("--- 类单元结构体 ---");

    #[derive(Debug)]
    struct AlwaysEqual;

    let subject = AlwaysEqual;
    println!("subject = {:?}", subject);
}

/// 结构体方法
pub fn struct_methods() {
    println!("--- 结构体方法 ---");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 方法
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn width(&self) -> bool {
            self.width > 0
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 = {:?}", rect1);
    println!("rect1.area() = {}", rect1.area());
    println!("rect1.width() = {}", rect1.width());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));
}

/// 关联函数
pub fn associated_functions() {
    println!("--- 关联函数 ---");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 关联函数 (没有 self 参数)
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let sq = Rectangle::square(32);
    println!("sq = {:?}", sq);

    // 多个 impl 块
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    println!("sq.area() = {}", sq.area());
}

/// 枚举定义
pub fn enum_definition() {
    println!("--- 枚举定义 ---");

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four = {:?}", four);
    println!("six = {:?}", six);

    // 将数据附加到枚举变体
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home = {:?}", home);
    println!("loopback = {:?}", loopback);

    // 枚举变体可以有不同类型的关联数据
    #[derive(Debug)]
    enum Message {
        Quit,                       // 没有关联数据
        Move { x: i32, y: i32 },    // 匿名结构体
        Write(String),              // String
        ChangeColor(i32, i32, i32), // 三个 i32
    }

    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("hello"));
    let color_msg = Message::ChangeColor(0, 160, 255);
    println!("quit = {:?}", quit);
    println!("move_msg = {:?}", move_msg);
    println!("write_msg = {:?}", write_msg);
    println!("color_msg = {:?}", color_msg);
}

/// Option 枚举
pub fn option_enum() {
    println!("--- Option 枚举 ---");

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("some_char = {:?}", some_char);
    println!("absent_number = {:?}", absent_number);

    // Option 与 T 是不同类型
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // 错误: 不能相加

    // 处理 Option
    if let Some(n) = y {
        let sum = x + n;
        println!("sum = {}", sum);
    }
}

/// match 控制流
pub fn match_control_flow() {
    println!("--- match 控制流 ---");

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    println!("Penny: {}", value_in_cents(Coin::Penny));
    println!("Nickel: {}", value_in_cents(Coin::Nickel));
    println!("Dime: {}", value_in_cents(Coin::Dime));
    println!("Quarter: {}", value_in_cents(Coin::Quarter));

    // 绑定值的模式
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    #[derive(Debug)]
    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents2(coin: Coin2) -> u8 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    println!(
        "Quarter: {}",
        value_in_cents2(Coin2::Quarter(UsState::Alaska))
    );

    // 匹配 Option<T>
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five = {:?}", five);
    println!("six = {:?}", six);
    println!("none = {:?}", none);

    // 通配模式
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // 使用 _ 忽略特定值
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // 使用 () 表示什么都不做
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    println!("Move player {} spaces", num_spaces);
}
fn reroll() {
    println!("Reroll");
}

/// if let 简洁控制流
pub fn if_let() {
    println!("--- if let ---");

    let config_max = Some(3u8);
    // match 方式
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // if let 方式
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if let with else
    let mut count = 0;
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    let coin = Coin::Penny;

    if let Coin::Quarter = coin {
        println!("State quarter!");
    } else {
        count += 1;
    }
    println!("count = {}", count);
}

/// 运行所有示例
pub fn run_all() {
    struct_definition();
    println!();
    tuple_structs();
    println!();
    unit_structs();
    println!();
    struct_methods();
    println!();
    associated_functions();
    println!();
    enum_definition();
    println!();
    option_enum();
    println!();
    match_control_flow();
    println!();
    if_let();
}
