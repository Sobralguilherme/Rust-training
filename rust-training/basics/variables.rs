fn main() {
    let x = 5; // variável imutável
    let mut y = 10; // variável mutável
    println!("x = {}, y = {}", x, y);
    y += 5;
    println!("y após modificação = {}", y);
}
