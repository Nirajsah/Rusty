#![allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Message {
    Hello(String),
}
fn main() {
    let player_direction: Direction = Direction::Up;
    let _hello_message = Message::Hello(String::from("Hello"));
    let message = "get";

    let est = match message {
        "get" => Message::Hello(message.to_string()),
        _ => Message::Hello("Default".to_string()),
    };

    println!("{:?}", est);

    match player_direction {
        Direction::Up => println!("Player is moving up"),
        Direction::Down => println!("Player is moving down"),
        Direction::Left => println!("Player is moving left"),
        Direction::Right => println!("Player is moving right"),
    }
}
