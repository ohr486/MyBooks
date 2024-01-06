fn main() {
    // 7.1
    const EUROPE: u8 = 0;
    const ASIA: u8 = 1;
    const AFRICA: u8 = 2;
    const AMERICA: u8 = 3;
    const OCEANIA: u8 = 4;
    let continent = ASIA;
    if continent == EUROPE { println!("E"); }
    else if continent == ASIA { println!("As"); }
    else if continent == AFRICA { println!("Af"); }
    else if continent == AMERICA { println!("Am"); }
    else if continent == OCEANIA { println!("O"); }

    #[allow(dead_code)]
    enum Continent1 {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }
    let contin1 = Continent1::Asia;
    match contin1 {
        Continent1::Europe => println!("E"),
        Continent1::Asia => println!("As"),
        Continent1::Africa => println!("Af"),
        // Continent1::Africa => fn aaa() {}, // compile error
        Continent1::America => println!("Am"),
        Continent1::Oceania => println!("O"),
    }

    // enum T {A, B, C, D}
    // let n: i32 = T::D; // compile error
    // let e: T = 1; // compile error

    // let a = 7.2; // warning
    12;
    true;
    // 4 > 7; // warning
    // 5.7 + 5. * a; // warning

    let a = 7.2;
    12;
    true;
    let _ = 4 > 7;
    let _ = 5.7 + 5. * a;

    #[allow(dead_code)]
    enum Continent2 {
        Europe,
        Asia,
        Africa,
        America,
        Oceania,
    }
    // let mut contin2 = Continent2::Asia;
    let contin2 = Continent2::Asia;
    match contin2 {
        Continent2::Europe => {
            // contin2 = Continent2::Asia;
            println!("E");
        },
        Continent2::Asia => { let a = 7; println!("{}", a); },
        Continent2::Africa => println!("Af"),
        Continent2::America => println!("Am"),
        Continent2::Oceania => println!("O"),
    }

    // 7.3
    // enum CardinalPoint { North, South, West, East }
    // let direction = CardinalPoint::South;
    // if direction == CardinalPoint::North { } // compile error

    // enum CardinalPoint { North, South, West, East }
    // if CardinalPoint::South < CardinalPoint::North { } // compile error

    // 7.4
    // #[allow(dead_code)]
    // enum CardinalPoint { North, South, West, East }
    // let direction = CardinalPoint::South;
    // match direction { // compile error
    //     CardinalPoint::North => println!("NORTH"),
    //     CardinalPoint::South => println!("SOUTH"),
    // }

    #[allow(dead_code)]
    enum CardinalPoint1 { North, South, West, East }
    let direction = CardinalPoint1::South;
    match direction {
        CardinalPoint1::North => println!("NORTH"),
        CardinalPoint1::South => println!("SOUTH"),
        _ => {},
    }

    // #[allow(dead_code)]
    // enum CardinalPoint { North, South, West, East }
    // let direction = CardinalPoint::South;
    // match direction {
    //     CardinalPoint::North => println!("NORTH"),
    //     _ => {}, // warning
    //     CardinalPoint::South => println!("SOUTH"),
    // }

    // 7.5
    match "value" {
        "val" => print!("value "),
        _ => print!("other "),
    }
    match 3 {
        3 => print!("three "),
        4 => print!("four "),
        5 => print!("five "),
        _ => print!("other "),
    }
    match '.' {
        ':' => print!("colon "),
        '.' => print!("point "),
        _ => print!("other "),
    }
    println!();

    // 7.6
    #[allow(dead_code)]
    enum Result1 {
        Success(u8),
        Failure(u16, char),
        Uncertainty,
    }
    // let outcome = Result::Success(1);
    let outcome = Result1::Failure(20, 'X');
    // let outcome = Result::Uncertainty;
    match outcome {
        Result1::Success(0) => println!("Result: 0"),
        Result1::Success(1) => println!("Result: 1"),
        Result1::Success(_) => println!("Result: other"),
        Result1::Failure(10, 'X') => println!("Error: 10 X"),
        Result1::Failure(10, _) => println!("Error: 10"),
        Result1::Failure(_, 'X') => println!("Error: X"),
        Result1::Failure(_, _) => println!("Error: other"),
        Result1::Uncertainty => {},
    }

    // 7.7
    #[allow(dead_code)]
    enum Result {
        Success(u8),
        Failure(u16, char),
        Uncertainty,
    }
    // let outcome = Result::Success(13);
    let outcome = Result::Failure(20, 'X');
    match outcome {
        Result::Success(0) => println!("Result: 0"),
        Result::Success(1) => println!("Result: 1"),
        Result::Success(n) => println!("Result: {}", n),
        Result::Failure(10, 'X') => println!("Error: 10 X"),
        Result::Failure(10, m) => println!("Error: 10 in module {}", m),
        Result::Failure(code, 'X') => println!("Error: n.{} X", code),
        Result::Failure(code, module) =>
            println!("Error: n.{} in module {}", code, module),
        Result::Uncertainty => {},
    }

    // 7.8
    #[allow(dead_code)]
    enum CardinalPoint3 { North, South, West, East }
    let direction3 = CardinalPoint3::South;
    println!("{}", match direction3 {
        CardinalPoint3::North => 'N',
        CardinalPoint3::South => 'S',
        _ => '*',
        // _ => {}, // compile error
    });

    // 7.9
    for n in -2..5 {
        println!("{} is {}.", n, match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative",
            _ => "plural",
        });
    }

    // 7.10
    #[allow(dead_code)]
    enum E {
        Case1(u32),
        Case2(char),
        Case3(i64, bool),
    }
    // let v = E::Case1(123);
    // let v = E::Case2('R');
    // let v = E::Case3(234, false);
    let v = E::Case3(1234, true);
    match v {
        E::Case3(n, b) => if b { println!("{}", n) },
        _ => {},
    }

    if let E::Case3(n, b) = v {
        if b { println!("{}", n) }
    }

    #[allow(dead_code)]
    enum E2 {
        Case1(u32),
        Case2(char),
    }
    let mut v = E2::Case1(0);
    while let E2::Case1(n) = v {
        print!("{}", n);
        if n == 6 { break; }
        v = E2::Case1(n + 1);
    }
    println!();
}