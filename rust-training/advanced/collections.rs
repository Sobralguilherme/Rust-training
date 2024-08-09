use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // Vetor (Vec)
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vetor: {:?}", v);

    // Acessando elementos
    let third = v.get(2);
    match third {
        Some(x) => println!("O terceiro elemento é {}", x),
        None => println!("Não há um terceiro elemento."),
    }

    // HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Azul"), 10);
    scores.insert(String::from("Vermelho"), 50);

    // Atualizando valores
    scores.insert(String::from("Azul"), 25);
    println!("Pontuações: {:?}", scores);

    // Iterando sobre o HashMap
    for (key, value) in &scores {
        println!("Equipe {} tem {} pontos", key, value);
    }

    // HashSet
    let mut books = HashSet::new();
    books.insert("O Hobbit");
    books.insert("O Senhor dos Anéis");
    books.insert("O Hobbit"); // não será adicionado novamente
    println!("Livros: {:?}", books);
}
