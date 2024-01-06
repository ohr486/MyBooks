fn main() {
    // 6.1
    let hexadecimal = 0x10; // 16進数
    let octal = 0o10; // 8進数
    let binary = 0b10; // 2進数
    let mut n = 10;
    print!("{} ", n);
    n = hexadecimal;
    print!("{} ", n);
    n = octal;
    print!("{} ", n);
    n = binary;
    println!("{} ", n);

    println!("{} {}", 0xA, 0b100000000);

    println!("{} {}", 10, 256);

    // 6.2
    let hexadecimal = 0x_00FF_F7A3;
    let decimal = 1_234_567;
    let octal = 0o_777_205_162;
    let binary = 0b_0110_1001_1111_0001;
    println!("{} {} {} {}", hexadecimal, decimal, octal, binary);

    // 6.3
    let one_thousand = 1e3; // 千
    let one_million = 1e6; // 百万
    let thirteen_billions_and_half = 13.5e9; // 135億
    let twelve_millionths = 12e-6; // 0.000012
    println!("{} {} {} {}",
        one_thousand, one_million,
        thirteen_billions_and_half, twelve_millionths
    );

    // 6.4
    let a: i8 = 5;
    let b: i16 = 5;
    let c: i32 = 5;
    let d: i64 = 5;
    let e: i128 = 5;
    println!("{} {} {} {} {}", a, b, c, d, e);

    // let a: i8 = 5;
    // let b: i16 = 5;
    // println!("{}", a + b); // compile error

    // 6.5
    let a: u8 = 5;
    let b: u16 = 5;
    let c: u32 = 5;
    let d: u64 = 5;
    let e: u128 = 5;
    println!("{} {} {} {} {}", a, b, c, d, e);

    // 6.6
    let arr = [11, 22, 33];
    let i: usize = 2;
    println!("{}", arr[i]);

    // let arr = [11, 22, 33];
    // let i: usize = 2;
    // print!{"{}", arr[i]};
    // let i: isize = 2;
    // print!{"{}", arr[i]}; // compile error
    // let i: u32 = 2;
    // print!{"{}", arr[i]}; // compile error
    // let i: u64 = 2;
    // print!{"{}", arr[i]}; // compile error

    // 6.7
    let a = [0];
    let i = 0;
    println!("{}", a[i]);

    let i = 0;
    let _j: u16 = i;

    // let i = 0;
    // let _j: u16 = i;
    // let _k: i16 = i; // compile error

    let i = 0;
    let _j: u16 = i;
    let _k = i;

    // let _n = 8_000_000_000; // compile error

    // 6.9
    let a: f64 = 4.6;
    let b: f32 = 3.91;
    println!("{} {}", a, b);

    let a = 4.6;
    let mut _b: f32 = 3.91e5;
    _b = a;

    // 6.10
    let a: i16 = 12;
    let b: u32 = 4;
    let c: f32 = 3.7;
    println!("{}", a as i8 + b as i8 + c as i8);

    // #[allow(overflowing_literals)]
    // let a = 500 as i8; // compile error
    // let b = 100_000 as u16; // compile error
    // let c = 10_000_000_000 as u32; // compile error
    // println!("{} {} {}", a, b, c);

    // 6.11
    let _a: i16 = -150;
    let _b = -150 as i16;
    let _c = -150 + _b - _b;
    let _d = -150i16;

    // 6.12
    let _: i8 = 127;
    let _: i16 = 32_767;
    let _: i32 = 2_147_483_647;
    let _: i64 = 9_223_372_036_854_775_807;
    let _: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let _: isize = 100;
    let _: u8 = 255;
    let _: u16 = 65_535;
    let _: u32 = 4_294_967_295;
    let _: u64 = 18_446_744_073_709_551_615;
    let _: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let _: usize = 100;
    let _: f32 = 1e38;
    let _: f64 = 1e308;

    // 6.13
    let a: bool = true; println!("[{}]", a);
    let b: char = 'a'; println!("[{}]", b);

    let e_grave = 'è';
    let japanese_character = 'さ';
    println!("{} {}", e_grave, japanese_character);

    // let _a = 'a' + 'b'; // compile error
    // let _b = false + true; // compile error

    println!("{} {} {} {} {}",
        true as u8, false as u8,
        'A' as u32, 'à' as u32, '€' as u32);

    let truthy = 1;
    let falsy = 0;
    println!("{} {} {} {}",
        truthy != 0, falsy != 0,
        65 as char, 224 as char);

    for n in 32..127 {
        println!("{}: [{}]", n, n as u8 as char);
    }
    for n in 160..256 {
        println!("{}: [{}]", n, n as u8 as char);
    }

    // 6.14
    let a: () = ();
    let b = { 12; 87; 283 };
    let c = { 12; 87; 283; };
    let d = {};
    let e = if false { };
    let f = while false { };
    println!("{:?} {} {:?} {:?} {:?} {:?}",
        a, b, c, d, e, f);

    // 6.15
    let _array1: [char; 3] = ['x', 'y', 'z'];
    let _array2: [f32; 200] = [0f32; 200];
    let _vector1: Vec<char> = vec!['x', 'y', 'z'];
    let _vector2: Vec<i32> = vec![0; 5000];

    // 6.16
    // let n = 20;
    // let _ = [0; n]; // compile error

    const N: usize = 20;
    let _ = [0; N];

    // 6.17
    // let _: () = 4u32 / 3u32; // compile error

    let _: () = 4 / 3; // compile error
}