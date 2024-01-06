fn f1() { print!("1"); }

#[allow(dead_code)]
fn f7() { print!("1"); }

fn main() {
    // 9.1
    fn line() {
        println!("----------");
    }
    line();
    line();
    line();

    f1();
    fn f2() { print!("2"); }
    f2();
    f1();
    f2();
    println!();

    // 9.2
    // a; // compile error
    // let a = 3;

    f3();
    fn f3() {}

    // 9.3
    // fn f4() {} // compile error
    // fn f3() {} // compile error

    {
        fn f5() { print!("a"); }
        f5();
        f5();
    }
    {
        fn f5() { print!("b"); }
        f5();
    }
    println!();

    // {
    //     fn f6() { }
    // }
    // f6(); // compile error

    f7(); // 2とプリント
    {
        f7(); // 3とプリント
    fn f7() { print!("3"); }
    }
    f7(); // 2とプリント
    fn f7() { print!("2"); }
    println!();

    // 9.4
    fn print_sum(addend1: f64, addend2: f64) {
        println!("{} + {} = {}", addend1, addend2, addend1 + addend2);
    }
    print_sum(3., 5.);
    print_sum(3.2, 5.1);

    {
        let addend1: f64 = 3.;
        let addend2: f64 = 5.;
        println!("{} + {} = {}", addend1, addend2, addend1 + addend2);
    }
    {
        let addend1: f64 = 3.2;
        let addend2: f64 = 5.1;
        println!("{} + {} = {}", addend1, addend2, addend1 + addend2);
    }

    // fn f8(a: i16) {}
    // f8(3.); // compile error
    // f8(3u16); // compile error
    // f8(3i16);
    // f8(3);

    // 9.5
    fn print_double(mut x: f64) {
        x *= 2.;
        print!("{}", x);
    }
    let x = 4.;
    print_double(x);
    println!(" {}", x);

    // 9.6
    fn double(x: f64) -> f64 { x * 2. }
    println!("{}", double(17.3));

    fn _f9(_x: i32) {}
    fn _f10(_x: i32) -> () {}

    fn _f11() -> i32 {
        4.5;
        "abc";
        73i32
    }
    fn _f12() -> i32 {
        4.5;
        "abc";
        73
    }
    fn _f13() -> i32 {
        4.5;
        "abc";
        73 + 100
    }

    // fn _f14() -> i32 { 4.5; "abc"; false } // compile error
    // fn _f15() -> i32 { 4.5; "abc"; () } // compile error
    // fn _f16() -> i32 { 4.5; "abc"; {} } // compile error
    // fn _f17() -> i32 { 4.5; "abc"; } // compile error

    // 9.7
    fn f18(x: f64) -> f64 {
        if x <= 0. { return 0.; }
        x + 3.
    }
    println!("{} {}", f18(1.), f18(-1.));

    fn f19(x: f64) -> f64 {
        if x <= 0. { return 0.; }
        return x + 3.;
    }
    println!("{} {}", f19(1.), f19(-1.));

    fn f20(x: f64) -> f64 {
        if x <= 0. { 0. } else { x + 3. }
    }
    println!("{} {}", f20(1.), f20(-1.));

    fn f21(x: i32) {
        if x <= 0 { return; }
        if x == 4 { return (); }
        if x == 7 { return {}; }
        println!("{}", x);
    }
    f21(5);

    fn f22() -> i32 { 3 }
    f22();

    fn f23() -> i32 { 3 }
    let _a: i32 = f23();

    // fn f24() -> i32 { 3 }
    // let _a: u32 = f24(); // compile error

    // 9.8
    fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
        (dividend / divisor, dividend % divisor)
    }
    println!("{:?}", divide(50, 11));

    #[allow(dead_code)]
    enum E { E1, E2 }
    #[allow(dead_code)]
    struct S {
        a: i32,
        b: bool
    }
    struct TS(f64, char);
    fn f25() -> E { E::E2 }
    fn f26() -> S { S { a: 49, b: true } }
    fn f27() -> TS { TS(4.7, 'w') }
    fn f28() -> [i16; 4] { [7, -2, 0, 19] }
    fn f29() -> Vec<i64> { vec![12000] }
    print!("{} ", match f25() { E::E1 => 1, _ => -1 });
    print!("{} ", f26().a );
    print!("{} ", f27().0 );
    print!("{} ", f28()[0] );
    print!("{} ", f29()[0] );
    println!();

    // 9.9
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    for n in 0..10 {
        arr[n] *= 2;
    }
    println!("{:?}", arr);

    fn double2(mut a: [i32; 10]) {
        for i in 0..10 {
            a[i] *= 2;
        }
    }
    let arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double2(arr);
    println!("{:?}", arr);

    fn double3(mut a: [i32; 10]) -> [i32; 10] {
        for i in 0..10 {
            a[i] *= 2;
        }
        a
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    arr = double3(arr);
    println!("{:?}", arr);

    // 9.10
    fn double4(a: &mut [i32; 10]) {
        for n in 0..10 {
            (*a)[n] *= 2;
        }
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double4(&mut arr);
    println!("{:?}", arr);

    fn double5(a: &mut [i32; 10]) {
        for n in 0..10 {
            a[n] *= 2;
        }
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double5(&mut arr);
    println!("{:?}", arr);

    fn double6(a: &mut [i32; 10]) {
        use std::ops::IndexMut;
        use std::ops::MulAssign;
        for n in 0..10 {
            (*(*a).index_mut(n)).mul_assign(2);
        }
    }
    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    double6(&mut arr);
    println!("{:?}", arr);

    // 9.11
    let a = 15;
    let ref_a = &a;
    println!("{} {} {}", a, *ref_a, ref_a);

    let a = &&&7;
    println!("{} {} {} {}", ***a, **a, *a, a);

    // 9.12
    let mut a: i32 = 10;
    let mut b: i32 = 20;
    let mut p: &mut i32 = &mut a;
    print!("{} ", *p);
    *p += 1;
    print!("{} ", *p);
    p = &mut b;
    print!("{} ", *p);
    *p += 1;
    print!("{} ", *p);
    println!();
}