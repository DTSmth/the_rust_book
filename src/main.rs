fn main() {
    enum IpAddr {
        V4 (String),
        V6 (String),
    }



    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message{
        fn call(&self){
            // do stuff
        }
    }

    let m = Message::Write(String::from("Hello, World!"));
    m.call();

    #[derive(Debug)]
    enum UsState {
        NorthCarolina,
        Virgina,
    }

    impl UsState {
        fn existed_in(&self, year: u16) -> bool {
            match self{
                UsState::NorthCarolina => year >= 1819,
                UsState::Virgina => year >= 1920,
            }
        }
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
                println!("lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            },
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("max is {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("max is {}", max);
    }

    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    fn describe_state_quarter(coin: Coin) -> Option<String> {
        let Coin::Quarter(state) = coin else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }



}
