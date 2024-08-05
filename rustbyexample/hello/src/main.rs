#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("Hello, world!");
    println!("My name is Rahul Anand");
    println!("My name is Rahul Anand");
    println!("My name is Rahul Anand");
    let x = 9;
    println!("Is `x` 10 or 100 x={}", x);

    println!("{number:0<6}", number = 2);
    println!("{number:0>3}", number = 2);
    let name = "Rahul";
    let age = 23;
    let peter = Person { name, age };
    println!("{:#?}", peter)
}
