pub struct User {
        pub name: String,
        pub age: u8,
        pub alive: bool
    }
    impl User {
        pub fn say_hello(&self, name: &str) {
            println!("Hello {}, my name is {}", name, self.name);
        }
    }