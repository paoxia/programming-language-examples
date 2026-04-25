//! Rust 完整教程 - 主文件
//!
//! 这是一个全面的 Rust 语言教程，涵盖从基础到高级的所有特性。

// 模块声明
mod basics;
mod ownership;
mod structs_enums;
mod collections;
mod error_handling;
mod generics_traits;
mod iterators_closures;
mod smart_pointers;
mod concurrency;
mod async_await;
mod advanced;

/// 打印章节标题
fn print_chapter(title: &str) {
    let len = title.chars().count() + 4;
    println!("\n{}", "=".repeat(len));
    println!("  {}", title);
    println!("{}", "=".repeat(len));
}

/// 打印分隔线
fn print_separator() {
    println!("\n{}", "-".repeat(60));
}

/// 主函数 - 运行所有教程示例
fn main() {
    println!("\n{}", "=".repeat(60));
    println!("        R U S T   完 整 教 程");
    println!("{}", "=".repeat(60));

    // 第 1 部分：基础
    print_chapter("第一部分：基础语法");
    basics::run_all();
    print_separator();

    // 第 2 部分：所有权系统
    print_chapter("第二部分：所有权与借用");
    ownership::run_all();
    print_separator();

    // 第 3 部分：结构体与枚举
    print_chapter("第三部分：结构体、枚举与模式匹配");
    structs_enums::run_all();
    print_separator();

    // 第 4 部分：集合类型
    print_chapter("第四部分：标准库集合");
    collections::run_all();
    print_separator();

    // 第 5 部分：错误处理
    print_chapter("第五部分：错误处理");
    error_handling::run_all();
    print_separator();

    // 第 6 部分：泛型与 Trait
    print_chapter("第六部分：泛型、Trait 与生命周期");
    generics_traits::run_all();
    print_separator();

    // 第 7 部分：迭代器与闭包
    print_chapter("第七部分：迭代器与闭包");
    iterators_closures::run_all();
    print_separator();

    // 第 8 部分：智能指针
    print_chapter("第八部分：智能指针");
    smart_pointers::run_all();
    print_separator();

    // 第 9 部分：并发编程
    print_chapter("第九部分：并发编程");
    concurrency::run_all();
    print_separator();

    // 第 10 部分：异步编程
    print_chapter("第十部分：异步编程");
    async_await::run_all();
    print_separator();

    // 第 11 部分：高级特性
    print_chapter("第十一部分：高级特性");
    advanced::run_all();
    print_separator();

    // 结束语
    println!("\n{}", "=".repeat(60));
    println!("          教程运行完成！");
    println!("{}", "=".repeat(60));
    println!("\n接下来你可以：");
    println!("  1. 阅读各个模块的源代码来深入理解");
    println!("  2. 修改示例代码进行实验");
    println!("  3. 编写自己的 Rust 程序");
    println!("  4. 阅读 Rust 官方文档 https://doc.rust-lang.org/");
    println!();
}
