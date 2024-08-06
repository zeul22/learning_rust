pub mod closures {
    pub fn test_closures() {
        println!("Returning some text.....");
    }
    //Clos er instance

    pub fn closure_fn() {
        let add: impl Fn() = || println!("Adding things up");

        add();
    }
}
