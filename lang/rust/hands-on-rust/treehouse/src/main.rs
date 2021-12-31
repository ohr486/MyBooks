use std::io::stdin;

ststruct Visitor {
    name: String,
    greeting: String,
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();

        let visitor_list = vec![
            Visitor::new("Bert", "Hello Bert, enjoy your treehouse."),
            Visitor::new("Steve", "Hi Steve. Your milk is in the fridge."),
            Visitor::new("Fred", "Wow, who invited Fred?"),
        ];

        match known_visitor {
            Some(Visitor) => visitor.greet_visitor(),
            Nome => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{#?}", visitor_list);
}
