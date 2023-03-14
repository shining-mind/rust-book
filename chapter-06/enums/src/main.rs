use std::process::exit;

fn main() {
    let a: Option<u8> = Some(3);
    let b: Option<u8> = None;
    println!("{:?}", a.map(|x| x * 3)); 
    println!("{:?}", b.map(|x| x * 3)); 
    ip_example();
    message_example();

    
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn ip_example() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("{:?}, {:?}", home, loopback);
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Write(str) => println!("Write message: {str}"),
            Message::Move { x, y } => println!("Moving to {x}, {y}"),
            Message::ChangeColor(r, g, b) => println!("New color: ({r}, {g}, {b})"),
            Message::Quit => exit(0),
        }
    }
}

fn message_example() {
    random_quit();
    let write = Message::Write(String::from("hello"));
    write.call();

    random_quit();
    let color = Message::ChangeColor(255, 255, 255);
    color.call();

    random_quit();
    let mv = Message::Move { x: 3, y: 4 };
    mv.call();
}

fn random_quit() {
    let value = rand::random::<f32>();
    println!("Got chance to exit: {value}");
    if value > 0.5 {
        println!("Exiting");
        let quit = Message::Quit;
        quit.call();
    }
}