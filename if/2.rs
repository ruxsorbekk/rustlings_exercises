use std::io;

fn pickey_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else {
        "No thanks!"
    }
}
fn main() {
    println!("Enter the food name: ");
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read the line");
    let s: &str = s.trim();
    println!("{}", pickey_eater(s));
}
