use std;

#[derive(Debug, Clone)]
struct Person {
    firstname: String,
    lastname: String,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hello ! I'm {} {} !", self.firstname, self.lastname)
    }
}

fn main() {
    let person: Person = Person {
        firstname: String::from("Yannis"),
        lastname: String::from("Bikouta"),
    };

    println!("{}", person.clone());
    println!("{}", person.firstname);
}
