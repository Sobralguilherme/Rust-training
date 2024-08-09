use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?;
    file.write_all(b"Olá, mundo!")?;

    let mut file = File::open("output.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Conteúdo do arquivo: {}", contents);

    Ok(())
}
