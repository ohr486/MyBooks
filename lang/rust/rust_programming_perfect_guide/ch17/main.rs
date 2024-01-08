fn main() {
    // 17.1
    let command_line: std::env::Args = std::env::args();
    for argument in command_line {
        print!("[{}]", argument);
    }
    println!();

    for a in std::env::args() {
        print!("[{}]", a);
    }
    println!();

    // 17.2
    // std::process::exit(107);

    // 17.3
    for var in std::env::vars() {
        println!("[{}]=[{}]", var.0, var.1);
    }

    print!("[{:?}]", std::env::var("abcd1"));
    std::env::set_var("abcd1", "This is the value");
    println!(" [{:?}]", std::env::var("abcd1"));

    print!("{}",
        if std::env::var("abcd2").is_ok() {
            "Already defined"
        } else {
            "Undefined"
        });
    std::env::set_var("abcd2", "This is the value");
    print!(", {}", match std::env::var("abcd2") {
        Ok(value) => value,
        Err(err) => format!("Still undefined: {}", err),
    });
    println!();

    // 17.4
    /*
    let mut line = String::new();
    println!("{:?}", std::io::stdin().read_line(&mut line));
    println!("[{}]", line);
    */

    /*
    let mut text = format!("First: ");
    let inp = std::io::stdin();
    inp.read_line(&mut text).unwrap();
    text.push_str("Second: ");
    inp.read_line(&mut text).unwrap();
    println!("{}: {} bytes", text, text.len());
    */

    /*
    let mut text = format!("First: ");
    let inp = std::io::stdin();
    inp.read_line(&mut text); // warning
    text.push_str("Second: ");
    inp.read_line(&mut text); // warning
    println!("{}: {} bytes", text, text.len());
    */

    // 17.5
    // fn f(x: i32) -> i32 {
    //     f4(f3(f2(f1(x))))
    // }

    fn f1(x: i32) -> Result<i32, String> {
        if x == 1 {
            Err(format!("Err. 1"))
        } else {
            Ok(x)
        }
    }
    fn f2(x: i32) -> Result<i32, String> {
        if x == 2 {
            Err(format!("Err. 2"))
        } else {
            Ok(x)
        }
    }
    fn f3(x: i32) -> Result<i32, String> {
        if x == 3 {
            Err(format!("Err. 3"))
        } else {
            Ok(x)
        }
    }
    fn f4(x: i32) -> Result<i32, String> {
        if x == 4 {
            Err(format!("Err. 4"))
        } else {
            Ok(x)
        }
    }
    fn f(x: i32) -> Result<i32, String> {
        match f1(x) {
            Ok(result) => {
                match f2(result) {
                    Ok(result) => {
                        match f3(result) {
                            Ok(result) => f4(result),
                            Err(err_msg) => Err(err_msg),
                        }
                    }
                    Err(err_msg) => Err(err_msg),
                }
            }
            Err(err_msg) => Err(err_msg),
        }
    }
    match f(2) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f(4) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f(5) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }

    fn f_2(x: i32) -> Result<i32, String> {
        let result1 = f1(x);
        if result1.is_err() { return result1; }
        let result2 = f2(result1.unwrap());
        if result2.is_err() { return result2; }
        let result3 = f3(result2.unwrap());
        if result3.is_err() { return result3; }
        f4(result3.unwrap())
    }
    match f_2(2) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f_2(4) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f_2(5) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }

    fn f_3(x: i32) -> Result<i32, String> {
        f4(f3(f2(f1(x)?)?)?)
    }
    match f_3(2) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f_3(4) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }
    match f_3(5) {
        Ok(y) => println!("{}", y),
        Err(e) => println!("Error: {}", e),
    }

    fn f1_2(n: i32) -> Result<i64, String> {
        Ok(f2_2(n * 2)? as i64 * 3)
    }
    fn f2_2(n: i32) -> Result<i32, String> {
        if n >= 0 {
            Ok(n * 5)
        } else {
            Err("Negative argument".to_string())
        }
    }
    print!("{:?} ", f1_2(10));
    print!("{:?} ", f1_2(7));
    print!("{:?} ", f1_2(-1));
    println!();

    // 17.6
    use std::io::Write;
    // std::io::stdout().write("Hi").unwrap(); // compile error
    // std::io::stdout().write(String::from("Hi")).unwrap(); // compile error
    std::io::stdout().write("Hello ".as_bytes()).unwrap();
    std::io::stdout().write(String::from("world").as_bytes()).unwrap();
    println!();

    // 17.7
    let int_str: String = 45.to_string();
    let float_str: String = 4.5.to_string();
    let bool_str: String = true.to_string();
    println!("{} {} {}", int_str, float_str, bool_str);

    // 17.8
    println!("{:?}", "true".parse::<bool>());
    println!("{:?}", "1.23e7".parse::<f32>());
    println!("{:?}", "1.23y7".parse::<f32>());

    println!("{:?}", "true".parse::<bool>().unwrap());
    println!("{:?}", "1.23e7".parse::<f32>().unwrap());
    println!("{:?}", "1.23y7".parse::<f32>().unwrap_err());

    // 17.9
    // use std::io::Write; // already imported
    let mut file = std::fs::File::create("data.txt").unwrap();
    file.write_all("eè€".as_bytes()).unwrap();

    use std::io::Read;
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    /*
    // use std::io::Read; // already imported
    // use std::io::Write; // already imported
    let mut command_line: std::env::Args = std::env::args();
    command_line.next().unwrap();
    let source = command_line.next().unwrap();
    let destination = command_line.next().unwrap();
    let mut file_in = std::fs::File::open(source).unwrap();
    let mut file_out = std::fs::File::create(destination).unwrap();
    let mut buffer = [0u8; 4096];
    loop {
        let nbytes = file_in.read(&mut buffer).unwrap();
        file_out.write_all(&buffer[..nbytes]).unwrap();
        if nbytes < buffer.len() { break;}
    }
    */

    // 17.10
    let mut command_line = std::env::args();
    command_line.next();
    let pathname = command_line.next().unwrap();
    let counts = count_lines(&pathname).unwrap();
    println!("file: {}", pathname);
    println!("n. of lines: {}", counts.0);
    println!("n. of empty lines: {}", counts.1);

    fn count_lines(pathname: &str)
        -> Result<(u32, u32), std::io::Error> {
        use std::io::BufRead;
        let f = std::fs::File::open(pathname)?;
        let f = std::io::BufReader::new(f);
        let mut n_lines = 0;
        let mut n_empty_lines = 0;
        for line in f.lines() {
            n_lines += 1;
            if line?.trim().len() == 0 {
                n_empty_lines += 1;
            }
        }
        Ok((n_lines, n_empty_lines))
    }
}