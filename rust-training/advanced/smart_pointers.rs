use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Contagem de referência de a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Contagem de referência de a após b = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("Contagem de referência de a após c = {}", Rc::strong_count(&a));
}
