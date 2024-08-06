pub mod condition_helpers {
    pub fn get_approval(approval: bool) {
        if approval {
            println!("You got the approval");
        } else {
            println!("You are not approved");
        }
    }

    pub fn age_limit(age: i8) {
        if age > 100 {
            println!("Are you Wolverine?");
        } else if age > 20 {
            println!("You are allowed to do things!");
        } else if age > 15 {
            println!("You are only allowed to drive");
        } else {
            println!("You are under-age kiddo")
        }
    }
}
