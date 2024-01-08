fn main() {
    // 15.1
    for i in 0..12 { println!("{}", i); }

    let dozen = 0..12;
    for i in dozen { println!("{}", i); }

    let range: std::ops::Range<usize> = 3..8;
    println!("{:?}, {}, {}, {}",
        range, range.start, range.end, range.len());
    for i in range { print!("{}, ", i); }
    println!();

    let r1 = 3u8..12u8;
    let r2 = 3u8..12;
    let r3 = 3..12u8;
    let r4 = 3..12;
    let r5 = -3..12;
    let r6 = 3..12 as i64;
    println!(
        "{} {} {} {} {} {}",
        std::mem::size_of_val(&r1),
        std::mem::size_of_val(&r2),
        std::mem::size_of_val(&r3),
        std::mem::size_of_val(&r4),
        std::mem::size_of_val(&r5),
        std::mem::size_of_val(&r6));

    // let r1 = 3u8..12i8; // compile error
    // let r2: std::ops::Range<u32> = -3..12; // compile error
    // let r3: std::ops::Range<i32> = 3i16..12; // compile error

    // let _r = 3u8..1200; // compile error

    let _r1 = false .. true;
    let _r2 = "hello" .. "world";
    let _r3 = 4.2 .. 7.9;

    // 15.2
    fn min1(arr: [i32; 8]) -> i32 {
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    println!("{}", min1([23, 17, 12, 16, 15, 28, 17, 30]));

    fn min2(arr: &[i32; 8]) -> i32 {
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    println!("{}", min2(&[23, 17, 12, 16, 15, 28, 17, 30]));

    fn min3(arr: &[i32; 8], start: usize, count: usize) -> i32 {
        let mut minimum = arr[start];
        for i in start + 1..start + count {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    println!("{}", min3(&[23, 17, 12, 16, 15, 28, 17, 30], 3, 2));

    // 15.3
    fn min4(arr: &[i32]) -> i32 {
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    println!("{}", min4(&[23, 17, 12, 16, 15, 28, 17, 30]));

    fn min5(arr: &[i32]) -> i32 {
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    print!("{} ", min5(&[23, 17]));
    println!("{}", min5(&vec![55, 22, 33, 44]));

    // 15.4
    fn min6(arr: &[i32]) -> i32 {
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    let arr = [23, 17, 12, 16, 15, 2];
    let range = 2..5;
    let slice_ref = &arr[range];
    println!("{}", min6(slice_ref));

    println!("{}", min6(&[23, 17, 12, 16, 15, 2][2..5]));

    let arr = [55, 22, 33, 44, 66, 7, 8];
    let v = vec![55, 22, 33, 44, 66, 7, 8];
    let sr1 = &arr[2..5];
    let sr2 = &v[2..5];
    println!("{:?} {:?} {:?} {:?}", sr1, sr2, &sr1[1..2], &sr2[1]);

    // 15.5
    let arr = [55, 22, 33, 44, 55];
    let _r1 = 4..4;
    let _a1 = &arr[_r1];
    let _r2 = 4..3;
    // let _a2 = &arr[_r2]; // runtime error
    let _r3 = -3i32..2;
    // let _a3 = &arr[_r3]; // compile error
    let _r4 = 3..8;
    // let _a4 = &arr[_r4]; // runtime error

    // 15.6
    let mut arr = [11, 22, 33, 44];
    {
        let sl_ref = &mut arr[1..3];
        print!("{:?}", sl_ref);
        sl_ref[1] = 0;
        print!(" {:?}", sl_ref);
    }
    println!(" {:?}", arr);

    let arr = [11, 22, 33, 44];
    {
        let mut sl_ref = &arr[1..3];
        print!("{:?}", sl_ref);
        sl_ref = &arr[0..1];
        print!(" {:?}", sl_ref);
    }
    println!(" {:?}", arr);

    // 15.7
    let arr = [11, 22, 33, 44];
    let n = 2;
    let sr1 = &arr[0..n];
    let sr2 = &arr[n..arr.len()];
    println!("{:?} {:?}", sr1, sr2);

    let arr = [11, 22, 33, 44];
    let n = 2;
    let sr1 = &arr[..n];
    let sr2 = &arr[n..];
    println!("{:?} {:?}", sr1, sr2);

    let r1: std::ops::RangeFrom<i32> = 3..;
    let r2: std::ops::RangeTo<i32> = ..12;
    println!("{:?} {:?} {} {}", r1, r2,
        std::mem::size_of_val(&r1),
        std::mem::size_of_val(&r2));

    for i in 3.. {
        if i * i > 40 { break; }
        print!("{} ", i);
    }
    println!();

    let range: std::ops::RangeFull = ..;
    let a1 = [11, 22, 33, 44];
    let a2 = &a1[range];
    println!("{} {:?} {:?}", std::mem::size_of_val(&range), a1, a2);

    // 15.8
    let arr = [11, 22, 33, 44];
    let r1: std::ops::RangeInclusive<usize> = 2..=3;
    print!("{:?} ", &arr[r1]);
    let r2: std::ops::RangeToInclusive<usize> = ..=2;
    println!("{:?}", &arr[r2]);
}