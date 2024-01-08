fn main() {
    // 16.1
    let s = "abc012è€";
    for i in 0..s.len() {
        println!("{}: {}", i, s.as_bytes()[i]);
    }

    // 16.2
    fn print_nth_char(s: &str, mut n: u32) {
        let mut iter: std::str::Chars = s.chars();
        loop {
            let item: Option<char> = iter.next();
            match item {
                Some(c) => if n == 0 {
                    print!("{}", c);
                    break;
                },
                None => { break; },
            }
            n -= 1;
        }
    }
    print_nth_char("€èe", 0);
    print_nth_char("€èe", 2);
    println!();

    fn print_codes(s: &str) {
        let mut iter = s.chars();
        loop {
            match iter.next() {
                Some(c) => { println!("{}: {}", c, c as u32); },
                None => { break; },
            }
        }
    }
    print_codes("€èe");

    // 16.3
    fn print_codes2(s: &str) {
        for c in s.chars() {
            println!("{}: {}", c, c as u32);
        }
    }
    print_codes2("€èe");

    let _v1 = (0u32..10).next();
    let _v2 = (5u32..).next();
    // let _v3 = (..8u32).next(); // compile error
    // let _v4 = (..).next(); // compile error

    for byte in "€èe".bytes() {
        print!("{} ", byte);
    }
    println!();

    let string: &str = "€èe";
    let string_it: std::str::Bytes = string.bytes();
    for byte in string_it {
        print!("{} ", byte);
    }
    println!();

    // 16.5
    let vec_iterator: std::vec::IntoIter<i32>
        = vec![10, 20, 30].into_iter();
    for item in vec_iterator {
        let j: i32 = item;
        print!("{} ", j + 1);
    }
    println!();

    for item in vec![10, 20, 30].into_iter() {
        print!("{} ", item + 1);
    }
    println!();

    let array_iterator: std::array::IntoIter<i32, 3_usize>
        = [10, 20, 30].into_iter();
    for item in array_iterator {
        let j: i32 = item;
        print!("{} ", j + 1);
    }
    println!();

    let slice_iterator: std::slice::Iter<i32>
        = [40, 50, 60][0..2].into_iter();
    for item in slice_iterator {
        let j: i32 = *item;
        print!("{} ", j + 1);
    }
    println!();

    /*
    let mut v = vec![10, 20, 30];
    for item in v.into_iter() {
        item += 1; // compile error
        print!("{} ", item);
    }
    */

    let v = vec![10, 20, 30];
    for mut item in v.into_iter() {
        item += 1;
        print!("{} ", item);
    }
    println!();

    let v = vec![10, 20, 30];
    let vec_ref_iterator: std::slice::Iter<i32> = v.iter();
    for item_ref in vec_ref_iterator {
        print!("{} ", *item_ref + 1);
    }
    println!();

    let array_ref_iterator: std::slice::Iter<i32>
        = [10, 20, 30].iter();
    for item in array_ref_iterator {
        print!("{} ", *item + 1);
    }
    print!("; ");
    let slice_ref_iterator: std::slice::Iter<i32>
        = [10, 20, 30][0..2].iter();
    for item in slice_ref_iterator {
        print!("{} ", *item + 1);
    }
    println!();

    // 16.8
    let slice1 = &[3, 4, 5];
    let slice2 = &[7, 8];
    let mut iterator = slice1.iter();
    for item_ref in iterator {
        print!("{} ", *item_ref);
    }
    print!("; ");
    iterator = slice2.iter();
    for item_ref in iterator {
        print!("{} ", *item_ref);
    }
    println!();

    let mut v = vec![3, 4, 5];
    let iterator: std::slice::IterMut<i32> = v.iter_mut();
    for mut_item_ref in iterator {
        *mut_item_ref += 1;
    }
    println!("{:?}", v);

    // 16.9
    for item in vec![10, 20, 30].into_iter() {
        print!("{} ", item + 1);
    }
    println!();

    for item in &vec![10, 20, 30] {
        print!("{} ", *item + 1);
    }
    println!();

    for item in vec![10, 20, 30].iter() {
        print!("{} ", *item + 1);
    }
    println!();

    for item in &vec![10, 20, 30] {
        print!("{} ", *item + 1);
    }
    println!();

    for item in vec![10, 20, 30].iter_mut() {
        *item += 1;
        print!("{} ", *item);
    }
    println!();

    for item in &mut vec![10, 20, 30] {
        *item += 1;
        print!("{} ", *item);
    }
    println!();

    // 16.11
    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.into_iter() {
        if n < 0 { print!("{} ", n); }
    }
    println!();

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.into_iter().filter(|x_ref| *x_ref < 0) {
        print!("{} ", n);
    }
    println!();

    // 16.12
    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.into_iter() {
        print!("{} ", n * 2);
    }
    println!();

    let arr = [66, -8, 43, 19, 0, -31];
    for n in arr.into_iter().map(|x| x * 2) {
        print!("{} ", n);
    }
    println!();

    // 16.13
    let arr = ['a', 'b', 'c'];
    for index in 0..arr.len() {
        print!("{} {}, ", index, arr[index]);
    }
    println!();

    let arr = ['a', 'b', 'c'];
    let mut index = 0;
    for ch in arr.into_iter() {
        print!("{} {}, ", index, ch);
        index += 1;
    }
    println!();

    let arr = ['a', 'b', 'c'];
    for (index, ch) in arr.into_iter().enumerate() {
        print!("{} {}, ", index, ch);
    }
    println!();

    // 16.14
    let s = "Hello, world!";
    let ch = 'R';
    let mut contains = false;
    for c in s.chars() {
        if c == ch {
            contains = true;
        }
    }
    println!("\"{}\" {} '{}'.",
        s,
        if contains {
            "contains"
        } else {
            "does not contain"
        },
        ch);

    let s = "Hello, world!";
    let ch = 'R';
    println!("\"{}\" {} '{}'.",
        s,
        if s.chars().any(|c| c == ch) {
            "contains"
        } else {
            "does not contain"
        },
        ch);

    print!("{} ",
        [45, 8, 2, 6].into_iter().any(|n| n < 0));
    println!("{}",
        [45, 8, -2, 6].into_iter().any(|n| n < 0));

    print!("{} ", [45, 8, 2, 6].into_iter()
        .any(|n: i32| -> bool { n < 0 }));
    println!("{}", [45, 8, -2, 6].into_iter()
        .any(|n: i32| -> bool { n < 0 }));

    // 16.15
    print!("{} ", [45, 8, 2, 6].into_iter()
        .all(|n: i32| -> bool { n > 0 }));
    println!("{}", [45, 8, -2, 6].into_iter()
        .all(|n: i32| -> bool { n > 0 }));

    // 16.16
    let s = "€èe";
    println!("{} {}", s.chars().count(), s.len());

    // 16.17
    println!("{}", [45, 8, -2, 6].into_iter().sum::<i32>());

    let s: i32 = [45, 8, -2, 6].into_iter().sum();
    println!("{}", s);

    let s: u32 = [0; 0].into_iter().sum();
    println!("{}", s);

    // 16.18
    let arr = [45, 8, -2, 6];
    match arr.into_iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match arr.into_iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match [0; 0].into_iter().min() {
        Some(n) => print!("{} ", n),
        _ => print!("---"),
    }
    println!();

    let arr = ["hello", "brave", "new", "world"];
    match arr.into_iter().min() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    match arr.into_iter().max() {
        Some(n) => print!("{} ", n),
        _ => (),
    }
    println!();

    // 16.19
    let arr = [36, 1, 15, 9, 4];
    let v = arr.into_iter().collect::<Vec<i32>>();
    println!("{:?}", v);

    let arr = [36, 1, 15, 9, 4];
    let v = arr.into_iter().collect::<Vec<_>>();
    println!("{:?}", v);

    let arr = [36, 1, 15, 9, 4];
    let v: Vec<_> = arr.into_iter().collect();
    println!("{:?}", v);

    let s = "Hello";
    println!("{:?}", s.chars().collect::<String>());
    println!("{:?}", s.chars().collect::<Vec<char>>());

    let s = "Hello";
    println!("{:?}", s.bytes().collect::<Vec<u8>>());
    println!("{:?}", s.as_bytes().iter().collect::<Vec<&u8>>());

    // 16.20
    let arr = [66, -8, 43, 19, 0, -31];
    let mut v = vec![];
    for i in 0..arr.len() {
        if arr[i] > 0 { v.push(arr[i] * 2); }
    }
    println!("{:?}", v);

    let arr = [66, -8, 43, 19, 0, -31];
    let mut v = vec![];
    for n in arr.into_iter() {
        if n > 0 { v.push(n * 2); }
    }
    println!("{:?}", v);

    let arr = [66, -8, 43, 19, 0, -31];
    let mut v = vec![];
    for n in arr
        .into_iter()
        .filter(|x| *x > 0)
        .map(|x| x * 2)
        {
            v.push(n);
        }
    println!("{:?}", v);

    let arr = [66, -8, 43, 19, 0, -31];
    let v = arr
        .into_iter()
        .filter(|x| *x > 0)
        .map(|x| x * 2)
        .collect::<Vec<_>>();
    println!("{:?}", v);

    // 16.21
    let v = [66, -8, 43, 19, 0, -31]
        .into_iter()
        .filter(|x| { print!("F{} ", x); *x > 0 })
        .map(|x| { print!("M{} ", x); x * 2 })
        .collect::<Vec<_>>();
    println!("{:?}", v);

    let mut v = vec![];
    for item in [66, -8, 43, 19, 0, -31]
        .into_iter()
        .filter(|x| { print!("F{} ", x); *x > 0 })
        .map(|x| { print!("M{} ", x); x * 2 })
        {
            v.push(item);
        }
    println!("{:?}", v);

    /*
    [66, -8, 43, 19, 0, -31]
        .into_iter()
        .filter(|x| { print!("F{} ", x); *x > 0 })
        .map(|x| { print!("M{} ", x); x * 2 }); // warning
    */
}