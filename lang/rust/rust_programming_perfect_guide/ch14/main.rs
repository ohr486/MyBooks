fn main() {
    // 14.1
    let mut a = "Hel";
    print!("{}", a);
    a = "lo";
    println!("{}", a);

    use std::mem::*;
    let a: &str = "";
    let b: &str = "01234456789";
    let c: &str = "abcdé";
    println!("{} {} {}",
        size_of_val(a),
        size_of_val(b),
        size_of_val(c));

    // let a: str; // compile error
    // fn f(a: str) {} // compile error
    // print!("{}", std::mem::size_of::<str>()); // compile error

    // use std::mem::*; // already used
    let a: &str = "";
    let b: &str = "01234456789";
    let c: &str = "abcdé";
    print!("{} {} {}; ",
        size_of_val(&a),
        size_of_val(&b),
        size_of_val(&c));
    print!("{} {} {}",
        size_of_val(&&a),
        size_of_val(&&b),
        size_of_val(&&c));
    println!();

    // 14.2
    let mut a: String = "He".to_string();
    a.push('l');
    a.push('l');
    a.push('o');
    println!("{}", a);

    let mut a: String = "Xy".to_string();
    a.remove(0);
    a.insert(0, 'H');
    a.pop();
    a.push('i');
    println!("{}", a);

    // 14.3
    let mut s1 = "".to_string();
    s1.push('e');
    let mut s2 = "".to_string();
    s2.push('é');
    let mut s3 = "".to_string();
    s3.push('€');
    print!("{} {}; ", s1.capacity(), s1.len());
    print!("{} {}; ", s2.capacity(), s2.len());
    print!("{} {};", s3.capacity(), s3.len());
    println!();

    let mut s1 = "".to_string();
    for _ in 0..16 {
        println!("{:p} {} {}",
            s1.as_ptr(), s1.capacity(), s1.len());
        s1.push('a');
    }
    let s2 = "x".to_string();
    s1.push('-');
    println!("{:p}", s2.as_ptr());
    println!("{:p} {} {}: {}",
        s1.as_ptr(), s1.capacity(), s1.len(), s1);

    // 14.4
    let s1 = String::new();
    let s2 = String::from("");
    let s3 = "".to_string();
    let s4 = "".to_owned();
    let s5 = format!("");
    println!("({}{}{}{}{})", s1, s2, s3, s4, s5);

    let s = "a,";
    let s1 = String::from(s);
    let s2 = s.to_string();
    let s3 = s.to_owned();
    // let s4 = format!(s);
    // let s5 = format!("a,{}");
    let s6 = format!("{}", s);
    println!("({}{}{}{})", s1, s2, s3, s6);

    // 14.5
    let s1: String = "abc".to_string();
    let s2: &String = &s1;
    let s3: &str = &s1;
    println!("{} {} {}", s1, s2, s3);

    // 14.6
    let ss1 = "He";
    let ss2 = "llo ";
    let ds1 = ss1.to_string();
    let ds2 = ss2.to_string();
    let ds3 = format!("{}{}", ss1, ss2);
    print!("{}", ds3);
    let ds3 = format!("{}{}", ss1, ds2);
    print!("{}", ds3);
    let ds3 = format!("{}{}", ds1, ss2);
    print!("{}", ds3);
    let ds3 = format!("{}{}", ds1, ds2);
    print!("{}", ds3);
    println!();

    let vs = ["Hello", ", ", "world", "!"];
    let mut result = String::new();
    for s in vs {
        result = format!("{}{}", result, s);
    }
    println!("{}", result);

    let vs = ["Hello", ", ", "wrold", "!"];
    let mut result = String::new();
    for s in vs {
        result.push_str(s);
    }
    println!("{}", result);

    let vs = ["Hello", ", ", "wrold", "!"];
    let mut result = String::new();
    for s in vs {
        result += s;
    }
    println!("{}", result);

    let comma = ", ".to_string();
    let world = "world".to_string();
    let excl_point = '!';
    let mut result = "Hello".to_string();
    result += &comma;
    result.push_str(&world);
    result.push(excl_point);
    println!("{}", result);
}