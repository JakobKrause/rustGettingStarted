#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize=1,
    Move=2,
    Echo=3,
    ChangeColor=4,
    Quit=5,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
