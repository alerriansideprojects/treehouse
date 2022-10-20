pub struct Visitor {
    pub name: String,
    pub action: VisitorAction,
    pub age: i8,
}

pub enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    pub fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    pub fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the Treehouse, {}!", self.name),
            VisitorAction::Probation => println!("{} is now a probationary member!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}