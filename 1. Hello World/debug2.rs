#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Not pretty or regular print
    println!("Not Pretty:\n{:?}", peter);
    // Pretty print
    println!("Pretty:\n{:#?}", peter);
}