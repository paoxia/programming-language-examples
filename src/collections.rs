//! 标准库集合类型教程

/// Vector
pub fn vectors() {
    println!("--- Vector ---");

    // 创建空 Vector
    let v: Vec<i32> = Vec::new();
    println!("Empty vector: {:?}", v);

    // 使用 vec! 宏创建并初始化
    let v = vec![1, 2, 3];
    println!("vec! macro: {:?}", v);

    // 更新 vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("After push: {:?}", v);

    // 读取元素
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 索引越界
    // let does_not_exist = &v[100]; // panic!
    let does_not_exist = v.get(100); // 返回 None
    println!("does_not_exist: {:?}", does_not_exist);

    // 借用规则
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // 错误: 不能同时拥有可变和不可变借用
    println!("The first element is: {}", first);
    v.push(6); // 现在可以了，first 不再使用

    // 遍历
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 可变遍历
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 解引用
        println!("{}", i);
    }

    // 使用 enum 存储多种类型
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Row: {:?}", row);
}

/// String
pub fn strings() {
    println!("--- String ---");

    // 创建 String
    let mut s = String::new();
    println!("Empty String: '{}'", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("to_string(): '{}'", s);

    let s = String::from("initial contents");
    println!("String::from(): '{}'", s);

    // String 是 UTF-8 编码的
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    println!("Hello in Chinese: {}", hello);

    // 更新 String
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("push_str: '{}'", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is still valid: '{}'", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("push: '{}'", s);

    // 拼接
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 被移动了，不能再使用
    println!("+ operator: '{}'", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: '{}'", s);
    println!("s1 still valid: '{}'", s1);

    // 不能索引 String
    let s1 = String::from("hello");
    // let h = s1[0]; // 错误
    println!("String length in bytes: {}", s1.len());

    let hello = String::from("Здравствуйте");
    println!("'{}' length in bytes: {}", hello, hello.len());

    // 字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("slice [0..4]: '{}'", s);

    // &hello[0..1] // 错误: 没有在字符边界

    // 遍历字符
    println!("Iterating chars:");
    for c in "Зд".chars() {
        println!("  {}", c);
    }

    // 遍历字节
    println!("Iterating bytes:");
    for b in "Зд".bytes() {
        println!("  {}", b);
    }
}

/// HashMap
pub fn hash_maps() {
    println!("--- HashMap ---");

    use std::collections::HashMap;

    // 创建空 HashMap
    let mut scores = HashMap::new();

    // 插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    // 使用 collect 创建
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores from collect: {:?}", scores);

    // 所有权
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name 和 field_value 已经被移动，不再有效
    println!("map: {:?}", map);

    // 访问值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue team score: {:?}", score);

    // 遍历
    println!("Iterating:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // 覆盖值
    scores.insert(String::from("Blue"), 25);
    println!("After overwrite: {:?}", scores);

    // 只在键没有对应值时插入
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    println!("After entry: {:?}", scores);

    // 更新基于旧值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word count: {:?}", map);
}

/// HashSet
pub fn hash_sets() {
    println!("--- HashSet ---");

    use std::collections::HashSet;

    // 创建 HashSet
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("set: {:?}", set);

    // 使用 collect
    let a = vec![1, 2, 3];
    let b: HashSet<_> = a.into_iter().collect();
    println!("from vec: {:?}", b);

    // 包含
    println!("contains 2? {}", set.contains(&2));
    println!("contains 5? {}", set.contains(&5));

    // 插入已存在值返回 false
    let was_there = set.insert(2);
    println!("insert 2 again: was_there={}", was_there);

    // 移除
    set.remove(&2);
    println!("after remove 2: {:?}", set);

    // 集合操作
    let set1: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5].iter().cloned().collect();

    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("intersection: {:?}", intersection);

    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("union: {:?}", union);

    let diff: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("difference: {:?}", diff);

    let sym_diff: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    println!("symmetric difference: {:?}", sym_diff);
}

/// 运行所有集合示例
pub fn run_all() {
    vectors();
    println!();
    strings();
    println!();
    hash_maps();
    println!();
    hash_sets();
}
