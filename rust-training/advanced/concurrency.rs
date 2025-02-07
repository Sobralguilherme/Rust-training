use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread número {} está executando!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Thread principal número {} está executando!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
