use crate::third::say_hello as say_hello_third;

pub fn say_hello (name: &str) {
    println!("Hello 1{}", name);

    say_hello_third(name);
}

pub mod second{
    pub mod third {
        pub fn say_hello (name: &str) {
            super::super::say_hello(name);
        }
    }
}