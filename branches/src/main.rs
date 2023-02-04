use std::{thread, time};

fn main() {
    let one_sec = time::Duration::from_millis(1000);

    for number in (1..11).rev() {
        println!("{number}!");
        thread::sleep(one_sec);
    }
    println!("LIFTOFF!!!");
}
