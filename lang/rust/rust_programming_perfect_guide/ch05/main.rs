// #[allow(unused_variables)]
fn main() {
    // 5.1
    let x = ["English", "This", "sentence", "a", "in", "is"];
    println!("{} {} {} {} {} {}",
        x[1], x[5], x[3], x[2], x[4], x[0]);

    let a = [true, false];
    let b = [1, 2, 3, 4, 5];
    println!("{}, {}.", a.len(), b.len());

    // let x = ["This", 4]; // compile error

    // let x = [4, 5.]; // compile error

    // compile error
    // let mut x = ["a"];
    // x[0] = 3;
    // x[-1] = "b";
    // x[0.] = "b";
    // x[false] = "b";
    // x["0"] = "b";

    // let x = ["a"];
    // let _y = x[1]; // compile error

    // 5.2
    // #[deny(unused_variables)] // compile error
    // let x = 1;
    // #[warn(unused_variables)] // compile warning
    // let y = 2;
    #[allow(unused_variables)]
    let z = 3;

    // 5.3
    // let x = ["a"];
    // let _y = x[1]; // compile error
    // print!("End");

    // let x = ["a"];
    // #[warn(unconditional_panic)]
    // let _y = x[1]; // panic
    // print!("End");

    // let x = ["a"];
    // #[allow(unconditional_panic)]
    // let _y = x[1]; // panic
    // print!("End");

    // 5.4
    let mut x = ["This", "is", "a", "sentence"];
    x[2] = "a nice";
    println!("{} {} {} {}",
        x[0], x[1], x[2], x[3]);

    let mut x = ["a", "b", "c"];
    print!("{}{}{}. ", x[0], x[1], x[2]);
    x = ["X", "Y", "Z"];
    print!("{}{}{}. ", x[0], x[1], x[2]);
    let y = ["1", "2", "3"];
    x = y;
    println!("{}{}{}. ", x[0], x[1], x[2]);

    // let mut x = ["a", "b", "c"];
    // x = ["X", "Y"]; // compile error
    // x = [15, 16, 17]; // compile error

    // 5.5
    let mut x = [4.; 5000];
    x[2000] = 3.14;
    println!("{}, {}", x[1000], x[2000]);

    let mut fib = [1; 12];
    for i in 2..fib.len() {
        fib[i] = fib[i - 2] + fib[i - 1];
    }
    for i in 0..fib.len() {
        print!{"{}, ", fib[i]};
    }
    println!();

    // 5.6
    let mut x = [[[23; 4]; 8]; 15];
    x[14][7][3] = 56;
    println!("{}, {}", x[0][0][0], x[14][7][3]);

    let x = [[[0; 4]; 8]; 15];
    println!("{}, {}, {}.", x.len(), x[0].len(), x[0][0].len());

    // let length = 6;
    // let arr = [0; length]; // compile error

    // 5.7
    let x = vec!["This", "is"];
    println!("{} {}. Length: {}.", x[0], x[1], x.len());

    let mut x = vec!["This", "is"]; print!("{} ", x.len());
    x.push("a"); print!("{} ", x.len());
    x.push("sentence"); print!("{} ", x.len());
    x[0] = "That";
    for i in 0..x.len() { print!("{} ", x[i]); }
    println!();

    let length = 5000;
    let mut y = vec![4.; length];
    y[6] = 3.14;
    y.push(4.89);
    println!("{}, {}, {}", y[6], y[4999], y[5000]);

    let mut _x = vec!["a", "b", "c"];
    _x = vec!["X", "Y"];

    // let mut _y = vec!["a", "b", "c"];
    // _x = vec![15, 16, 17]; // compile error

    // 5.8
    let mut x = vec!["This", "is", "a", "sentence"];
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.insert(1, "line");
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.insert(2, "contains");
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.remove(3);
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.push("about Rust");
    for i in 0..x.len() { print!("{} ", x[i]); } println!();
    x.pop();
    for i in 0..x.len() { print!("{} ", x[i]); } println!();

    // let mut x = vec!["This", "is", "a", "sentence"];
    // x.insert("line", 1); // compile error

    let mut _x = vec![12, 13, 14, 15];
    _x.insert(3, 1);

    // 5.9
    // f(["help", "debug"], vec![0, 4, 15]);
    // f([], vec![]);

    // let _a = []; // compile error

    let _a = [""; 0];

    let _a = vec![true; 0];
    let _b = vec![false; 0];

    // f([""; 0], vec![0; 0]);

    // 5.10
    // println!("{} {}", [1, 2, 3], vec![4, 5]); // compile error

    println!("{:?} {:?}", [1, 2, 3], vec![4, 5]);

    // 5.11
    let mut a1 = [4, 56, -2];
    let a2 = [7, 81, 12500];
    println!("{:?} {:?}", a1, a2);
    a1 = a2;
    println!("{:?} {:?}", a1, a2);
    a1[1] = 10;
    println!("{:?} {:?}", a1, a2);

    let mut a1 = vec![4, 56, -2];
    let a2 = vec![7, 81, 12500];
    println!("{:?} {:?}", a1, a2);
    a1 = a2;
    println!("{:?}", a1);
    a1[1] = 10;
    println!("{:?}", a1);
    // println!("{:?} {:?}", a1, a2); // compile error
}