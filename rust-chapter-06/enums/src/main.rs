enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    
        let m = Message::Write(String::from("hello"));
        m.call();
    
        let some_number = Some(5);
        let some_char = Some('e');
    
        let absent_number: Option<i32> = None;
    
        let x: i8 = 5;
        let y: Option<i8> = Some(5);
    
        // let sum = x + y; // Invalid
        let sum = x + y.unwrap();
    }

    {
        let value = value_in_cents(Coin::Quarter(UsState::Alaska));
        println!("{}", value)
    }

    {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);

        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }

        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }

        fn add_fancy_hat() {}
        fn remove_fancy_hat() {}
        fn move_player(num_spaces: u8) {}
        fn reroll() {}
    }

    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => (),
        }

        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }
    }
}

fn route(ip_kind: IpAddr) {}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}