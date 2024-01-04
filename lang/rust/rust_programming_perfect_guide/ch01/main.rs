fn main(){
    // 1.2
    print!("Hello, world!\n");

    // 1.3
    print!("{}, {}", "Hello", "world!\n");
    // print!("{}, !", "Hello", "world!"); // compile error
    // print!("{}, {}!", "Hello"); // compile error

    // 1.4
    print!("First line\nSecond line\nThird line\n");
    println!("text of the line");
    print!("text of the line\n");

    // 1.5
    println!("My number: 140");
    println!("My number: {}", "140");
    println!("My number: {}", 140);
    println!("My number: {}", 000140);
    println!("{}: {}", "My number", 140);

    // 1.6
    // This program
    // prints a number.
    println!("{}", 34); // thirty-four
    /* print!("{}", 80);
    */

    /* This is /* a valid*/
    comment, even /* if /* it contains
    comments*/ inside */itself. */

    // /* This is /* instead is not allowed in Rust,
    // while in C is tolerated (but it may generate a warning).*/
}