fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("Número é divisível por 4");
    } else if number % 3 == 0 {
        println!("Número é divisível por 3");
    } else {
        println!("Número não é divisível por 4 ou 3");
    }

    let mut count = 0;
    loop {
        println!("Contagem: {}", count);
        if count == 5 {
            break;
        }
        count += 1;
    }
}
