struct Cat {
    name: String,
    age: f64,
}

pub fn main() {
    let cat = Cat {
        name: String::from("Furry McFurson"),
        age: 3.5,
    };

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;

    println!("{} is {} years old", cat.name, cat.age);
}