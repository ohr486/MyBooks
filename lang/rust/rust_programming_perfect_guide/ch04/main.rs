fn main() {
    // 4.1
    let n = 4;
    if n > 0 { println!("positive"); }

    let n = 0;
    print!("number is ");
    if n > 0 {
        println!("positive");
    } else {
        println!("not positive");
    }

    let n = 4;
    if n > 1000 {
        println!("big");
    } else {
        if n > 0 {
            println!("small");
        } else {
            if n < 0 {
                println!("negative");
            }
            else {
                println!("neither positive nor negative");
            }
        }
    }

    let n = 4;
    if n > 1000 {
        println!("big");
    }
    else if n > 0 {
        println!("small");
    }
    else if n < 0 {
        println!("negative");
    }
    else {
        println!("neither positive nor negative");
    }

    // 4.2
    let n = 4;
    println!("{}",
        if n > 1000 {
            "big"
        }
        else if n > 0 {
            "small"
        }
        else if n < 0 {
            "negative"
        }
        else {
            "neither positive nor negative"
        }
    );

    // let val = if cond { "abc" }; // compile error

    // let val = if cond { "abc" } else { 12 }; // compile error

    let _a = if true { "abc" } else { "xy" };
    let _b = if true { 3456 } else { 12 };
    let _c = if true { 56.9 } else { 12. };

    // 4.3
    let mut n = 1;
    while n <= 10 {
        print!("{} ", n * n);
        n += 1;
    }
    println!();

    let mut n = 0;
    while n < 50 {
        n += 1;
        if n % 3 != 0 {
            if n * n <= 400 {
                print!("{} ", n * n);
            }
        }
    }
    println!();

    let mut n = 0;
    while n < 50 {
        n += 1;
        if n % 3 == 0 { continue; }
        if n * n > 400 { break; }
        print!("{} ", n * n);
    }
    println!();

    // 4.4
    // let mut n = 1;
    // while true { // compile warning
    //     let n2 = n * n;
    //     if n2 > 200 { break; }
    //     print!("{} ", n2);
    //     n += 1;
    // }
    // println!();

    let mut n = 1;
    loop {
        let nn = n * n;
        if nn > 200 { break; }
        print!("{} ", nn);
        n += 1;
    }
    println!();

    // 4.5
    for n in 1..11 {
        print!("{} ", n * n);
    }
    println!();

    for n in 1..=10 {
        print!("{} ", n * n);
    }
    println!();

    let index = 8;
    for index in 0..4 { print!("{} ", index); }
    println!(":{}", index);

    let mut limit = 4;
    for n in 1..limit + 2 {
        limit -= 1;
        println!("limit={} n={} ", limit, n);
    }
    println!("limit: {}", limit);

    // 4.6
    print!("1");
    {
        print!("2");
        print!("3");
        {
            print!("4");
            {
                print!("5");
                { { } };
                print!("6");
            }
        }
        print!("7");
    }
    println!();

    // { let n = 10; }
    // print!("{} ", n); // compile error

    {
        let n = 10;
        {
            let m = 4;
            {
                print!("{} ", n);
            }
            print!("{} ", n + m);
        } // mのスコープが終わる
    } // nのスコープが終わる
    println!();

    {
        let n = 10;
        {
            let n = 4;
            print!("{} ", n);
        } // 第2のnのスコープが終わる
        print!("{} ", n);
    } // 第1のnのスコープが終わる
    println!();

    let mut _i = 1;
    if true { let _i = 2; }
    print!("{} ", _i);
    while _i > 0 { _i -= 1; let _i = 5; }
    println!("{} ", _i)
}