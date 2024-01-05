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

}