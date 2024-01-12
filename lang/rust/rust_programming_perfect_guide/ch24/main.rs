// 24.3
/*
struct S3 {
    b: bool,
    ri: & i32,
}
fn create_s3(ri: &i32) -> S3 {
    S3 { b: true, ri: ri }
}
*/

struct S4 { b: bool, ri: &'static i32 }
fn create_s4(ri: &i32) -> S4 {
    static ZERO: i32 = 0;
    static ONE: i32 = 1;
    S4 {
        b: true,
        ri: if *ri > 0 { &ONE } else { &ZERO },
    }
}

// 24.4
struct S5<'a> { b: bool, ri: &'a i32 }
fn create_s5<'b>(ri: &'b i32) -> S5<'b> {
    S5 { b: true, ri: ri }
}

fn main() {
    // 24.1
    trait Tr1 {
        fn f<'a>(x: &'a u8, b: bool) -> &'a u8;
    }

    trait Tr2 {
        fn f(x: &u8, b: bool) -> &u8;
    }

    trait Tr3 {
        fn f(b: bool, x: (u32, &u8)) -> &u8;
    }

    trait Tr4 {
        fn f(x: &u8) -> (&u8, &f64, bool, &Vec<String>);
    }

    trait Tr5 {
        fn f<'a>(x: &'a u8) -> (&u8, &'a f64, bool, &'static Vec<String>);
    }

    // 24.2
    trait Tr6 {
        fn f(&self, y: &u8)
        -> (&u8, &f64, bool, &Vec<String>);
    }

    trait Tr7 {
        fn f<'a>(&'a self, y: &u8)
        -> (&'a u8, &'a f64, bool, &'a Vec<String>);
    }

    trait Tr8 {
        fn f<'a>(&self, y: &'a u8)
        -> (&u8, &'a f64, bool, Vec<String>);
    }

    // 24.3
    let x: i32 = 12;
    let y: & i32 = &x;
    println!("{}", *y);

    /*
    let y: &i32;
    {
        let x: i32 = 12;
        y = &x;
    }
    println!("{}", *y);
    */

    /*
    struct S1 {
        b: bool,
        ri: & i32,
    }
    let x: i32 = 12;
    let y: S1 = S1 { b: true, ri: &x };
    println!("{} {}", y.b, *y.ri);
    */

    /*
    struct S2 {
        b: bool,
        ri: & i32,
    }
    let y: S2;
    {
        let x: i32 = 12;
        y = S2 { b: true, ri: &x };
    }
    println!("{} {}", y.b, *y.ri);
    */

    /*
    let y: S3;
    {
        let x: i32 = 12;
        y = create_s3(&x);
    }
    println!("{} {}", y.b, *y.ri);
    */

    let y: S4;
    {
        let x: i32 = 12;
        y = create_s4(&x);
    }
    println!("{} {}", y.b, *y.ri);

    // 24.4
    let x: i32 = 12;
    let y: S5;
    y = create_s5(&x);
    println!("{} {}", y.b, *y.ri);

    /*
    let y: S5;
    {
        let x: i32 = 12;
        y = create_s5(&x);
    }
    println!("{} {}", y.b, *y.ri);
    */

    // struct _S1 { _f: &i32 } // compile error
    // struct _S2<'a> { _f: &i32 } // compile error
    // struct _S3 { _f: &'a i32 } // compile error
    // struct _S4<'a> { _f: &'static i32 } // compile error
    struct _S5 { _f: &'static i32 }
    struct _S6<'a> { _f: &'a i32 }

    // 24.5
    struct TS<'a> (&'a u8);
    enum E<'a, 'b> {
        _A(&'a u8),
        _B,
        _C(bool, &'b f64, char),
        _D(&'static str),
    }
    let byte = 34;
    let _ts = TS(&byte);
    let _e = E::_A(&byte);

    fn f<'a>(b: &'a mut u8) -> &'a u8 {
        *b += 1;
        b
    }
    let mut byte = 12u8;
    let byte_ref = f(&mut byte);
    println!("{}", *byte_ref);
}