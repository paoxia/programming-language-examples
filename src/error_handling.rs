//! 错误处理教程

/// panic! 宏
pub fn panic_example() {
    println!("--- panic! 宏 ---");

    // 直接调用 panic!
    // panic!("crash and burn");

    // 越界访问会 panic
    let v = vec![1, 2, 3];
    // v[99]; // 这会 panic

    println!("panic! 示例 (已注释)");

    // 使用 RUST_BACKTRACE=1 查看调用栈
}

/// Result 枚举
pub fn result_enum() {
    println!("--- Result 枚举 ---");

    use std::fs::File;
    use std::io::ErrorKind;

    // Result<T, E>
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // 处理打开文件可能的错误
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // 使用闭包更简洁
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    println!("Result 处理完成");
}

/// unwrap 和 expect
pub fn unwrap_expect() {
    println!("--- unwrap 和 expect ---");

    use std::fs::File;

    // unwrap: 如果是 Ok 就返回值，是 Err 就 panic
    // let _greeting_file = File::open("hello.txt").unwrap();

    // expect: 类似 unwrap，但可以自定义 panic 信息
    // let _greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    println!("unwrap/expect 示例 (已注释)");
}

/// 传播错误
pub fn propagating_errors() {
    println!("--- 传播错误 ---");

    use std::fs::File;
    use std::io::{self, Read};

    // 手动传播
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    match read_username_from_file() {
        Ok(s) => println!("Read username: {}", s),
        Err(e) => println!("Error reading: {}", e),
    }

    // 使用 ? 运算符传播错误
    fn read_username_from_file2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    match read_username_from_file2() {
        Ok(s) => println!("Read username 2: {}", s),
        Err(e) => println!("Error reading 2: {}", e),
    }

    // 链式调用
    fn read_username_from_file3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    match read_username_from_file3() {
        Ok(s) => println!("Read username 3: {}", s),
        Err(e) => println!("Error reading 3: {}", e),
    }

    // 使用标准库函数
    fn read_username_from_file4() -> Result<String, io::Error> {
        std::fs::read_to_string("hello.txt")
    }

    match read_username_from_file4() {
        Ok(s) => println!("Read username 4: {}", s),
        Err(e) => println!("Error reading 4: {}", e),
    }
}

/// ? 运算符可以用于 Option
pub fn question_mark_option() {
    println!("--- ? 与 Option ---");

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    let text = "hello\nworld";
    println!("last_char_of_first_line: {:?}", last_char_of_first_line(text));

    let empty = "";
    println!("last_char_of_empty: {:?}", last_char_of_first_line(empty));
}

/// 创建自定义错误类型
pub fn custom_errors() {
    println!("--- 自定义错误类型 ---");

    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    enum ParseError {
        EmptyString,
        InvalidNumber,
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ParseError::EmptyString => write!(f, "string is empty"),
                ParseError::InvalidNumber => write!(f, "string is not a number"),
            }
        }
    }

    impl Error for ParseError {}

    fn parse_number(s: &str) -> Result<i32, ParseError> {
        if s.is_empty() {
            return Err(ParseError::EmptyString);
        }
        s.parse().map_err(|_| ParseError::InvalidNumber)
    }

    match parse_number("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match parse_number("") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match parse_number("not a number") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

/// Box<dyn Error> trait 对象
pub fn boxed_error() {
    println!("--- Box<dyn Error> ---");

    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let _greeting_file = File::open("hello.txt")?;
        Ok(())
    }

    match main() {
        Ok(_) => println!("No error"),
        Err(e) => println!("Got error: {}", e),
    }
}

/// 错误处理指导原则
pub fn error_handling_guidelines() {
    println!("--- 错误处理指导原则 ---");

    println!("1. 在可能有害的状态时使用 panic!");
    println!("2. 当你不希望处理错误时使用 unwrap/expect");
    println!("3. 使用 Result 传播错误给调用者");
    println!("4. 创建自定义错误类型表达业务逻辑错误");
    println!("5. 使用 ? 运算符简化错误传播");
    println!("6. 合理使用 panic! 作为快速原型");
}

/// 运行所有错误处理示例
pub fn run_all() {
    panic_example();
    println!();
    result_enum();
    println!();
    unwrap_expect();
    println!();
    propagating_errors();
    println!();
    question_mark_option();
    println!();
    custom_errors();
    println!();
    boxed_error();
    println!();
    error_handling_guidelines();
}
