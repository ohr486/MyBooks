fn main() {
    // 3.1
    let number = 12;
    let other_number = 53;
    println!("{}", number + other_number);

    let number = 12;
    println!("{} {}", number, 47);

    // 3.2
    let mut number = 12;
    print!("{}", number);
    number = 53;
    println!(" {}", number);

    // 3.3
    // let mut number = 12; // compile error
    // println!("{}", number);

    // 3.4
    let number;
    number = 12;
    println!("{}", number);

    // let number; // compile error
    // println!("{}", number);

    let number1;
    let number2 = 22;
    number1 = number2;
    println!("{}", number1);

    // let number1;
    // println!("{}", number1); // compile error
    // number1 = 12;

    // 3.5
    // let number = 12; // compile error

    let _number = 12;

    let _ = 12;

    let _number = 12;
    println!("{}", _number);

    // let _ = 12;
    // println!("{}", _); // compile error

    // 3.6
    let truth = true;
    let falsity = false;
    println!("{} {}", truth, falsity);

    let truth = 5 > 2;
    let falsity = -12.3 >= 10.;
    println!("{} {} {}", truth, falsity, -50 < 6);

    println!("{} {} {}", "abc" < "abcd", "ab" < "ac", "A" < "a");

    // 3.7
    let truth = true;
    let falsity = false;
    println!("{} {}", ! truth, ! falsity);
    println!("{} {}  {} {}", falsity && falsity, falsity && truth,
        truth && falsity, truth && falsity);
    println!("{} {}  {} {}", falsity || falsity, falsity || truth,
             truth || falsity, truth || falsity);

    println!("{}", true || true && !true);
    println!("{}", (true || true) && !true);

    // 3.8
    let mut n = 1;
    print!("{}", n);
    n = 2;
    print!(" {}", n);
    n = 3;
    // n = 3.14; // compile error
    println!(" {}", n);

    // 3.9
    // let number; // compile error

    let number1 = 12;
    let _number2 = number1;

    let mut n = 1.;
    print!("{}", n);
    n = 2.;
    print!(" {}", n);
    n = 3.14;
    println!(" {}", n);

    // 3.10
    let mut n = 1;
    print!("{}", n);
    n = 2;
    print!(" {}", n);
    let n = 3.14;
    println!(" {}", n);

    let mut _n = 1;
    _n = 2;
    let _n = 3.14;
    // _n = 5.9; // compile error

    let x = 120; print!("{} ", x);
    let x = "abcd"; print!("{} ", x);
    let mut x = true; print!("{} ", x);
    x = false; println!("{}", x);

    // 3.11
    let mut a = 12;
    a = a + 1;
    a = a - 4;
    a = a * 7;
    a = a / 6;
    println!("{}", a);

    let mut a = 12;
    a += 1;
    a -= 4;
    a *= 7;
    a /= 6;
    println!("{}", a);

    // 3.12
    println!("{} {}", str::len("abcde"), "abcde".len());
}