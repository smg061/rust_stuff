#[derive(Debug)]
pub enum Res<T,E> {
    Thing(T),
    Error(E),
}



fn main () {
    let a = divide(4,5);

    let b = divide(10,0);

    match b {
        Res::Thing(v) => print!("{}", v),
        Res::Error(s) => println!("{}", &s)
    }
    match a {
        Res::Thing(v) => print!("{}", v),
        Res::Error(s) => println!("{}", &s)
    }

}

fn divide(a: i32, b:i32)-> Res<i32, String> {

    if b == 0 {
        return Res::Error("Cannot divide by zero".to_string());
    }

    Res::Thing(a/b)
}