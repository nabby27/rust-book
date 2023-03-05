fn main() {
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
    
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let x = 5;
        let y = x;
    
        println!("x = {}, y = {}", x, y);
    }

    {
        let s = String::from("hello");
        takes_ownership(s);

        // println!("Inside scope block: {}", s); // Invalid 

        let x = 5;
        makes_copy(x);
        println!("Inside scope block: {}", x); 
    }

    {
        let s1 = gives_ownership();

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);
    }

    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);
    
        println!("The length of '{}' is {}.", s2, len);
    }

    {
        let s1 = String::from("hello");

        let len = calculate_length2(&s1);
    
        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");

        change(&mut s);
    }

    {
        let mut s = String::from("hello");

        let r1 = &mut s;
        // let r2 = &mut s; // Invalid
    
        println!("{}", r1);
    }

    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        }
    
        let r2 = &mut s;
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        // let r3 = &mut s; // Invalid
    
        println!("{}, {}", r1, r2);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
     
        let r3 = &mut s;
        println!("{}", r3);
    }

    {
        let reference_to_nothing = dangle();
    }

    {
        let mut s = String::from("Hello World!");
        let w = first_word(&s);

        // s.clear(); // Invalid

        println!("the first word of sentence '{}' is '{}'", s, w);

        let my_string = String::from("hello world");
        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        let word = first_word(&my_string);
    
        let my_string_literal = "hello world";
        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);
        let word = first_word(my_string_literal);
    }

    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[2..3];
        println!("{:?}", slice);
    }

}

fn takes_ownership(some_string: String) {
    println!("Inside function takes_ownership: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Inside function makes_copy: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}