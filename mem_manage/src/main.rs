// Memory Management is the critical aspect of why Rust is so fast!
fn main() {
    // Features that help memory management
    // 1. Immutability
    let mut x: i32 = 25;
    println!("{}", x); // will give warning of mutable variable
                       // x = 500;
                       // println!("{}", x);

    // 2. Stack & Heap
    // Stack:Faster allocation & deallocation -stores- Number, Boolean, Fixed size Array
    // Heap: Slower-stores-Vectors, Strings

    stack_fn(); //  uses stack memory
    heap_fn(); //  uses heap memory
    update_string(); //  changes size of variable at runtime

    // 3. Ownership : How a Rust program manages memory, especifically heap.
    let x = 1; // crated on stack
    let y = 3; // created on stack
    println!("{}", sum(x, y));
    println!("Hello, world!");

    // Borrowing terminology
    // let my_string = String::from("hello");
    // takes_ownership(my_string);
    // println!("{}", my_string); // This line would cause a compile error because ownership has been moved.

    // fixes to this

    // Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1); // Compiles now
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}
fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!(
        "Capacity:{}, Length:{}, pointer:{:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    println!("Before update: {}", s);

    // Append some text to the string
    s.push_str(" and some additional text");
    println!(
        "Capacity:{}, Length:{}, pointer:{:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
    println!("After update: {}", s);
}
