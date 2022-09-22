use std::io::stdin;
mod visitor;

use visitor::{Visitor, VisitorAction};

fn get_name() -> String {
    let mut user_name = String::new();

    stdin()
        .read_line(&mut user_name)
        .expect("Failed to read line.");

    user_name.trim().to_lowercase()
}
fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 45),
        Visitor::new(
            "steve",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-Free milk is in the fridge."),
            },
            15,
        ),
        Visitor::new("fred", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello, what is your name? (Leave Empty and press ENTER to quit)");
        let user_name = get_name();

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == user_name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if user_name.is_empty() {
                    break;
                }

                println!(
                    "{} is not on the visitor list. Adding to list for next time.",
                    user_name
                );
                visitor_list.push(Visitor::new(&user_name, VisitorAction::Probation, 0));
            }
        }
    }
}
