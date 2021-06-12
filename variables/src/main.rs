fn main() {
    let mut x = 5;
    println!("porumai ... x is {} ", x);
    x = 6;
    println!("porumai ... x is {} ", x);

    greet("Arun");
}

fn greet(person: &str) {
    println!("porumai ... wait and hope !!! - {}", person);
}
