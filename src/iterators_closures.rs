//! 闭包和迭代器教程

/// 闭包基础
pub fn closures_basics() {
    println!("--- 闭包基础 ---");

    // 简单闭包
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // 带类型标注的闭包
    let add_one_typed: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };
    println!("add_one_typed(5) = {}", add_one_typed(5));

    // 多行闭包
    let expensive_closure = |num| {
        println!("calculating slowly...");
        std::thread::sleep(std::time::Duration::from_millis(10));
        num
    };
    println!("expensive_closure(5) = {}", expensive_closure(5));

    // 闭包捕获环境变量
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    println!("equal_to_x(y) = {}", equal_to_x(y));

    // 函数不能捕获环境
    // fn equal_to_x_fn(z: i32) -> bool { z == x } // 错误
}

/// 闭包 trait
pub fn closure_traits() {
    println!("--- 闭包 trait ---");

    // FnOnce: 消耗捕获变量的所有权，只能调用一次
    let s = String::from("hello");
    let f = || s;
    println!("f() = {}", f());
    // f(); // 错误: 不能再次调用

    // FnMut: 可变借用，可以多次调用
    let mut s = String::from("hello");
    let mut f = || s.push_str(", world");
    f();
    println!("s = {}", s);

    // Fn: 不可变借用，可以多次调用
    let s = String::from("hello");
    let f = || println!("s = {}", s);
    f();
    f();
    println!("s still valid: {}", s);

    // move 强制获得所有权
    let s = String::from("hello");
    let f = move || println!("s = {}", s);
    f();
    // println!("s = {}", s); // 错误: s 已被移动
}

/// 迭代器
pub fn iterators() {
    println!("--- 迭代器 ---");

    // Iterator trait
    // trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());
    println!("next: {:?}", iter.next());

    // for 循环使用迭代器
    let v = vec![1, 2, 3];
    print!("for loop: ");
    for val in v {
        print!("{} ", val);
    }
    println!();

    // iter(): 不可变引用
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    println!("iter: {:?}", iter.next());

    // iter_mut(): 可变引用
    let mut v = vec![1, 2, 3];
    let mut iter = v.iter_mut();
    if let Some(x) = iter.next() {
        *x = 10;
    }
    println!("iter_mut: {:?}", v);

    // into_iter(): 获取所有权
    let v = vec![1, 2, 3];
    let mut iter = v.into_iter();
    println!("into_iter: {:?}", iter.next());
    // println!("v: {:?}", v); // 错误: v 已被移动
}

/// 迭代器适配器
pub fn iterator_adapters() {
    println!("--- 迭代器适配器 ---");

    // map: 转换
    let v: Vec<i32> = vec![1, 2, 3];
    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // filter: 过滤
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // enumerate: 添加索引
    let v = vec!['a', 'b', 'c'];
    print!("enumerate: ");
    for (i, val) in v.iter().enumerate() {
        print!("({}: {}) ", i, val);
    }
    println!();

    // zip: 组合两个迭代器
    let a = vec![1, 2, 3];
    let b = vec!['a', 'b', 'c'];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("zip: {:?}", zipped);

    // chain: 连接迭代器
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let chained: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("chain: {:?}", chained);
}

/// 消费适配器
pub fn consuming_adapters() {
    println!("--- 消费适配器 ---");

    // sum: 求和
    let v = vec![1, 2, 3];
    let sum: i32 = v.iter().sum();
    println!("sum: {}", sum);

    // product: 乘积
    let product: i32 = v.iter().product();
    println!("product: {}", product);

    // count: 计数
    let count = v.iter().count();
    println!("count: {}", count);

    // collect: 收集到集合
    let a = [1, 2, 3, 4, 5];
    let v: Vec<i32> = a.iter().map(|&x| x * 2).collect();
    println!("collect: {:?}", v);

    // max: 最大值
    let max = v.iter().max();
    println!("max: {:?}", max);

    // min: 最小值
    let min = v.iter().min();
    println!("min: {:?}", min);

    // fold: 累积
    let sum_fold: i32 = v.iter().fold(0, |acc, &x| acc + x);
    println!("sum_fold: {}", sum_fold);

    // any / all
    let any_gt_5 = v.iter().any(|&x| x > 5);
    let all_positive = v.iter().all(|&x| x > 0);
    println!("any_gt_5: {}", any_gt_5);
    println!("all_positive: {}", all_positive);
}

/// 自定义迭代器
pub fn custom_iterator() {
    println!("--- 自定义迭代器 ---");

    struct Counter {
        count: u32,
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    print!("Counter: ");
    for num in Counter::new() {
        print!("{} ", num);
    }
    println!();

    // 使用其他迭代器方法
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum of custom iterator: {}", sum);
}

/// 使用闭包和迭代器重构
pub fn refactoring() {
    println!("--- 重构示例 ---");

    #[derive(Debug, PartialEq)]
    struct Shoe {
        size: u32,
        style: String,
    }

    // 命令式风格
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        let mut in_size = Vec::new();
        for shoe in shoes {
            if shoe.size == shoe_size {
                in_size.push(shoe);
            }
        }
        in_size
    }

    // 函数式风格
    fn shoes_in_size_fp(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }

    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_size_fp(shoes, 10);
    println!("shoes in my size: {:?}", in_my_size);
}

/// 运行所有示例
pub fn run_all() {
    closures_basics();
    println!();
    closure_traits();
    println!();
    iterators();
    println!();
    iterator_adapters();
    println!();
    consuming_adapters();
    println!();
    custom_iterator();
    println!();
    refactoring();
}
