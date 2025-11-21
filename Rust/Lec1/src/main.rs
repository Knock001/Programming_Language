use lec1::Person;
use lec1::greet;

fn main() {
    greet("Rust");
    let p = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{} is {} years old", p.name, p.age);
}
