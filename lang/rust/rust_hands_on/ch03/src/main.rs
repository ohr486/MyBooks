fn main() {
    println!("===== 3-1 所有権と参照 =====");

    // リテラルの所有権
    let msg = "Hello!";
    let msg2 = msg;
    println!("{}", msg);
    println!("{}", msg2);

    // 様々な値の所有権を調べる
    let msg = String::from("Hello!");
    // let msg2 = msg; // error
    let msg2 = msg.clone();
    println!("{}", msg);
    println!("{}", msg2);

    // 数値の代入とコピー
    let num = 1234;
    let num2 = num;
    println!("num: {}", num);
    println!("num2: {}", num2);

    // 関数と所有権の移動
    let msg = String::from("Hello!");
    // print_msg1(msg); // error
    print_msg1(msg.clone()); // error
    println!("msg: {}", msg);

    // 戻り値で所有権を返す
    let mut msg = String::from("Hello!");
    msg = print_msg2(msg);
    println!("msg: {}", msg);

    // 参照
    let msg = String::from("Hello!");
    print_msg3(&msg);
    println!("msg: {}", msg);

    // 参照とシャドーイング
    let msg = &String::from("Hello!");
    println!("[out block] msg: {}", msg);
    {
        let msg = print_msg4(msg);
        println!("[in block] msg: {}", msg);
    }
    println!("[out block] msg: {}", msg);

    // 参照した値の変更
    let mut msg = String::from("Hello");
    println!("[before] msg: {}", msg);
    print_msg5(&mut msg);
    println!("[after] msg: {}", msg);

    // 参照元の値の取得
    let msg = "Hello!";
    let msg_p = &msg;
    let msg_v = *msg_p;
    println!("msg:{}, msg_p:{}, msg_v:{}.", msg, msg_p, msg_v);

    // スライス
    let msg = "Hello, wrold!";
    let world = &msg[7..12];
    println!("`{}` in `{}`.", world, msg);

    // 可変テキストのスライス
    // let mut msg = String::from("Hello, world!");
    // let world = &msg[7..12];
    // println!("[before insert] `{}` in `{}`.", world, msg);
    // msg.insert_str(7, "RUST?"); // error
    // println!("[after insert] `{}` in `{}`.", world, msg);

    // スライスを元に可変テキストを作成
    let mut msg = String::from("Hello, world!");
    let world = String::from(&msg[7..12]);
    println!("[before insert] `{}` in `{}`.", world, msg);
    msg.insert_str(7, "RUST?");
    let mut world = String::from(&msg[7..12]);
    world.push('!');
    println!("[after insert] `{}` in `{}`.", world, msg);

    // 配列のスライス
    let data = [12, 34, 56, 78, 90];
    let part = &data[2..4];
    println!("{:?} in {:?}", part, data);

    // 配列スライスも不変
    // let mut data = vec![12, 34, 56, 78, 90];
    // let part = &data[2..4];
    // data.insert(1, 999); // error
    // println!("{:?} in {:?}", part, data);

    // 配列スライスをVecとして使う
    let mut data = vec![12, 34, 56, 78, 90];
    let mut part = data[2..4].to_vec();
    data.insert(3, 999);
    part.push(-1);
    println!("{:?} in {:?}", part, data);

    println!("===== 3-2 構造体 =====");

    // 構造体を使ってみる
    let taro = Person1 {
        name: String::from("Taro"),
        mail: String::from("taro@yamada"),
        age: 39,
    };
    let hanako = Person1 {
        name: String::from("Hanako"),
        mail: String::from("hanako@flower"),
        age: 28,
    };
    print_person1(taro);
    print_person1(hanako);

    // 省略記法によるインスタンス作成
    let taro = person1(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = person1(String::from("Hanako"), String::from("hanako@flower"), 39);
    print_person1(taro);
    print_person1(hanako);

    // タプル構造体
    let taro = Person2(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = Person2(String::from("Hanako"), String::from("hanako@flower"), 28);
    print_person2(taro);
    print_person2(hanako);

    // Personに出力メソッドを実装
    let taro = person3(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = person3(String::from("Hanako"), String::from("hanako@flower"), 28);
    taro.print();
    hanako.print();

    // &selfのない関数
    let taro = person4(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = person4(String::from("Hanako"), String::from("hanako@flower"), 28);
    taro.print();
    hanako.print();
    println!("Person4's fields: {:?}", Person4::fields());

    println!("===== 3-3 トレイト =====");

    // トレイトを利用
    let taro = person5(String::from("Taro"), String::from("taro@yamada"), 39);
    let hanako = student5(String::from("Hanako"), String::from("hanako@flower"), 2);
    taro.print();
    hanako.print();

    // トレイトを関数で使う
    //print6(taro);
    //print6(hanako);

    // 借用を使う
    print7(&taro);
    print7(&hanako);

    // 戻り値でのトレイト指定
    let taro = person8("Taro", "taro@yamada", 39);
    let hanako = student8("Hanako", "hanako@flower", 28);
    print8(&taro);
    print8(&hanako);

    // 構造体の作成関数をBox戻り値にする
    let taro = person9("Taro", "taro@yamada", 39);
    let hanako = student9("Hanako", "hanako@flower", 28);
    print9(&taro);
    print9(&hanako);

    // 構造体をprintln!で出力させる
    let taro = person10("Taro", "taro@yamada", 39);
    let hanako = student10("Hanako", "hanako@flower", 28);
    println!("{:?}", taro);
    println!("{:?}", hanako);

    // トレイトのデフォルト実装
    let taro = person11("Taro", "taro@yamada", 39);
    let hanako = student11("Hanako", "hanako@flower", 28);
    taro.print();
    hanako.print();

    println!("===== 3-4 ジェネリクス =====");

    // ジェネリクスを使ったSample構造体

    let taro = Sample1 {
        name: String::from("Taro"),
        value: String::from("this is message."),
    };
    let hanako = Sample1 {
        name: String::from("Hanako"),
        value: 1234,
    };
    println!("{:?}", taro);
    println!("{:?}", hanako);

    // Sample構造体を作るsample関数を用意する
    let taro = sample2("Taro", "this is message.");
    let hanako = sample2("Hanako", 1234);
    println!("{:?}", taro);
    println!("{:?}", hanako);

    // SampleにVec<T>フィールドを追加する
    let taro = sample3("Taro", vec![123, 456, 789]);
    let hanako = sample3("Hanako", vec!["Hello", "Welcome", "Bye!"]);
    taro.print_values();
    hanako.print_values();
}

// 関数と所有権の移動
fn print_msg1(msg: String) {
    println!("Message is {}", msg);
}

// 戻り値で所有権を返す
fn print_msg2(msg: String) -> String {
    println!("Message is {}", msg);
    msg
}

// 参照
fn print_msg3(msg: &String) {
    println!("Message is \"{}\".", msg);
}

// シャドーイング
fn print_msg4(msg: &String) -> String {
    let msg = String::from("*** ") + msg + " ***";
    println!("[in func] Message is \"{}\"", msg);
    msg
}

// 参照した値の変更
fn print_msg5(msg: &mut String) {
    msg.push_str("!!!!");
    println!("[in func] Message is \"{}\".", msg);
}

// 構造体を使ってみる
struct Person1 {
    name: String,
    mail: String,
    age: i32,
}
fn print_person1(p: Person1) {
    println!("I'm {}({}). Mail to {}.", p.name, p.age, p.mail);
}

// 省略記法によるインスタンス作成
fn person1(name: String, mail: String, age: i32) -> Person1 {
    Person1 { name, mail, age }
}

// タプル構造体
struct Person2(String, String, i32);

fn print_person2(p: Person2) {
    println!("I'm {}({}). Mail to {}.", p.0, p.2, p.1);
}

// Personに出力メソッドを実装
struct Person3 {
    name: String,
    mail: String,
    age: i32,
}

fn person3(name: String, mail: String, age: i32) -> Person3 {
    Person3 { name, mail, age }
}

impl Person3 {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

// &selfのない関数
struct Person4 {
    name: String,
    mail: String,
    age: i32,
}

fn person4(name: String, mail: String, age: i32) -> Person4 {
    Person4 { name, mail, age }
}

impl Person4 {
    fn print(&self) {
        println!("{}<{}>({})", self.name, self.mail, self.age);
    }

    fn fields() -> [String; 3] {
        [
            String::from("name:String"),
            String::from("mail:String"),
            String::from("age:i32"),
        ]
    }
}

// トレイトを利用する
trait Print5 {
    fn print(&self);
}

struct Person5 {
    name: String,
    mail: String,
    age: i32,
}

impl Print5 for Person5 {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

fn person5(name: String, mail: String, age: i32) -> Person5 {
    Person5 { name, mail, age }
}

struct Student5 {
    name: String,
    mail: String,
    grade: i32,
}

impl Print5 for Student5 {
    fn print(&self) {
        println!("grade:{}: {}<{}>.", self.grade, self.name, self.mail);
    }
}

fn student5(name: String, mail: String, grade: i32) -> Student5 {
    Student5 { name, mail, grade }
}

// トレイトを関数で使う
//fn print6(ob:impl Print5) {
//    ob.print();
//}

// 借用を使う
fn print7(ob: &impl Print5) {
    ob.print();
}

// 戻り値でのトレイト指定
fn person8(name: &str, mail: &str, age: i32) -> impl Print5 {
    Person5 {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    }
}

fn student8(name: &str, mail: &str, grade: i32) -> impl Print5 {
    Student5 {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    }
}

fn print8(ob: &impl Print5) {
    ob.print();
}

// 構造体の作成関数をBox戻り値にする
fn person9(name: &str, mail: &str, age: i32) -> Box<dyn Print5> {
    Box::new(Person5 {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    })
}

fn student9(name: &str, mail: &str, grade: i32) -> Box<dyn Print5> {
    Box::new(Student5 {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    })
}

fn print9(ob: &Box<dyn Print5>) {
    ob.print();
}

// 構造体をprintln!で出力させる
#[derive(Debug)]
struct Person10 {
    name: String,
    mail: String,
    age: i32,
}

#[derive(Debug)]
struct Student10 {
    name: String,
    mail: String,
    grade: i32,
}

fn person10(name: &str, mail: &str, age: i32) -> Person10 {
    Person10 {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    }
}

fn student10(name: &str, mail: &str, grade: i32) -> Student10 {
    Student10 {
        name: String::from(name),
        mail: String::from(mail),
        grade: grade,
    }
}

// トレイトのデフォルト実装
trait Print11 {
    fn print(&self) {
        println!("PRINT is not yet implemented...");
    }
}

#[derive(Debug)]
struct Person11 {
    name: String,
    mail: String,
    age: i32,
}

#[derive(Debug)]
struct Student11 {
    name: String,
    mail: String,
    grade: i32,
}

impl Print11 for Person11 {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

impl Print11 for Student11 {}

fn person11(name: &str, mail: &str, age: i32) -> Person11 {
    Person11 {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    }
}

fn student11(name: &str, mail: &str, grade: i32) -> Student11 {
    Student11 {
        name: String::from(name),
        mail: String::from(mail),
        grade: grade,
    }
}

// ジェネリクスを使ったSample構造体
#[derive(Debug)]
struct Sample1<T> {
    name: String,
    value: T,
}

// Sample構造体を作るsample関数を用意する
fn sample2<T>(name: &str, value: T) -> Sample1<T> {
    Sample1 {
        name: String::from(name),
        value: value,
    }
}

// SampleにVec<T>フィールドを追加する
#[derive(Debug)]
struct Sample3<T: core::fmt::Debug> {
    name: String,
    values: Vec<T>,
}

impl<T: core::fmt::Debug> Sample3<T> {
    fn print_values(&self) {
        println!("*** {} ***", &self.name);
        for item in &self.values {
            println!("{:?}", item);
        }
    }
}

fn sample3<T: core::fmt::Debug>(name: &str, values: Vec<T>) -> Sample3<T> {
    Sample3 {
        name: String::from(name),
        values: values,
    }
}
