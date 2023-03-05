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