fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("O maior número é {}", result);

    let chars = vec![y, m, a, q];
    let result = largest(&chars);
    println!("O maior caractere é {}", result);
}
