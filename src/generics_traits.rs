//! 泛型、trait 和生命周期教程

/// 函数定义中的泛型
pub fn generic_functions() {
    println!("--- 泛型函数 ---");

    // 找出最大元素
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

/// 结构体中的泛型
pub fn generic_structs() {
    println!("--- 泛型结构体 ---");

    // 单泛型参数
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer = {:?}", integer);
    println!("float = {:?}", float);

    // 多个泛型参数
    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("both_integer = {:?}", both_integer);
    println!("both_float = {:?}", both_float);
    println!("integer_and_float = {:?}", integer_and_float);
}

/// 枚举中的泛型
pub fn generic_enums() {
    println!("--- 泛型枚举 ---");

    // Option<T>
    enum Option<T> {
        Some(T),
        None,
    }

    // Result<T, E>
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    println!("Option<T> 和 Result<T, E> 是标准库的泛型枚举");
}

/// 方法定义中的泛型
pub fn generic_methods() {
    println!("--- 泛型方法 ---");

    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x() = {}", p.x());

    // 只对特定类型实现方法
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 3.0f32, y: 4.0f32 };
    println!("distance_from_origin = {}", p.distance_from_origin());

    // 方法有不同的泛型参数
    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3 = {:?}", p3);
}

/// trait 定义
pub fn trait_definition() {
    println!("--- trait 定义 ---");

    // 定义一个 trait
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // 实现 trait
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());
}

/// trait 默认实现
pub fn trait_default_impl() {
    println!("--- trait 默认实现 ---");

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // 使用默认实现
    impl Summary for NewsArticle {}

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
             hockey team in the NHL."),
    };
    println!("New article available! {}", article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

/// trait 作为参数
pub fn trait_as_parameter() {
    println!("--- trait 作为参数 ---");

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // impl Trait 语法
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait Bound 语法
    pub fn notify2<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    notify(&tweet);
    notify2(&tweet);

    // 通过 + 指定多个 trait bound
    use std::fmt::Display;
    pub fn notify_display<T: Summary + Display>(_item: &T) {}

    // where 子句
    pub fn some_function<T, U>(_t: &T, _u: &U)
    where
        T: Display + Clone,
        U: Clone + std::fmt::Debug,
    {
    }
}

/// 返回实现 trait 的类型
pub fn return_trait() {
    println!("--- 返回 trait ---");

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // 返回 impl Trait
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        }
    }

    let s = returns_summarizable();
    println!("summarizable: {}", s.summarize());

    // 注意: 不能返回不同的类型
    /*
    fn returns_summarizable2(switch: bool) -> impl Summary {
        if switch {
            NewsArticle { ... }
        } else {
            Tweet { ... }
        }
    }
    */
}

/// 使用 trait bound 修复 largest 函数
pub fn fix_largest() {
    println!("--- 修复 largest ---");

    // 同时要求 PartialOrd 和 Clone trait
    fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
        let mut largest = list[0].clone();
        for item in list {
            if item > &largest {
                largest = item.clone();
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // 或者返回引用
    fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_ref(&char_list);
    println!("The largest char is {}", result);
}

/// 使用 trait bound 有条件地实现方法
pub fn conditional_impl() {
    println!("--- 有条件实现 ---");

    use std::fmt::Display;

    #[derive(Debug)]
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    // 只为实现了 Display 和 PartialOrd 的类型实现方法
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let pair = Pair::new(5, 10);
    pair.cmp_display();

    // 有条件地实现 trait (blanket implementation)
    // 例如，标准库对任何实现了 Display 的类型自动实现 ToString trait
}

/// 运行所有示例
pub fn run_all() {
    generic_functions();
    println!();
    generic_structs();
    println!();
    generic_enums();
    println!();
    generic_methods();
    println!();
    trait_definition();
    println!();
    trait_default_impl();
    println!();
    trait_as_parameter();
    println!();
    return_trait();
    println!();
    fix_largest();
    println!();
    conditional_impl();
}
