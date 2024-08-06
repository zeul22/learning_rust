pub mod helpers;
fn main() {
    println!("Hello, world!");
    helpers::condition_helpers::get_approval(true);
    println!("Enter your age:");
    let myinput: &mut String = &mut String::from("");
    std::io::stdin().read_line(myinput).expect("Failed to read line");
    let mut age = myinput.trim().parse::<i8>();

    // This Gives Error!
    // std::io::stdin().read_line(myinput).unwrap();
    // let age: i8 = myinput.replace("\n", "").parse()::<i8>().unwrap();
    // helpers::condition_helpers::age_limit(age);

    // Try && Catch thing
    //wherever user input is involved, it is suggested to write like this
    match age {
        Ok(mut valid_age) => {
            helpers::condition_helpers::age_limit(valid_age);
            while valid_age > 18 {
                println!("Reducing Age...");
                valid_age -= 1;
            }
        }
        Err(_) => println!("Please enter a valid age."),
    }

    // Loop
    let mut x: i8 = 5;
    while x > 0 {
        println!("{}", x);
        x -= 1;
    }

    // test_loop()
    test_loop();
    do_somethin();
}

fn test_loop() {
    let mut x: i8 = 0;
    loop {
        if x >= 5 {
            break;
        }
        println!("Hello from the Loop {0}", x + 1);
        x += 1;
    }
}

fn do_somethin() {
    let ages = [14, 15, 12, 20, 16];

    for value in ages {
        print!("Age:{0}, ", value);
        helpers::condition_helpers::age_limit(value);
        
    }
}
