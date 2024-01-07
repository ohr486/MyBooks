fn main() {
    // 11.3
    static _A: u32 = 3;
    static _B: i32 = -1_000_000;
    static _C: f64 = 5.7e10;
    static _D: u8 = 200;

    // 11.4
    let _a: u32 = 3;
    let _b: i32 = -1_000_000;
    let _c: f64 = 5.7e10;
    let _d: u8 = 200;

    #[allow(unused_variables)]
    fn f1(x1: i32) {
        let y1 = 2 + x1;
    }
    fn f2(x2: i32) {
        f1(x2 + 7);
    }
    let k = 20;
    f1(k + 4);
    f2(30);

    // DANGER
    /*
    const SIZE: usize = 100_000;
    const N_ARRAY: usize = 1_000_000;
    fn create_array() -> [u8; SIZE] { [0u8; SIZE] }
    fn recursive_func(n: usize) {
        let a = create_array();
        println!("{} {}", N_ARRAY - n + 1, a[0]);
        if n > 1 { recursive_func(n - 1) }
    }
    recursive_func(N_ARRAY);
    */

    // 11.6
    // DANGER
    /*
    const SIZE: usize = 100_000;
    const N_ARRAY: usize = 1_000_000;
    fn create_array() -> Box<[u8; SIZE]> { Box::new([0u8; SIZE]) }
    fn recursive_func(n: usize) {
        let a = create_array();
        println!("{} {}", N_ARRAY - n + 1, a[0]);
        if n > 1 { recursive_func(n - 1) }
    }
    recursive_func(N_ARRAY);
    */

    // 11.8
    fn f(p: &f64) {
        let a = Box::new(*p);
        {
            let b = Box::new([1, 2, 3]);
            print!("{} {:?}", *a, *b);
        }
        let c = Box::new(true);
        print!(" {} {}", a, c);
    }
    f(&3.4);
    println!();

    // 11.10
    let a = 7;
    let a_box: Box<i32>;
    let mut a_ref: &i32 = &a;
    print!("{} {};", a, *a_ref);
    a_box = Box::new(a + 2);
    a_ref = &*a_box;
    println!(" {} {} {}", a, *a_ref, *a_box);

    let a = 7;
    let mut a_box: Box<i32>;
    let a_ref: &i32 = &a;
    print!("{} {};", a, a_ref);
    a_box = Box::new(a + 2);
    print!(" {} {} {};", a, a_ref, a_box);
    a_box = Box::new(*a_ref);
    println!(" {} {} {}", a, a_ref, a_box);
}