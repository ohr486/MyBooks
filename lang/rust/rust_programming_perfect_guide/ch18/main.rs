// 18.4
fn f() {
    print!("f ");
    g();
    m::f();
    m::m::f();
}
fn g() { print!("g "); }
mod m {
    pub fn f() {
        print!("1.f(m::f()) ");
        g();
        m::f();
        super::g();
    }
    fn g() { print!("1.g(m::g()) "); }
    pub mod m {
        pub fn f() {
            print!("2.f(m::m::f()) ");
            g();
            super::g();
            super::super::g();
            crate::g();
        }

        fn g() { print!("2.g(m::m::g()) ");
        }
    }
}

fn main() {
    // 18.1
    println!("{} {}",
        "abcd".to_string(),
        std::string::ToString::to_string("abcd"));

    println!("{} {}",
        [1, 2, 3, 4].len(),
        <[i32]>::len(&[1, 2, 3, 4]));

    let mut v1 = vec![0u8; 0];
    let mut v2 = vec![0u8; 0];
    v1.push(7);
    Vec::push(&mut v2, 7);
    println!("{:?} {:?}", v1, v2);

    // 18.2
    struct Person {
        personal_names: String,
        family_names: String,
    }
    fn naming(p: Person) -> String {
        format!("{} {}",
            p.personal_names,
            p.family_names)
    }
    let person = Person {
        personal_names: "John".to_string(),
        family_names: "Doe".to_string(),
    };
    println!("{}", naming(person));

    // println!("{}", person.naming()); // compile error

    struct Person2 {
        personal_names: String,
        family_names: String,
    }
    impl Person2 {
        fn naming(self) -> String {
            format!("{} {}",
                self.personal_names,
                self.family_names)
        }
    }
    let person = Person2 {
        personal_names: "John".to_string(),
        family_names: "Doe".to_string(),
    };
    println!("{}", person.naming());

    struct Person3 (String, u32);
    #[allow(dead_code)]
    enum Visibility { Visible, Hidden, Collapsed }
    impl Person3 {
        fn age(&self) -> u32 {
            self.1
        }
    }
    impl Visibility {
        fn is_not_visible(&self) -> bool {
            match self {
                Visibility::Visible => false,
                _ => true,
            }
        }
    }
    print!("{} ", Person3("John".to_string(), 30).age());
    println!("{}", Visibility::Collapsed.is_not_visible());

    // 18.3
    struct Person4 {
        personal_names: String,
        family_names: String,
    }
    impl Person4 {
        fn new() -> Self {
            Self {
                personal_names: String::new(),
                family_names: String::new(),
            }
        }
        fn naming(&self) -> String {
            format!("{} {}",
                self.personal_names,
                self.family_names)
        }
    }
    impl Person4 {
        fn set_personal_names(&mut self, new_name: String) {
            self.personal_names = new_name;
        }
    }
    let mut person = Person4::new();
    print!("[{}] ", person.naming());
    person.personal_names = "John".to_string();
    person.family_names = "Doe".to_string();
    print!("[{}] ", person.naming());
    person.set_personal_names("Jane".to_string());
    println!("[{}]", person.naming());

    // 18.4
    /*
    mod routines {
        fn f() -> u32 { g() }
        fn g() -> u32 { 123 }
    }
    println!("{}", f()); // compile error
    */

    mod routines {
        pub fn f() -> u32 { g() }
        fn g() -> u32 { 123 }
    }
    println!("{}", routines::f());

    f();
    println!();

    // 18.5
    fn f1(x: f32) -> f32 { x }
    fn f2(x: f32) -> f32 { x }
    let a: f32 = 2.3;
    let b: f32 = 3.4;
    println!("{} {}", f1(a), f2(b));

    type Number = f32;
    fn f1_2(x: Number) -> Number { x }
    fn f2_2(x: Number) -> Number { x }
    let a: Number = 2.3;
    let b: Number = 3.4;
    println!("{} {}", f1_2(a), f2_2(b));

    type Number2 = f32;
    let a: Number2 = 2.3;
    let _b: f32 = a;
}