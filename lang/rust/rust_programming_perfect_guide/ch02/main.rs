fn main() {
    // 2.1
    println!("The sum is {}", 80 + 34);
    println!("{} + {} = {}", 34, 80, 80 + 34);

    // 2.2
    println!("{}", (23 - 6) % 5 + 20 * 30 / (3 + 4));

    // 2.3
    println!("The sum is {}", 80.3 + 34.8);
    println!("The sum is {}", 80.3 + 34.9);
    println!("{}", (23. - 6.) % 5. + 20. * 30. / (3. + 4.));
    // println!("{}", 2.7 + 1); // compile error
    println!("{}", 2.7 + 1.);
    println!("{} {}", -12 % 10, -1.2 % 1.);

    // 2.4
    print!("{} + ", 80);
    print!("{} =", 34);
    println!(" {}", 80 + 34);
    print!("{} + ", 80);print!("{} = ",34);
            println ! ( "{}" ,
        80      + 34 ) ;

    // 2.5
    // println!("{}", "This"
    //         "is not allowed"); // compile error
    println!("{}", "These
        are
        three lines");
    println!("{}", "This \
        is \
        just one line");
    println!("{}", "These
are
three lines");
    println!("{}", "These\n\
        are\n\
        three lines");
}