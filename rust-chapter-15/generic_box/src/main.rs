use std::ops::Deref;

use crate::List::{Cons, Nil};

fn main() {
    {
        let b = Box::new(5);
        println!("b = {}", b);
    }

    {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    {
        let x = 5;
        let y = MyBox::new(x);
    
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);

        // Same but no need 
        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]);
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}