fn main() {
    // 21.2
    let start_time = std::time::Instant::now();
    for i in 0..10_000 {
        println!("{}", i);
    }
    println!("{:?}", start_time.elapsed());

    // 21.3
    use std::time::Instant;
    const SIZE: usize = 100_000_000;
    let start_time = Instant::now();
    let mut v = Vec::<usize>::with_capacity(SIZE);
    let t1 = start_time.elapsed();
    for i in 0..SIZE {
        v.push(i);
    }
    let t2 = start_time.elapsed();
    for _ in 0..SIZE {
        v.pop();
    }
    let t3 = start_time.elapsed();
    println!("{:?} {:?} {:?}", t1, t2 - t1, t3 - t2);

    // use std::time::Instant; // already used
    const SIZE2: usize = 100_000;
    let start_time = Instant::now();
    let mut v = Vec::<usize>::with_capacity(SIZE2);
    let t1 = start_time.elapsed();
    for i in 0..SIZE2 {
        v.insert(0, i);
    }
    let t2 = start_time.elapsed();
    for _ in 0..SIZE2 {
        v.remove(0);
    }
    let t3 = start_time.elapsed();
    println!("{:?} {:?} {:?}", t1, t2 - t1, t3 - t2);

    // 21.4
    // use std::time::Instant; // already used
    const SIZE3: usize = 40_000;
    let start_time = Instant::now();
    let mut v = Vec::<usize>::new();
    for i in 0..SIZE3 {
        v.push(i);
        v.push(SIZE3 + i);
        v.remove(0);
        v.push(SIZE3 * 2 + i);
        v.remove(0);
    }
    let t1 = start_time.elapsed();
    while v.len() > 0 {
        v.remove(0);
    }
    let t2 = start_time.elapsed();
    println!("{:?} {:?}", t1, t2 - t1);

    // use std::time::Instant; // already used
    let start_time = Instant::now();
    let mut v = Vec::<usize>::new();
    for i in 0..SIZE3 {
        v.insert(0, i);
        v.insert(0, SIZE3 + i);
        v.pop();
        v.insert(0, SIZE3 * 2 + i);
        v.pop();
    }
    let t1 = start_time.elapsed();
    while v.len() > 0 {
        v.pop();
    }
    let t2 = start_time.elapsed();
    println!("{:?} {:?}", t1, t2 - t1);

    // use std::time::Instant; // already used
    let start_time = Instant::now();
    let mut vd = std::collections::VecDeque::<usize>::new();
    for i in 0..SIZE3 {
        vd.push_back(i);
        vd.push_back(SIZE3 + i);
        vd.pop_front();
        vd.push_back(SIZE3 * 2 + i);
        vd.pop_front();
    }
    let t1 = start_time.elapsed();
    while vd.len() > 0 {
        vd.pop_front();
    }
    let t2 = start_time.elapsed();
    println!("{:?} {:?}", t1, t2 - t1);

    // use std::time::Instant; // already used
    let mut v = Vec::<usize>::new();
    let mut vd = std::collections::VecDeque::<usize>::new();
    let start_time = Instant::now();
    for i in 0..SIZE3 {
        v.push(i);
    }
    let t1 = start_time.elapsed();
    for i in 0..SIZE3 {
        vd.push_back(i);
    }
    let mut count = 0;
    let t2 = start_time.elapsed();
    for i in v.iter() {
        count += i;
    }
    let t3 = start_time.elapsed();
    for i in vd.iter() {
        count += i;
    }
    let t4 = start_time.elapsed();
    println!("{} {:?} {:?} {:?} {:?}", count, t1, t2 - t1, t3 - t2, t4 - t3);

    // 21.6
    let data = vec![48, 18, 20, 35, 17, 13, 39, 12, 42, 33, 29, 27, 50, 16];
    let mut v = Vec::<i32>::new();
    fn add(v: &mut Vec<i32>, item: i32) {
        v.push(item);
    }
    fn extract(v: &mut Vec<i32>) -> i32 {
        v.pop().unwrap()
    }
    let mut i = 0;
    loop {
        if i == data.len() { break; }
        add(&mut v, data[i]);
        i += 1;
        if i == data.len() { break; }
        add(&mut v, data[i]);
        i += 1;
        print!("{} ", extract(&mut v));
    }
    while ! v.is_empty() {
        print!("{} ", extract(&mut v));
    }
    println!();

    let data = vec![48, 18, 20, 35, 17, 13, 39, 12, 42, 33, 29, 27, 50, 16];
    let mut v = Vec::<i32>::new();
    fn add2(v: &mut Vec<i32>, item: i32) {
        v.push(item);
        v.sort();
    }
    fn extract2(v: &mut Vec<i32>) -> i32 {
        v.pop().unwrap()
    }
    let mut i = 0;
    loop {
        if i == data.len() { break; }
        add2(&mut v, data[i]);
        i += 1;
        if i == data.len() { break; }
        add2(&mut v, data[i]);
        i += 1;
        print!("{} ", extract2(&mut v));
    }
    while ! v.is_empty() {
        print!("{} ", extract2(&mut v));
    }
    println!();

    use std::collections::BinaryHeap;
    let mut v = BinaryHeap::<i32>::new();
    fn add3(v: &mut BinaryHeap<i32>, item: i32) {
        v.push(item);
    }
    fn extract3(v: &mut BinaryHeap<i32>) -> i32 {
        v.pop().unwrap()
    }
    let mut i = 0;
    loop {
        if i == data.len() { break; }
        add3(&mut v, data[i]);
        i += 1;
        if i == data.len() { break; }
        add3(&mut v, data[i]);
        i += 1;
        print!("{} ", extract3(&mut v));
    }
    while ! v.is_empty() {
        print!("{} ", extract3(&mut v));
    }
    println!();

    // 21.7
    let arr = [6, 8, 2, 8, 4, 9, 6, 1, 8, 0];
    let mut v = Vec::<_>::new();
    let mut hs = std::collections::HashSet::<_>::new();
    let mut bs = std::collections::BTreeSet::<_>::new();
    for i in arr.iter() {
        v.push(i);
        hs.insert(i);
        bs.insert(i);
    }
    print!("Vec:");
    for i in v.iter() { print!(" {}", i); }
    println!(". {:?}", v);
    print!("HashSet:");
    for i in hs.iter() { print!(" {}", i); }
    println!(". {:?}", hs);
    print!("BTreeSet:");
    for i in bs.iter() { print!(" {}", i); }
    println!(". {:?}", bs);

    const SIZE4: u32 = 40_000;
    let mut v = Vec::<_>::new();
    let mut hs = std::collections::HashSet::<_>::new();
    let mut bs = std::collections::BTreeSet::<_>::new();
    let start_time = Instant::now();
    for i in 0..SIZE4 { v.push(i); }
    let t1 = start_time.elapsed();
    for i in 0..SIZE4 { hs.insert(i); }
    let t2 = start_time.elapsed();
    for i in 0..SIZE4 { bs.insert(i); }
    let t3 = start_time.elapsed();
    for i in 0..SIZE4 { if ! v.contains(&i) { return; } }
    let t4 = start_time.elapsed();
    v.swap(10_000, 20_000);
    v.sort();
    let t5 = start_time.elapsed();
    for i in 0..SIZE4 {
        if v.binary_search(&i).is_err() { return; }
    }
    let t6 = start_time.elapsed();
    for i in 0..SIZE4 { if ! hs.contains(&i) { return; } }
    let t7 = start_time.elapsed();
    for i in 0..SIZE4 { if ! bs.contains(&i) { return; } }
    let t8 = start_time.elapsed();
    println!("Pushes into Vec: {:?} per item", t1 / SIZE4);
    println!("Insertions into HashSet: {:?} per item", (t2 - t1) / SIZE4);
    println!("Insertions into BTreeSet: {:?} per item", (t3 - t2) / SIZE4);
    println!("Linear search in Vec: {:?} per item", (t4 - t3) / SIZE4);
    println!("Sorting of Vec: {:?} per item", (t5 - t4) / SIZE4);
    println!("Binary search in Vec: {:?} per item", (t6 - t5) / SIZE4);
    println!("Search in HashSet: {:?} per item", (t7 - t6) / SIZE4);
    println!("Search in BTreeSet: {:?} per item", (t8 - t7) / SIZE4);

    // 21.8
    let arr = [(640, 'T'), (917, 'C'), (412, 'S'), (670, 'T'), (917, 'L')];
    let mut v = Vec::<_>::new();
    let mut hm = std::collections::HashMap::<_, _>::new();
    let mut bm = std::collections::BTreeMap::<_, _>::new();
    for &(key, value) in arr.iter() {
        v.push((key, value));
        hm.insert(key, value);
        bm.insert(key, value);
    }
    print!("Vec:");
    for &(key, value) in v.iter() {
        print!(" {}: {},", key, value);
    }
    println!("\n    {:?}", v);
    print!("HashMap:");
    for (key, value) in hm.iter() {
        print!(" {}: {},", key, value);
    }
    println!("\n    {:?}", hm);
    print!("BTreeMap:");
    for (key, value) in bm.iter() {
        print!(" {}: {},", key, value);
    }
    println!("\n    {:?}", bm);





}