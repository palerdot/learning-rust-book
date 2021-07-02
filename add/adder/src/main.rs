use add_one;

fn main() {
    let num = 5;
    println!(
        "porumai ... plus one of {} is {}",
        num,
        add_one::add_one(num)
    );
}
