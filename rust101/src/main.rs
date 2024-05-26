fn main() {
    println!("Hello, world!");

    // basic variables
    let x: i64 = 12346;
    let y: &str = "Hello, This side Rahul Anand";
    let z = 25;

    println!("{}", x);
    println!("{}", z);
    println!("{}", y);

    println!("x:{} y:{} z:{}", x, y, z);

    let mut a: i8 = 10;

    println!("a:{}", a);
    a = 12;
    println!("a:{}", a);
    // Strings are complicated in rust as they change their size in runtime!

    let b: String = String::from("Hello from different type!");
    println!("{}", b);

    // Conditionals
    let is_even = false;
    if is_even {
        println!("It's an even number");
    } else {
        println!("It's an odd number");
    }

    // For Loop
    for i in 0..10 {
        print!("{} ", i)
    }
    println!();

    let words = String::from("My name is Rahul Anand");

    let mut answer = String::new(); //initializing

    // Getting the first word
    for char in words.chars() {
        answer.push(char);
        if char == ' ' {
            break;
        }
    }
    println!("{}", answer);
    let c: i32 = do_sum(4, 2);
    println!("{}", c);
}

// function
fn do_sum(a: i32, b: i32) -> i32 {
    return a + b;
}
