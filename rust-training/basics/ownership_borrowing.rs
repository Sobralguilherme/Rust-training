fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}", s1); // Isso causaria um erro de compilação
    println!("{}", s2);

    let s3 = String::from("world");
    let len = calculate_length(&s3); // Passando referência
    println!("O comprimento de '{}' é {}", s3, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
