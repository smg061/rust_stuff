
#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color,
}

impl Person {
    pub fn print(self) -> String {
        format!("name = {}, age = {} has {} children", self.name, self.age, self.children)
    }
}
#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}
fn main() {
    let p = Person {
        name: "matt".to_string(),
        age: 34,
        children: 2,
        fave_color: Color::Red("it's red".to_owned())
    };
    let s = p.print();
    let c = Color::Red("hello".to_string()); 
    match c {
        Color::Red(s) => println!("it's red {}!", s),
        Color::Blue => println!("it's blue"),
        Color::Green => println!("it's green!"),
    }
    println!("{}", &s);
}
