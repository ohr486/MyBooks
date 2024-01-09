fn main() {
    // 19.1
    fn quartic_root(x: f64) -> f64 { x.sqrt().sqrt() }
    let qr = quartic_root(100f64);
    println!("{} {}", qr * qr * qr *qr, qr);

    fn quartic_root_f64(x: f64) -> f64 { x.sqrt().sqrt() }
    fn quartic_root_f32(x: f32) -> f32 { x.sqrt().sqrt() }
    println!("{} {}",
        quartic_root_f64(100f64),
        quartic_root_f32(100f32));

    /*
    fn quartic_root2<Number>(x: Number) -> Number { x.sqrt().sqrt() }
    println!("{} {}",
        quartic_root2(100f64),
        quartic_root2(100f32)); // compile error
    */

    // 19.2
    trait HasSquareRoot {
        fn sq_root(self) -> Self;
    }
    impl HasSquareRoot for f32 {
        fn sq_root(self) -> Self { self.sqrt() }
    }
    impl HasSquareRoot for f64 {
        fn sq_root(self) -> Self { self.sqrt() }
    }
    fn quartic_root3<Number>(x: Number) -> Number
    where Number: HasSquareRoot {
        x.sq_root().sq_root()
    }
    println!("{} {}",
        quartic_root3(100f64),
        quartic_root3(100f32));

    // 19.3
    /*
    fn f<T>(a: T) -> T { a }
    fn g<T>(a: T) -> T {
        let b: T = a;
        let mut c = b;
        c = f(c);
        c
    }
    fn h<T>(a: &T) -> &T { a }
    */

    /*
    fn g2(a: i32) { }
    fn f2<T>(a: T) -> bool {
        g(a);
        a == a
    }
    */

    let mut a = 'A';
    let mut b = 'B';
    print!("{}, {}; ", a, b);
    std::mem::swap(&mut a, &mut b);
    println!("{}, {}", a, b);

    // 19.4
    fn sqrt() {}
    trait HasSquareRoot2 {
        fn sqrt(self) -> Self;
    }
    impl HasSquareRoot2 for f32 {
        fn sqrt(self) -> Self { self.sqrt() }
    }
    impl HasSquareRoot2 for f64 {
        fn sqrt(self) -> Self { self.sqrt() }
    }
    fn quartic_root4<Number>(x: Number) -> Number
    where Number: HasSquareRoot2 {
        x.sqrt().sqrt()
    }
    sqrt();
    println!("{} {}",
        quartic_root4(100f64),
        quartic_root4(100f32));

    // 19.5
    trait HasSqrtAndAbs {
        fn sqrt(self) -> Self;
        fn abs(self) -> Self;
    }
    impl HasSqrtAndAbs for f32 {
        fn sqrt(self) -> Self { self.sqrt() }
        fn abs(self) -> Self { self.abs() }
    }
    impl HasSqrtAndAbs for f64 {
        fn sqrt(self) -> Self { self.sqrt() }
        fn abs(self) -> Self { self.abs() }
    }
    fn abs_quartic_root<Number>(x: Number) -> Number
    where Number: HasSqrtAndAbs {
        x.abs().sqrt().sqrt()
    }
    println!("{} {}",
        abs_quartic_root(-100f64),
        abs_quartic_root(-100f32));

    // 19.6
    trait HasSquareRoot3 {
        fn sqrt(self) -> Self;
    }
    impl HasSquareRoot3 for f32 {
        fn sqrt(self) -> Self { self.sqrt() }
    }
    impl HasSquareRoot3 for f64 {
        fn sqrt(self) -> Self { self.sqrt() }
    }
    trait HasAbsoluteValue3 {
        fn abs(self) -> Self;
    }
    impl HasAbsoluteValue3 for f32 {
        fn abs(self) -> Self { self.abs() }
    }
    impl HasAbsoluteValue3 for f64 {
        fn abs(self) -> Self { self.abs() }
    }
    fn abs_quartic_root3<Number>(x: Number) -> Number
    where Number: HasSquareRoot3 + HasAbsoluteValue3 {
        x.abs().sqrt().sqrt()
    }
    println!("{} {}",
        abs_quartic_root3(-100f64),
        abs_quartic_root3(-100f32));

    // 19.7
    trait HasSquareRoot4 {
        fn sqrt(self) -> Self;
    }
    impl HasSquareRoot4 for f32 {
        fn sqrt(self) -> Self { self.sqrt() }
    }
    impl HasSquareRoot4 for f64 {
        fn sqrt(self) -> Self { self.sqrt() }
    }
    trait HasAbsoluteValue4 {
        fn abs(self) -> Self;
    }
    impl HasAbsoluteValue4 for f32 {
        fn abs(self) -> Self { self.abs() }
    }
    impl HasAbsoluteValue4 for f64 {
        fn abs(self) -> Self { self.abs() }
    }
    trait HasSqrtAndAbs4: HasSquareRoot4 + HasAbsoluteValue4 { }
    impl HasSqrtAndAbs4 for f32 {}
    impl HasSqrtAndAbs4 for f64 {}
    fn abs_quartic_root4<Number>(x: Number) -> Number
    where Number: HasSqrtAndAbs4 {
        x.abs().sqrt().sqrt()
    }
    println!("{} {}",
        abs_quartic_root4(-100f64),
        abs_quartic_root4(-100f32));

    // 19.8
    fn length(s: &str) -> usize { s.chars().count() }
    let s = "€èe";
    println!("{} {}", s.len(), length(s));

    /*
    impl str { // compile error
        fn length(&self) -> usize { self.chars().count() }
    }
    let s = "€èe";
    println!("{} {}", s.len(), s.length());
    */

    trait HasLength {
        fn length(&self) -> usize;
    }
    impl HasLength for str {
        fn length(&self) -> usize { self.chars().count() }
    }
    let s = "€èe";
    println!("{} {}", s.len(), s.length());

    // 19.9
    struct Complex {
        re: f64,
        im: f64,
    }
    impl std::fmt::Display for Complex {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{} {} {}i",
                self.re,
                if self.im >= 0. { '+' } else { '-' },
                self.im.abs()
            )
        }
    }
    let c1 = Complex { re: -2.3, im: 0. };
    let c2 = Complex { re: -2.1, im: -5.2 };
    let c3 = Complex { re: -2.2, im: 5.2 };
    println!("{}, {}, {}", c1, c2, c3);

    // 19.10
    fn exponentiate(base: f64, exponent: f64) -> f64 {
        (base.ln() * exponent).exp()
    }
    println!("{}", exponentiate(2.5, 3.2));

    trait HasLnExpMultiply {
        fn ln(self) -> Self;
        fn exp(self) -> Self;
        fn multiply(self, other: Self) -> Self;
    }
    impl HasLnExpMultiply for f64 {
        fn ln(self) -> Self { self.ln() }
        fn exp(self) -> Self { self.exp() }
        fn multiply(self, other: Self) -> Self { self * other }
    }
    impl HasLnExpMultiply for f32 {
        fn ln(self) -> Self { self.ln() }
        fn exp(self) -> Self { self.exp() }
        fn multiply(self, other: Self) -> Self { self * other }
    }
    fn exponentiate2<Number>(base: Number, exponent: Number) -> Number
    where Number: HasLnExpMultiply {
        (base.ln().multiply(exponent)).exp()
    }
    println!("{} {}",
         exponentiate2(2.5, 3.2),
         exponentiate2(2.5f32, 3.2));

    trait HasLnExp {
        fn ln(self) -> Self;
        fn exp(self) -> Self;
    }
    impl HasLnExp for f64 {
        fn ln(self) -> Self { self.ln() }
        fn exp(self) -> Self { self.exp() }
    }
    impl HasLnExp for f32 {
        fn ln(self) -> Self { self.ln() }
        fn exp(self) -> Self { self.exp() }
    }
    trait HasMultiply<Rhs> {
        fn multiply(self, rhs: Rhs) -> Self;
    }
    impl<Rhs> HasMultiply<Rhs> for f64 where Rhs: Into<Self> {
        fn multiply(self, rhs: Rhs) -> Self { self * rhs.into() }
    }
    impl<Rhs> HasMultiply<Rhs> for f32 where Rhs: Into<Self> {
        fn multiply(self, rhs: Rhs) -> Self { self * rhs.into() }
    }
    fn exponentiate3<Base, Exponent>(
        base: Base, exponent: Exponent) -> Base
    where Base: HasLnExp + HasMultiply<Exponent> {
        (base.ln().multiply(exponent)).exp()
    }
    println!("{}", exponentiate3(2.5f32, 3i16));
    println!("{}", exponentiate3(2.5f64, 3i16));
    println!("{}", exponentiate3(2.5f32, 3f32));
    println!("{}", exponentiate3(2.5f64, 3f32));
    println!("{}", exponentiate3(2.5f64, 3f64));

    // 19.11
    trait Dictionary<Key> {
        fn get(&self, key: Key) -> Option<String>;
    }
    struct Record {
        id: u32,
        name: String,
    }
    struct RecordSet {
        data: Vec<Record>,
    }
    impl Dictionary<u32> for RecordSet {
        fn get(&self, key: u32) -> Option<String> {
            for record in self.data.iter() {
                if record.id == key {
                    return Some(String::from(&record.name));
                }
            }
            None
        }
    }
    fn get_name<D>(dict: &D, id: u32) -> Option<String>
    where D: Dictionary<u32>
    {
        dict.get(id)
    }
    let names = RecordSet {
        data: vec![
            Record { id: 34, name: "John".to_string() },
            Record { id: 49, name: "Jane".to_string() },
        ],
    };
    println!("{:?}, {:?}",
        get_name(&names, 48),
        get_name(&names, 49));

    trait Dictionary2<Key, Count> {
        fn get(&self, key: Key) -> Option<String>;
        fn count(&self, key: Key) -> Count;
    }
    struct Record2 {
        id: u32,
        name: String,
    }
    struct RecordSet2 {
        data: Vec<Record2>,
    }
    impl Dictionary2<u32, usize> for RecordSet2 {
        fn get(&self, key: u32) -> Option<String> {
            for record in self.data.iter() {
                if record.id == key {
                    return Some(String::from(&record.name));
                }
            }
            None
        }
        fn count(&self, key: u32) -> usize {
            let mut c = 0;
            for record in self.data.iter() {
                if record.id == key {
                    c += 1;
                }
            }
            c
        }
    }
    fn get_name2<D>(dict: &D, id: u32) -> Option<String>
    where D: Dictionary2<u32, usize>
    {
        dict.get(id)
    }
    let names = RecordSet2 {
        data: vec![
            Record2 { id: 34, name: "John".to_string() },
            Record2 { id: 49, name: "Jane".to_string() },
        ],
    };
    println!("{}, {}; {:?}, {:?}",
        names.count(48),
        names.count(49),
        get_name2(&names, 48),
        get_name2(&names, 49));

    trait Dictionary3 {
        type Key;
        type Count;
        fn get(&self, key: Self::Key) -> Option<String>;
        fn count(&self, key: Self::Key) -> Self::Count;
    }
    struct Record3 {
        id: u32,
        name: String,
    }
    struct RecordSet3 {
        data: Vec<Record3>,
    }
    impl Dictionary3 for RecordSet3 {
        type Key = u32;
        type Count = usize;
        fn get(&self, key: Self::Key) -> Option<String> {
            for record in self.data.iter() {
                if record.id == key {
                    return Some(String::from(&record.name));
                }
            }
            None
        }
        fn count(&self, key: Self::Key) -> Self::Count {
            let mut c = 0;
            for record in self.data.iter() {
                if record.id == key {
                    c += 1;
                }
            }
            c
        }
    }
    fn get_name3<D>(
        dict: &D,
        id: <D as Dictionary3>::Key,
    ) -> Option<String>
    where D: Dictionary3
    {
        dict.get(id)
    }
    let names = RecordSet3 {
        data: vec![
            Record3 { id: 34, name: "John".to_string() },
            Record3 { id: 49, name: "Jane".to_string() },
        ],
    };
    println!("{}, {}; {:?} {:?}",
        names.count(48),
        names.count(49),
        get_name3(&names, 48),
        get_name3(&names, 49));

    // 19.12
    fn get_third(r: std::ops::Range<u32>) -> Option<u32> {
        if r.len() >= 3 {
            Some(r.start + 2)
        } else {
            None
        }
    }
    println!("{:?}, {:?}", get_third(10..12), get_third(20..29));

    fn get_third2(s: &[f64]) -> Option<f64> {
        if s.len() >= 3 {
            Some(s[2])
        } else {
            None
        }
    }
    println!("{:?}, {:?}",
        get_third2(&[3.1, 3.2]),
        get_third2(&[4.1, 4.2, 4.3, 4.4]));

    fn get_third3<Iter>(mut iterator: Iter) -> Option<Iter::Item>
    where Iter: Iterator,
    {
        iterator.next();
        iterator.next();
        iterator.next()
    }
    println!(
        "{:?}, {:?}, {:?}, {:?}",
        get_third3(10..12),
        get_third3(20..29),
        get_third3([3.1, 3.2].iter()),
        get_third3([4.1, 4.2, 4.3, 4.4].iter()));

    println!(
        "{:?}, {:?}, {:?}, {:?}",
        (10..12).nth(2),
        (20..29).nth(2),
        ([3.1, 3.2].iter()).nth(2),
        ([4.1, 4.2, 4.3, 4.4].iter()).nth(2));
}