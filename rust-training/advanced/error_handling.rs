fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Divisão por zero!"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let result = divide(4.0, 2.0);
    match result {
        Ok(quotient) => println!("O quociente é {}", quotient),
        Err(err) => println!("Erro: {}", err),
    }

    let result = divide(4.0, 0.0);
    match result {
        Ok(quotient) => println!("O quociente é {}", quotient),
        Err(err) => println!("Erro: {}", err),
    }
}
