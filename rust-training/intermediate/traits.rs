trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    headline: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

fn main() {
    let article = Article {
        headline: String::from("Rust está em ascensão"),
        author: String::from("Jane Doe"),
        content: String::from("Rust é uma linguagem de programação segura e rápida."),
    };

    println!("Resumo do artigo: {}", article.summarize());
}
