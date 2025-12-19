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
        name:String::from("Taro"),
        mail:String::from("taro@yamada"),
        age:39
    };
    let hanako = Person1 {
        name:String::from("Hanako"),
        mail:String::from("hanako@flower"),
        age:28
    };
    print_person1(taro);
    print_person1(hanako);

    // 省略記法によるインスタンス作成
    let taro = person1(
        String::from("Taro"),
        String::from("taro@yamada"),
        39
    );
    let hanako = person1(
        String::from("Hanako"),
        String::from("hanako@flower"),
        39
    );
    print_person1(taro);
    print_person1(hanako);

    // タプル構造体
    let taro = Person2(
        String::from("Taro"),
        String::from("taro@yamada"),
        39
    );
    let hanako = Person2(
        String::from("Hanako"),
        String::from("hanako@flower"),
        28
    );
    print_person2(taro);
    print_person2(hanako);

    // Personに出力メソッドを実装
    let taro = person3(
        String::from("Taro"),
        String::from("taro@yamada"),
        39
    );
    let hanako = person3(
        String::from("Hanako"),
        String::from("hanako@flower"),
        28
    );
    taro.print();
    hanako.print();

    // &selfのない関数
    let taro = person4(
        String::from("Taro"),
        String::from("taro@yamada"),
        39
    );
    let hanako = person4(
        String::from("Hanako"),
        String::from("hanako@flower"),
        28
    );
    taro.print();
    hanako.print();
    println!("Person4's fields: {:?}", Person4::fields());
    
    println!("===== 3-3 トレイト =====");





}

// 関数と所有権の移動
fn print_msg1(msg:String) {
    println!("Message is {}", msg);
}

// 戻り値で所有権を返す
fn print_msg2(msg:String) -> String {
    println!("Message is {}", msg);
    msg
}

// 参照
fn print_msg3(msg:&String) {
    println!("Message is \"{}\".", msg);
}

// シャドーイング
fn print_msg4(msg:&String) -> String {
    let msg = String::from("*** ") + msg + " ***";
    println!("[in func] Message is \"{}\"", msg);
    msg
}

// 参照した値の変更
fn print_msg5(msg:&mut String) {
    msg.push_str("!!!!");
    println!("[in func] Message is \"{}\".", msg);
}

// 構造体を使ってみる
struct Person1 {
    name:String,
    mail:String,
    age:i32
}
fn print_person1(p:Person1) {
    println!("I'm {}({}). Mail to {}.", p.name, p.age, p.mail);
}

// 省略記法によるインスタンス作成
fn person1(name:String, mail:String, age:i32) -> Person1 {
    Person1 {name, mail, age}
}

// タプル構造体
struct Person2(String, String, i32);

fn print_person2(p:Person2) {
    println!("I'm {}({}). Mail to {}.", p.0, p.2, p.1);
}

// Personに出力メソッドを実装
struct Person3 {
    name:String,
    mail:String,
    age:i32
}

fn person3(name:String, mail:String, age:i32) -> Person3 {
    Person3 {name, mail, age}
}

impl Person3 {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

// &selfのない関数
struct Person4 {
    name:String,
    mail:String,
    age:i32
}

fn person4(name:String, mail:String, age:i32) -> Person4 {
    Person4 {name, mail, age}
}

impl Person4 {
    fn print(&self) {
        println!("{}<{}>({})", self.name, self.mail, self.age);
    }

    fn fields() -> [String; 3] {
        [
            String::from("name:String"),
            String::from("mail:String"),
            String::from("age:i32")
        ]
    }
}





