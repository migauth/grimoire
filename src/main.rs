use std::io;

fn main() {
    let mut choice = String::new();

    io::stdin().read_line(&mut choice);

    println!("You chose {choice}")
}
