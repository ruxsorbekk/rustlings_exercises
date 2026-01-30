use rand::Rng;

pub fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let mut rng = rand::rng();
    let a: Vec<i32> = (0..100).map(|_| rng.random_range(1..=10)).collect();

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}