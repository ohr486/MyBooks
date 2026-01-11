use rand::Rng;

fn main() {
    println!("===== 4-1 Noneとエラー処理 =====");

    // OptionとNone
    let mut data = vec![];
    for n in 0..5 {
        data.push(Some(n));
    }
    print_all1(data);

    let mut data = vec![];
    for _ in 0..10 {
        data.push(random1());
    }
    print_all2(data);

    // panicによる強制終了
    let mut data = vec![];
    for _ in 0..10 {
        data.push(random1());
    }
    //print_all3(data);

    // Resultによるエラーリカバリ
    let mut data = vec![];
    for _ in 0..10 {
        data.push(random1());
    }
    print_all4(data);

    // より細かなエラー処理
    let mut data = vec![];
    for _ in 0..10 {
        data.push(random1());
    }
    // print_all5(data);
}

// OptionとNone

fn print_all1(data: Vec<Option<i32>>) {
    for item in data {
        println!("{:?}", item);
    }
}

fn random1() -> Option<i32> {
    let n = rand::thread_rng().gen_range(0..10);
    match n {
        0 => None,
        _ => Some(n)
    }
}

fn print_all2(data: Vec<Option<i32>>) {
    for item in data {
        print2(item);
    }
}

fn print2(item: Option<i32>) {
    match item {
        None => println!("no-data..."),
        Some(n) => println!("No, {}.", n)
    }
}

// panicによる強制終了

fn print3(item: Option<i32>) {
    match item {
        None => panic!("NODATA!!"),
        Some(n) => println!("No, {}.", n)
    }
}

fn print_all3(data: Vec<Option<i32>>) {
    for item in data {
        print3(item);
    }
}

// Resultによるエラーリカバリ

fn print4(item: Option<i32>) -> Result<String, String> {
    match item {
        None => {
            Err(String::from("ERROR IS OCCURED."))
        },
        Some(n) => {
            println!("No, {}.", n);
            Ok(String::from("OK"))
        }
    }
}

fn print_all4(data: Vec<Option<i32>>) {
    for item in data {
        let res = print4(item);
        match res {
            Ok(s) => println!("--- {} ---", s),
            Err(s) => println!("*** {} ***", s)
        }
    }
}

// より細かなエラー処理

enum ErrKind5 {
    Caution,
    Danger
}

fn print_all5(data: Vec<Option<i32>>) {
    for item in data {
        let res = print5(item);
        match res {
            Ok(s) => println!("--- {} ---", s),
            Err(k) => match k {
                ErrKind5::Caution => {
                    println!("*** CAUTION! ***");
                },
                ErrKind5::Danger => {
                    println!("*** DANGER! ***");
                    panic!("DANGER ERROR.");
                }
            }
        }
    }
}

fn print5(item: Option<i32>) -> Result<String, ErrKind5> {
    match item {
        None => {
            Err(ErrKind5::Danger)
        },
        Some(n) => {
            println!("No, {}.", n);
            if n == 1 {
                Err(ErrKind5::Caution)
            } else {
                Ok(String::from("OK"))
            }
        }
    }
}
