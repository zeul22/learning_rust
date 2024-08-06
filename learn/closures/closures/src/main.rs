// Anonymous function=Closures

pub mod closures;
pub mod helpers;

fn main() {
    println!("We will be working anonymous functions, a.k.a closures!");

    closures::closures::test_closures();
    closures::closures::get_approval()
}
