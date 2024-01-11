fn main() {
    // 23.1
    /*
    let ref_to_n;
    {
        let n = 12;
        ref_to_n = &n;
    }
    println!("{}", *ref_to_n); // compile error
    */

    /*
    let mut v = vec![12];
    let ref_to_first = &v[0];
    v.push(13); // compile error
    println!("{}", *ref_to_first);
    */

    // 23.2
    let n = 12;
    let _ref_to_n = &n;

    let a = 12;
    let mut b = &a;
    let c = &a;
    print!("{} {} ", b, c);
    b = &23;
    println!("{}", b);

    let mut n = 12;
    let _ref1_to_n = &mut n;
    let _ref2_to_n = &n;

    // 23.4
    /*
    let _n;
    let _r = &_n; // compile error
    _n = 12; // compile error
    */

    /*
    let _r;
    {
        let _n = 12;
        _r = &_n; // compile error
    }
    println!("{}", _r);
    */

    // 23.5
    let mut _n = 12;
    {
        let _ref_n = &_n;
        let _m = _n;
        let _k = *_ref_n;
    }
    _n = 13;
    _n += 1;

    // 23.6
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        result = {
            let _m1 = &n1;
            let _m2 = &n2;
            _m1
        }
    }
    println!("{}", *result);

    /*
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        result = {
            let _m1 = &n1;
            let _m2 = &n2; // compile error
            _m2
        }
    }
    println!("{}", *result);
    */

    /*
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        fn func(_m1: &i32, _m2: &i32) -> &i32 {
            _m1
        }
        result = func(&n1, &n2);
    }
    println!("{}", *result);
    */

    /*
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        fn func(_m1: &i32, _m2: &i32) -> &i32 {
            _m2
        }
        result = func(&n1, &n2);
    }
    println!("{}", *result);
    */

    // 23.7
    fn _func(_n: u32, _b: &bool) {
        let _s = "Hello".to_string();
    }

    /*
    fn func() -> &str {
        "Hello"
    }
    */

    fn _func2(v: &Vec<u8>) -> &u8 {
        &v[3]
    }

    /*
    trait Tr {
        fn f(flag: bool, b: &i32, c: (char, &i32))
        -> (&i32, f64, &i32);
    }
    */

    trait Tr {
        fn f<'a>(flag: bool, b: &'a i32, c: (char, &'a i32))
        -> (&'a i32, f64, &'static i32);
    }

    trait Tr2 {
        fn f<'a>(flag: bool, b: &'a i32, c: (char, &i32))
        -> (&'static i32, f64, &'a i32);
    }

    trait Tr3 {
        fn f<'a, 'b, T1, T2>(flag: bool, b: &'a T1, c: (char, &'b i32))
        -> (&'b i32, f64, &'a T2);
    }

    // 23.8
    static FOUR: u8 = 4;
    fn f() -> (bool, &'static u8, &'static str, &'static f64) {
        (true, &FOUR, "Hello", &3.14)
    }
    println!("{} {} {} {}", f().0, *f().1, f().2, *f().3);

    /*
    fn f2(n: &u8) -> &'static u8 { n }
    fn g2<'a>(m: &'a u8) -> &'static u8 { m }
    */

    fn f3<'a, 'b>(x: &'a i32, y: &'b i32) -> (&'b i32, bool, &'a i32) {
        (y, true, x)
    }
    let i = 12;
    let j = 13;
    let r = f3(&i, &j);
    println!("{} {} {}", *r.0, r.1, *r.2);

    /*
    fn f4<'a, 'b>(x: &'a i32, y: &'b i32) -> (&'b i32, bool, &'a i32) {
        (x, true, y)
    }
    let i = 12;
    let j = 13;
    let r = f4(&i, &j);
    println!("{} {} {}", *r.0, r.1, *r.2);
    */

    fn f5<'a>(x: &'a i32, y: &'a i32) -> (&'a i32, bool, &'a i32) {
        (x, true, y)
    }
    let i = 12;
    let j = 13;
    let r = f5(&i, &j);
    println!("{} {} {}", *r.0, r.1, *r.2);

    fn _f6<'a>(n: i32, x: &'a Vec<u8>, _y: &Vec<u8>) -> &'a u8 {
        if n == 0 { return &x[0]; }
        if n < 0 { &x[1] } else { &x[0] }
    }

    /*
    fn _f7<'a>(n: i32, x: &'a Vec<u8>, y: &Vec<u8>) -> &'a u8 {
        if n == 0 { return &x[0]; }
        if n < 0 { &x[1] } else { &y[2] }
    }
    */

    /*
    fn _f8<'a>(x: &'a Vec<u8>, y: &Vec<u8>) -> &'a u8 {
        if true { &x[0] } else { &y[0] }
    }
    */

    // 23.9
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        fn func<'a>(_m1: &'a i32, _m2: &i32) -> &'a i32 {
            _m1
        }
        result = func(&n1, &n2);
    }
    println!("{}", *result);

    /*
    let n1 = 11;
    let result;
    {
        let n2 = 12;
        fn func<'a>(_m1: &i32, _m2: &'a i32) -> &'a i32 {
            _m2
        }
        result = func(&n1, &n2);
    }
    println!("{}", *result);
    */

    // 23.10
    fn f9<'a, 'b>(x: &'a i32, y: &'b i32) -> (&'a i32, bool, &'b i32) {
        (x, true, y)
    }
    let i1 = 12;
    let i2;
    {
        let j1 = 13;
        let j2;
        let r = f9(&i1, &j1);
        i2 = r.0;
        j2 = r.2;
        print!("{} ", *j2);
    }
    println!("{}", *i2);

    /*
    fn f10<'a>(x: &'a i32, y: &'a i32) -> (&'a i32, bool, &'a i32) {
        (x, true, y)
    }
    let i1 = 12;
    let i2;
    {
        let j1 = 13;
        let j2;
        let r = f10(&i1, &j1);
        i2 = r.0;
        j2 = r.2;
        print!("{} ", *j2);
    }
    println!("{}", *i2);
    */
}