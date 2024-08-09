# Rust Training

Este repositório é destinado ao treinamento e aprimoramento de habilidades em Rust. Contém uma série de exercícios que cobrem desde conceitos básicos até tópicos avançados.

## Estrutura do Projeto

```plaintext
rust-training/
├── README.md
├── basics/
│   ├── hello_world.rs
│   ├── variables.rs
│   ├── control_structures.rs
│   ├── ownership_borrowing.rs
│   └── functions.rs
├── intermediate/
│   ├── structs_enums.rs
│   ├── error_handling.rs
│   ├── collections.rs
│   ├── lifetimes.rs
│   └── generics.rs
└── advanced/
    ├── concurrency.rs
    ├── file_io.rs
    ├── smart_pointers.rs
    └── collections.rs
 ```

### Diretórios e Arquivos

- **basics/**: Contém exercícios básicos para introduzir conceitos fundamentais de Rust.
  - `hello_world.rs`: Primeiro programa em Rust.
  - `variables.rs`: Introdução a variáveis e mutabilidade.
  - `control_structures.rs`: Uso de estruturas de controle como `if`, `else`, e `match`.
  - `ownership_borrowing.rs`: Conceitos de posse, empréstimo, e regras de tempo de vida.
  - `functions.rs`: Definição e chamada de funções.

- **intermediate/**: Introduz conceitos intermediários, como estruturas, enumerações e manipulação de erros.
  - `structs_enums.rs`: Uso de estruturas (`structs`) e enumerações (`enums`).
  - `error_handling.rs`: Tratamento de erros com `Result` e `Option`.
  - `collections.rs`: Utilização de coleções como `Vec`, `HashMap`, e `HashSet`.
  - `lifetimes.rs`: Explicação sobre tempos de vida (`lifetimes`) e como usá-los.
  - `generics.rs`: Introdução a genéricos e como eles promovem a reutilização de código.

- **advanced/**: Explora tópicos avançados, incluindo concorrência, I/O de arquivos, e smart pointers.
  - `concurrency.rs`: Conceitos de concorrência usando threads.
  - `file_io.rs`: Operações de entrada e saída de arquivos.
  - `smart_pointers.rs`: Uso de smart pointers como `Box`, `Rc`, e `RefCell`.
  - `collections.rs`: Uso avançado de coleções padrão.

## Como Usar

1. Clone este repositório:

    ```bash
    git clone https://github.com/Sobralguilherme/Rust-training
    ```

2. Navegue até o diretório do projeto:

    ```bash
    cd rust-training
    ```

3. Execute os arquivos `.rs` usando o comando `rustc` ou diretamente com `cargo run` se você criar um projeto Cargo para cada exercício.

    ```bash
    rustc basics/hello_world.rs
    ./hello_world
    ```
