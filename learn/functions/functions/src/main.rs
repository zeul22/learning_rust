pub mod helpers;

fn test_function() {
    let x = 22.25;
    let y = (x as i32) + 10;
    println!("{:?}", y)
}
fn main() {
    println!("Hello, world!");
    test_function();

    // My trials
    // let mut name = String::from("Rahul");
    // println!("{}", name);
    // name = String::from("Kumar");
    // greet(name.clone());
    // println!("{}", name)
    let full_name: String = helpers::get_full_name("Rahul", "Anand");

    println!("Hello from {0}", full_name);
    helpers::get_age(40);
}

#[allow(dead_code)]
fn greet(name: String) {
    println!("Hello! {}", name);
}
// #[allow(dead_code)]
