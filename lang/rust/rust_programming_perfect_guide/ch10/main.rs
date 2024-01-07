// 10.5
fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }

fn swap_i16_u16(a: i16, b: u16) -> (u16, i16) { (b, a) }
fn swap_f32_bool(a: f32, b: bool) -> (bool, f32) { (b, a) }

fn main() {
    // 10.1
    fn f1(ch: char, num1: i16, num2: i16) -> i16 {
        if ch == 'a' { num1 }
        else { num2 }
    }
    println!("{}", f1('a', 37, 41));

    println!("{}", f1('a', 37.2 as i16, 41. as i16));

    // 10.2
    fn f2<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' { num1 }
        else { num2 }
    }
    let a: i16 = f2::<i16>('a', 37, 41);
    let b: f64 = f2::<f64>('b', 37.2, 41.1);
    println!("{} {}", a, b);

    // 10.3
    fn f3<T>(ch: char, num1: T, num2: T) -> T {
        if ch == 'a' { num1 }
        else { num2 }
    }
    let a: i16 = f3('a', 37, 41);
    let b: f64 = f3('b', 37.2, 41.1);
    println!("{} {}", a, b);

    fn f4<T>(a: T, _b: T) -> T { a }
    let _a = f4(12u8, 13u8);
    let _b = f4(12i64, 13i64);
    // let _c = f4(12i16, 13u16); // compile error
    // let _d: i32 = f4(12i16, 13i16); // error

    fn f5<Param1, Param2>(_a: Param1, _b: Param2) {}
    f5('a', true);
    f5(12.56, "Hello");
    f5((3, 'a'), [5, 6, 7]);

    // 10.4
    #[allow(dead_code)]
    struct S1<T1, T2> {
        c: char,
        n1: T1,
        n2: T1,
        n3: T2,
    }
    let _s1 = S1 { c: 'a', n1: 34, n2: 782, n3: 0.02 };
    struct SE1<T1, T2> (char, T1, T1, T2);
    let _se1 = SE1 ('a', 34, 782, 0.02);

    #[allow(dead_code)]
    struct S2<T1, T2> {
        c: char,
        n1: T1,
        n2: T2,
    }
    let _s2 = S2::<u16, f32> { c: 'a', n1: 34, n2: 0.02 };
    struct SE2<T1, T2> (char, T1, T2);
    let _se2 = SE2::<u16, f32> ('a', 34, 0.02);

    // 10.5
    let x = swap(3i16, 4u16);
    let y = swap(5f32, true);
    println!("{:?} {:?}", x, y);

    let x = swap_i16_u16(3i16, 4u16);
    let y = swap_f32_bool(5f32, true);
    println!("{:?} {:?}", x, y);

    fn swap2<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
    let x = swap2('A', 4.5);
    let y = swap2('g', -6.);
    println!("{:?} {:?}", x, y);

    fn swap2_char_f64(a: char, b: f64) -> (f64, char) { (b, a) }
    let x = swap2_char_f64('A', 4.5);
    let y = swap2_char_f64('g', -6.);
    println!("{:?} {:?}", x, y);

    // 10.7
    enum Result1<SuccessCode, FailureCode> {
        Success(SuccessCode),
        Failure(FailureCode, char),
        Uncertainty,
    }
    let mut _res = Result1::Success::<u32, u16>(12u32);
    _res = Result1::Uncertainty;
    _res = Result1::Failure(0u16, 'd');

    #[allow(dead_code)]
    enum Result2<SuccessCode, FailureCode> {
        Success(SuccessCode),
        Failure(FailureCode, char),
        Uncertainty,
    }
    let mut _res = Result2::Success::<u32, u16>(12u32);
    _res = Result2::Uncertainty;
    // _res = Result2::Failure(0u32, 'd'); // compile error

    // 10.8
    let mut v = vec![11, 22, 33];
    for _ in 0..5 {
        let item: Option<i32> = v.pop();
        match item {
            Some(number) => print!("{}, ", number),
            None => print!("#, "),
        }
    }
    println!();

    // 10.9
    fn divide1(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0. {
            Err(format!("Divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }
    println!("{:?}, {:?}",
             divide1(8., 2.),
             divide1(8., 0.));

    fn divide2(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0. {
            Err(format!("Divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }
    fn show_divide(num: f64, den: f64) {
        match divide2(num, den) {
            Ok(val) => println!("{} / {} = {}", num, den, val),
            Err(msg) => println!("Cannot divide {} by {}: {}", num, den, msg),
        }
    }
    show_divide(8., 2.);
    show_divide(8., 0.);

    // 10.10
    fn divide3(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0. {
            Err(format!("Divide by zero"))
        } else {
            Ok(numerator / denominator)
        }
    }
    let r1 = divide3(8., 2.);
    let r2 = divide3(8., 0.);
    println!("{} {}", r1.is_ok(), r1.is_err());
    println!("{} {}", r2.is_ok(), r2.is_err());
    println!("{}", r1.unwrap());
    // println!("{}", r2.unwrap()); // error

    let mut a = Some(12);
    print!("{} {}; ", a.is_some(), a.is_none());
    a = None;
    println!("{} {}", a.is_some(), a.is_none());

    let mut v = vec![11, 22, 33];
    for _ in 0..v.len() {
        print!("{} ", v.pop().unwrap());
    }
    println!();
}