// 20.3
mod complex3 {
    pub struct Complex3 {
        pub re: f64,
        pub im: f64,
    }
    impl std::ops::Add for Complex3 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self { re: self.re + rhs.re, im: self.im + rhs.im }
        }
    }
}

mod complex4 {
    pub struct Complex4 {
        re: f64,
        im: f64,
    }
    impl Complex4 {
        pub fn from_re_im(re: f64, im: f64) -> Self {
            Self { re, im }
        }
        pub fn re(&self) -> &f64 { &self.re }
        pub fn im(&self) -> &f64 { &self.im }
    }
    impl std::ops::Add for Complex4 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self { re: self.re + rhs.re, im: self.im + rhs.im }
        }
    }
}

mod complex5 {
    pub struct Complex5<Num> {
        re: Num,
        im: Num,
    }
    impl<Num> Complex5<Num> {
        pub fn from_re_im(re: Num, im: Num) -> Self {
            Self { re, im }
        }
        pub fn re(&self) -> &Num { &self.re }
        pub fn im(&self) -> &Num { &self.im }
    }
    impl<Num> std::ops::Add for Complex5<Num>
    where Num: std::ops::Add<Output = Num>
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self { re: self.re + rhs.re, im: self.im + rhs.im }
        }
    }
}

fn main() {
    // 20.2
    struct CommunicationChannel {
        address: String,
        port: u16,
    }
    impl Drop for CommunicationChannel {
        fn drop(&mut self) {
            println!("Closing port {}:{}", self.address, self.port);
        }
    }
    impl CommunicationChannel {
        fn create(address: &str, port: u16) -> CommunicationChannel {
            println!("Opening port {}:{}", address, port);
            CommunicationChannel {
                address: address.to_string(),
                port: port,
            }
        }
        fn send(&self, msg: &str) {
            println!("Sent to {}:{} the message '{}'",
                self.address, self.port, msg);
        }
    }
    let channel_a = CommunicationChannel::create("usb4", 879);
    channel_a.send("Message 1");
    {
        let channel_b = CommunicationChannel::create("eth1", 12000);
        channel_b.send("Message 2");
    }
    channel_a.send("Message 3");

    struct S ( i32 );
    impl Drop for S {
        fn drop(&mut self) {
            println!("Dropped {}", self.0);
        }
    }
    let _ = S (1);
    let _ = S (2);
    let _ = S (3);
    {
        let _ = S (4);
        let _ = S (5);
        let _ = S (6);
        println!("INNER");
    }
    println!("OUTER");

    struct S2 ( i32 );
    impl Drop for S2 {
        fn drop(&mut self) {
            println!("Dropped {}", self.0);
        }
    }
    S2 (1); S2 (2); S2 (3);
    {
        S2 (4); S2 (5); S2 (6);
        println!("INNER");
    }
    println!("OUTER");

    // 20.3
    struct Complex {
        re: f64,
        im: f64,
    }
    fn add_complex(lhs: Complex, rhs: Complex) -> Complex {
        Complex { re: lhs.re + rhs.re, im: lhs.im + rhs.im }
    }
    let z1 = Complex { re: 3.8, im: -2.1 };
    let z2 = Complex { re: -1.5, im: 8.6 };
    let z3 = add_complex(z1, z2);
    println!("{} + {}i", z3.re, z3.im);

    struct Complex2 {
        re: f64,
        im: f64,
    }
    impl std::ops::Add for Complex2 {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Self { re: self.re + rhs.re, im: self.im + rhs.im }
        }
    }
    let z1 = Complex2 { re: 3.8, im: -2.1 };
    let z2 = Complex2 { re: -1.5, im: 8.6 };
    use std::ops::Add;
    let z3 = z1.add(z2);
    println!("{} + {}i", z3.re, z3.im);

    let z1 = Complex2 { re: 3.8, im: -2.1 };
    let z2 = Complex2 { re: -1.5, im: 8.6 };
    let z3 = z1 + z2;
    println!("{} + {}i", z3.re, z3.im);

    use complex3::Complex3;
    let z1 = Complex3 { re: 3.8, im: -2.1 };
    let z2 = Complex3 { re: -1.5, im: 8.6 };
    let z3 = z1 + z2;
    println!("{} + {}i", z3.re, z3.im);

    use complex4::Complex4;
    let z1 = Complex4::from_re_im(3.8, -2.1);
    let z2 = Complex4::from_re_im(-1.5, 8.6);
    let z3 = z1 + z2;
    println!("{} + {}i", z3.re(), z3.im());

    use complex5::Complex5;
    let z1 = Complex5::from_re_im(3.8, -2.1);
    let z2 = Complex5::from_re_im(-1.5, 8.6);
    let z3 = z1 + z2;
    println!("{} + {}i", z3.re(), z3.im());

    // 20.4
    let ln = 1_f64 / 3_f64;
    let sn = ln as f32;
    println!("{}", sn);

    /*
    struct LargeNumber (f64);
    struct SmallNumber (f32);
    let ln = LargeNumber (1. / 3.);
    let sn = ln as SmallNumber;
    println!("{}", sn.0); // compile error
    */

    struct LargeNumber (f64);
    struct SmallNumber (f32);
    impl Into<SmallNumber> for LargeNumber {
        fn into(self) -> SmallNumber {
            SmallNumber (self.0 as f32)
        }
    }
    let ln = LargeNumber (1. / 3.);
    let sn: SmallNumber = ln.into();
    println!("{}", sn.0);

    struct LargeNumber2 (f64);
    struct SmallNumber2 (f32);
    impl From<LargeNumber2> for SmallNumber2 {
        fn from(source: LargeNumber2) -> Self {
            Self (source.0 as f32)
        }
    }
    let ln = LargeNumber2 (1. / 3.);
    // let sn = SmallNumber2::from(ln);
    let sn: SmallNumber2 = ln.into();
    println!("{}", sn.0);

    // 20.5
    struct LargeNumber3 (f64);
    struct SmallNumber3 (f32);
    impl TryFrom<LargeNumber3> for SmallNumber3 {
        type Error = String;
        fn try_from(source: LargeNumber3) -> Result<Self, Self::Error> {
            if source.0.abs() > f32::MAX as f64 {
                Err("too large number".to_string())
            }
            else if source.0 != 0.
                && source.0.abs() < f32::MIN_POSITIVE as f64 {
                Err("too small number".to_string())
            }
            else {
                let result = source.0 as f32;
                if result as f64 == source.0 {
                    Ok(Self (result))
                }
                else {
                    Err("precision loss".to_string())
                }
            }
        }
    }
    fn show_result(n: Result<SmallNumber3, String>) {
        match n {
            Ok(x) => println!("Converted to {}", x.0),
            Err(msg) => println!("Conversion Error: {}", msg),
        }
    }
    let ln = LargeNumber3 (1.5);
    show_result(SmallNumber3::try_from(ln));
    let ln = LargeNumber3 (1.0 / 3.);
    show_result(SmallNumber3::try_from(ln));
    let ln = LargeNumber3 (1e50);
    show_result(SmallNumber3::try_from(ln));
    let ln = LargeNumber3 (1e-50);
    show_result(SmallNumber3::try_from(ln));

    // 20.6
    struct Text { characters: String }
    impl Text {
        fn from(text: &str) -> Text {
            Text { characters: text.to_string() }
        }
        fn draw(&self) {
            print!("{}", self.characters);
        }
    }
    let greeting = Text::from("Hello");
    greeting.draw();
    struct BoxedText {
        text: Text,
        first: char,
        last: char,
    }
    impl BoxedText {
        fn write_text_and_borders(text: &str, first: char, last: char) -> BoxedText {
            BoxedText {
                text: Text::from(text),
                first: first,
                last: last,
            }
        }
        fn draw(&self) {
            print!("{}", self.first);
            self.text.draw();
            print!("{}", self.last);
        }
    }
    let boxed_greeting = BoxedText::write_text_and_borders("Hi", '[', ']');
    print!(", ");
    boxed_greeting.draw();
    println!();

    // 20.7
    trait Draw2 {
        fn draw(&self);
    }
    struct Text2 { characters: String }
    impl Text2 {
        fn from(text: &str) -> Text2 {
            Text2 { characters: text.to_string() }
        }
    }
    impl Draw2 for Text2 {
        fn draw(&self) {
            print!("{}", self.characters);
        }
    }
    struct BoxedText2 {
        text: Text2,
        first: char,
        last: char,
    }
    impl BoxedText2 {
        fn write_text_and_borders(text: &str, first: char, last: char) -> BoxedText2 {
            BoxedText2 {
                text: Text2::from(text),
                first: first,
                last: last,
            }
        }
    }
    impl Draw2 for BoxedText2 {
        fn draw(&self) {
            print!("{}", self.first);
            self.text.draw();
            print!("{}", self.last);
        }
    }
    let greeting = Text2::from("Hello");
    let boxed_greeting = BoxedText2::write_text_and_borders("Hi", '[', ']');
    fn draw_text<T>(txt: T) where T: Draw2 {
        txt.draw();
    }
    draw_text(greeting);
    print!(", ");
    draw_text(boxed_greeting);
    println!();

    // 20.8
    let greeting2 = Text2::from("Hello2");
    let boxed_greeting2 = BoxedText2::write_text_and_borders("Hi2", '[', ']');
    fn draw_text2<T>(txt: &T) where T: Draw2 {
        txt.draw();
    }
    draw_text2(&greeting2);
    print!(", ");
    draw_text2(&boxed_greeting2);
    println!();

    let greeting3 = Text2::from("Hello3");
    let boxed_greeting3 = BoxedText2::write_text_and_borders("Hi3", '[', ']');
    fn draw_text3(txt: &dyn Draw2) {
        txt.draw();
    }
    draw_text3(&greeting3);
    print!(", ");
    draw_text3(&boxed_greeting3);
    println!();

    // 20.9
    let greeting4 = Text2::from("Hello4");
    let boxed_greeting4 = BoxedText2::write_text_and_borders("Hi4", '[', ']');
    use std::mem::size_of_val;
    print!("{} {} {}, {} {} {}, ",
        size_of_val(&greeting4),
        size_of_val(&&greeting4),
        size_of_val(&&&greeting4),
        size_of_val(&boxed_greeting4),
        size_of_val(&&boxed_greeting4),
        size_of_val(&&boxed_greeting4));
    fn draw_text4(txt: &dyn Draw2) {
        print!("{} {} {} ",
            size_of_val(txt),
            size_of_val(&txt),
            size_of_val(&&txt));
        txt.draw();
    }
    draw_text4(&greeting4);
    print!(", ");
    draw_text4(&boxed_greeting4);
    println!();

    // 20.10
    /*
    fn draw_text5<T>(txt: T) where T: Draw2 {
        txt.draw();
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let to_box = input.trim() == "b";
    if to_box {
        draw_text5(boxed_greeting4);
    } else {
        draw_text5(greeting4);
    }
    */

    fn draw_text6(txt: &dyn Draw2) {
        txt.draw();
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let to_box = input.trim() == "b";
    draw_text6(if to_box { &boxed_greeting4 } else { &greeting4 });
    println!();
}