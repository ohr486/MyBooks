fn main() {
    // 13.1
    let mut arr = [4, 8, 1, 10 , 0, 45, 12, 7];
    arr.sort();
    println!("{:?}", arr);

    let mut arr = [4, 8, 1, 10 , 0, 45, 12, 7];
    use std::cmp::Ordering;
    fn desc(a: &i32, b: &i32) -> Ordering {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    }
    arr.sort_by(desc);
    println!("{:?}", arr);

    // 13.2
    /*
    let two = 2.;
    fn print_double(x: f64) {
        println!("{}", x * two); // compile error
    }
    print_double(17.2);
    */

    const TWO2: f64 = 2.;
    fn print_double2(x: f64) {
        println!("{}", x * TWO2);
    }
    print_double2(17.2);

    static TWO3: f64 = 2.;
    fn print_double3(x: f64) {
        println!("{}", x * TWO3);
    }
    print_double3(17.2);

    // 13.3
    let mut arr = [4, 8, 1, 10 , 0, 45, 12, 7];
    // use std::cmp::Ordering; // already imported
    let desc = |a: &i32, b: &i32| -> Ordering {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    };
    arr.sort_by(desc);
    println!("{:?}", arr);

    let mut arr = [4, 8, 1, 10 , 0, 45, 12, 7];
    // use std::cmp::Ordering; // already imported
    arr.sort_by(|a, b|
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal });
    println!("{:?}", arr);

    let mut arr = [4, 8, 1, 10 , 0, 45, 12, 7];
    arr.sort_by(|a, b| b.cmp(a));
    println!("{:?}", arr);

    let mut arr = [4, 8, 1, 10 , 0, 45, 12, 7];
    arr.sort_by(|a, b| (-*a).cmp(&-*b));
    println!("{:?}", arr);

    // 13.4
    let factor = 2;
    let multiply = |a| a * factor;
    print!("{} ", multiply(13));
    let multiply_ref = &multiply;
    println!(
        "{} {} {} {} {}",
        (*multiply_ref)(13),
        multiply_ref(13),
        (|a| a * factor)(13),
        (|a: i32| a * factor)(13),
        |a| -> i32 { a * factor }(13));

    println!(
        "{}",
        (|v: &Vec<i32>| {
            let mut sum = 0;
            for i in 0..v.len() {
                sum += v[i];
            }
            sum
        })(&vec![11, 22, 34]));
}