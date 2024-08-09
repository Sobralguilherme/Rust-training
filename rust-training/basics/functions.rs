fn main() {
    println!("O resultado de 5 + 4 é: {}", add(5, 4));
    greet("Mundo");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn greet(name: &str) {
    println!("Olá, {}!", name);
}
