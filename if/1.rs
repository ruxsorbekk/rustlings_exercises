use std::io;

fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else if a == b {
        a
    } else {
        b
    }
}
fn main() {
    println!("Enter a: ");
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read the line");
    let a: i32 = a.trim().parse().expect("Not a number");

    println!("Enter b: ");
    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read the line");
    let b: i32 = b.trim().parse().expect("Not a number");
    println!("Result = {}", bigger(a, b));
}
