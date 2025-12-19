use std::collections::HashMap;

fn main() {
    // loop
    let max = 100;
    let mut ans = 0;
    let mut count = 1;
    loop {
        if count > max {
            break;
        }
        ans += count;
        count += 1;
    }
    println!("1から{}までの合計は、{}です。", max, ans);

    // while
    let max = 100;
    let mut ans = 0;
    let mut count = 0;
    while count <= max {
        ans += count;
        count += 1;
    }
    println!("1から{}までの合計は、{}です。", max, ans);

    // for
    let max = 100;
    let mut ans = 0;
    for item in 1..=max {
        ans += item;
    }
    println!("1から{}までの合計は、{}です。", max, ans);

    // 配列
    let data = [12, 34, 56, 78, 90];
    let mut ans = 0;
    for item in data {
        ans += item;
    }
    println!("データの合計は、{}です。", ans);

    // タプル
    let taro = ("Taro", 39, true);
    let hanako = ("Hanako", 28, false);
    println!("{:?}", taro);
    println!("{:?}", hanako);
    println!("name: {}, {}", taro.0, hanako.0);
    println!("age: {}, {}", taro.1, hanako.1);
    println!("male?: {}, {}", taro.2, hanako.2);

    // タプルの分配
    let taro = ("Taro", 39, true);
    let hanako = ("Hanako", 28, false);
    let (name, age, male) = taro;
    println!("name:{}, age:{}, male?:{}", name, age, male);
    let (name, age, male) = hanako;
    println!("name:{}, age:{}, male?:{}", name, age, male);

    // 定数のタプル
    const TARO:(&str, i32, bool) = ("Taro", 39, true);
    const HANAKO:(&str, i32, bool) = ("Hanako", 28, false);
    let (name, age, male) = TARO;
    println!("name:{}, age:{}, male?:{}", name, age, male);
    let (name, age, male) = HANAKO;
    println!("name:{}, age:{}, male?:{}", name, age, male);

    // Vec
    let mut data = Vec::new();
    data.push(123);
    data.push(456);
    data.push(789);
    println!("0:{}, 1:{}, 2:{}.", data[0], data[1], data.get(2).unwrap());

    // Vec繰り返し
    let data = vec![123, 456, 789];
    let mut result = 0;
    for item in data {
        result += item;
    }
    println!("データの合計は、{}です。", result);

    // insert/delete
    let mut data = vec![123, 456, 789];
    println!("{:?}", data);
    data.remove(1);
    println!("{:?}", data);
    data.insert(2, 100);
    println!("{:?}", data);

    // HashMap
    let mut map = HashMap::new();
    println!("{:?}", map);
    map.insert(String::from("first"), 123);
    println!("{:?}", map);
    map.insert(String::from("second"), 456);
    println!("{:?}", map);
    map.insert(String::from("third"), 789);
    println!("{:?}", map);
    map.remove("second");
    println!("{:?}", map);

    // HashMap 値の取得
    let mut map = HashMap::new();
    println!("{:?}", map);
    map.insert(String::from("first"), 123);
    println!("{:?}", map);
    map.insert(String::from("second"), map["first"] * 2);
    println!("{:?}", map);
    map.insert(String::from("third"), map.get("first").unwrap() + map.get("second").unwrap());
    println!("{:?}", map);

    // HashMap 繰り返し処理
    let mut map = HashMap::new();
    map.insert(String::from("first"), 123);
    map.insert(String::from("second"), 456);
    map.insert(String::from("third"), 789);
    let mut result = 0;
    for (ky, val) in map {
        println!("{}: {}.", ky, val);
        result += val;
    }
    println!("total: {}.", result);

    // String
    let s1 = String::new();
    let s2 = String::from("Hello");
    let s3 = "World";
    let s4 = s1 + &s2 + &s3;
    println!("{}", s4);

    // push, push_str
    let mut s1 = String::new();
    s1.push_str("Hello");
    s1.push_str("World!");
    s1.push('X');
    println!("{}", s1);

    // insert, remove, clear
    let mut s1 = String::from("Hello,World!");
    println!("{}", s1);
    s1.insert_str(6, " Rust ");
    println!("{}", s1);
    s1.insert(7, '*');
    println!("{}", s1);
    s1.insert(12, '*');
    println!("{}", s1);
    s1.remove(5);
    println!("{}", s1);

    // Range
    let s1 = String::from("Hello,Rust World!");
    println!("{}", s1);
    let s2 = &s1[0..5];
    println!("{}", s2);
    let s3 = &s1[6..10];
    println!("{}", s3);
    let s4 = &s1[11..16];
    println!("{}", s4);
    let s5 = String::new() + s4 + s3 + s2;
    println!("{}", s5);

    println!("===== 2-4 =====");

    // 関数
    hello(String::from("taro"));
    hello(String::from("hanako"));

    // 戻り値
    print_msg(100);
    print_msg(200);
    print_msg(300);

    // 匿名関数
    let calc = |max| {
        let mut result = 0;
        for n in 0..max {
            result += n;
        }
        result
    };

    let print_msg = |max| {
        println!("{} までの合計は、{}です。", max, calc(max));
    };

    print_msg(100);
    print_msg(200);
    print_msg(300);

    // クロージャ
    let max = 100;
    let res = calc(max);
    let print_msg = || {
        println!("{} までの合計は、{} です。", max, res);
    };
    print_msg();

    let max = 200;
    let res = calc(max);
    let print_msg = || {
        println!("0-{} Total: {}", max, res);
    };
    print_msg();

    // クロージャ外とのやりとり
    let mut x = 10;
    let mut double = || {
        x *= 2;
        x
    };
    println!("x = {}.", double());
    println!("x = {}.", double());
    println!("x = {}.", double());

    // クロージャに借用された変数
    let mut x = 10;
    let mut double = || {
        x *= 2;
        x
    };
    println!("x = {}.", double());
    println!("x = {}.", double());
    // x = 100; // 変数xは借用されているので代入できない
    // println!("x = {}.", x);
    println!("x = {}.", double());
}

// 2-4
fn hello(name:String) {
    println!("Hello, {}!", name);
}

fn print_msg(max:i32) {
    println!("{} までの合計は、{} です。", max, calc(max));
}

fn calc(max:i32) -> i32 {
    let mut result = 0;
    for n in 0..max {
        result += n;
    }
    result
}

