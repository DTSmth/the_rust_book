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
}
