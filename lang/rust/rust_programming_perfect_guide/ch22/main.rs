fn main() {
    // 22.2
    let _num = Box::new(3);

    let a = 3;
    {
        let _a_ref = &a;
    }
    println!("{}", a);

    // 22.3
    let v1 = vec![11, 22, 33];
    #[allow(unused_variables)]
    let v2 = v1;

    let v1 = vec![11, 22, 33];
    #[allow(unused_variables)]
    let v2 = v1;
    // println!("{}", v1.len()); // compile error

    let v1 = vec![11, 22, 33];
    let v2 = v1.clone();
    let v3 = v1;
    // print!("{} ", v1.len()); // compile error
    println!("{} {}", v2.len(), v3.len());

    // 22.5
    let s1 = "abcd".to_string();
    let s2 = s1.clone();
    let s3 = s1;
    // print!("{} ", s1.len()); // compile error
    println!("{} {}", s2.len(), s3.len());

    let i1 = Box::new(12345i16);
    let i2 = i1.clone();
    let i3 = i1;
    // print!("{} ", *i1); // compile error
    println!("{} {}", *i2, *i3);

    let v1 = vec![false; 3];
    let mut _v2 = vec![false; 2];
    _v2 = v1;
    // v1; // compile error

    fn f(_v2: Vec<bool>) {}
    let v1 = vec![false; 3];
    f(v1);
    // v1; // compile error

    let v1 = vec![false; 0];
    let mut _v2 = vec![false; 0];
    _v2 = v1;
    // v1; // compile error

    struct S {}
    let s1 = S {};
    let _s2 = s1;
    // s1; // compile error

    // 22.6
    let i1 = 123;
    let _i2 = i1;
    let s1 = "abc";
    let _s2 = s1;
    let r1 = &i1;
    let _r2 = r1;
    println!("{} {} {}", i1, s1, r1);

    // 22.7
    let a1 = 123;
    let b1 = a1.clone();
    let c1 = b1;
    print!("{} {} {}", a1, b1, c1);
    let a2 = Vec::<bool>::new();
    let b2 = a2.clone();
    let c2 = b2;
    print!(" {:?}", a2);
    // print!(" {:?}", b2); // compile error
    print!(" {:?}", c2);
    let a3 = std::fs::File::open(".").unwrap();
    // let b3 = a3.clone(); // compile error
    let c3 = a3;
    // print!(" {:?}", a3); // compile error
    println!(" {:?}", c3);

    // 22.8
    struct S1 {}
    let _s = S1 {};
    // let _ = _s.clone(); // compile error

    struct S2 {}
    impl Clone for S2 {
        fn clone(&self) -> Self { Self {} }
    }
    let s = S2 {};
    let _ = s.clone();

    struct S3 {}
    impl Clone for S3 {
        fn clone(&self) -> Self { Self {} }
    }
    let s = S3 {};
    let _ = s.clone();
    let _s2 = s;
    // let _s3 = s; // compile error

    struct S4 {}
    impl Clone for S4 {
        fn clone(&self) -> Self { Self {} }
    }
    impl Copy for S4 {}
    let s = S4 {};
    let _ = s.clone();
    let _s2 = s;
    let _s3 = s;

    /*
    struct S5 {}
    impl Copy for S5 {} // compile error
    */

    /*
    struct S6 { x: Vec<i32> }
    impl Copy for S6 {} // compile error
    impl Clone for S6 {
        fn clone(&self) -> Self { *self }
    }
    */

    struct S7 { x: Vec<i32> }
    impl Clone for S7 {
        fn clone(&self) -> Self {
            S7 { x: self.x.clone() }
        }
    }
    let mut s1 = S7 { x: vec![12] };
    let s2 = s1.clone();
    s1.x[0] += 1;
    println!("{} {}", s1.x[0], s2.x[0]);
}