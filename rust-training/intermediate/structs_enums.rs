struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("A área do retângulo é: {} pixels quadrados.", rect.area());

    let msg = Message::Write(String::from("Olá"));
    match msg {
        Message::Quit => println!("Recebido comando Quit"),
        Message::Move { x, y } => println!("Movendo para ({}, {})", x, y),
        Message::Write(text) => println!("Mensagem recebida: {}", text),
        Message::ChangeColor(r, g, b) => println!("Mudando cor para RGB({}, {}, {})", r, g, b),
    }
}
