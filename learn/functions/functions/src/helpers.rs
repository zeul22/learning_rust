pub mod greetings {
    pub fn get_full_name(first: &str, last: &str) -> String {
        let full_name: String = format!("{0} {1}", first, last);
        println!("My first name is {0} & last name is {1}", first, last);
        return full_name;
    }

    pub fn get_age(age: i8) {
        println!("Your age is {0}", age)
    }
}
