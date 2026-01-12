use mymodule::add;
use mymodule::calc;

fn _main1() {
    println!("===== クレートを利用する =====");
    let x = 10;
    let y = 20;
    let res = add(x, y);

    println!("answer: {} + {} = {}", x, y, res);
}

fn _main2() {
    println!("===== モジュールを定義する =====");
    let x = 10;
    let y = 20;
    let res = calc::add(x, y);

    println!("answer: {} + {} = {}", x, y, res);
}

fn main() {
    println!("===== テストを試す =====");
    let x = 123;
    let res = calc::is_prime(x);

    println!("answer: {} = {}", x, res);
}
