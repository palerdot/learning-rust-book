use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter number to calculate fibonacci");

    io::stdin()
        .read_line(&mut n)
        .expect("Please provide the number to calculate fibonacci");

    let n = n.trim().parse().expect("Please provide a number");

    let fib = find_nth_fibonacci(n);
    println!("porumai ... {}th fibonacci is - {}", n, fib);
}

fn find_nth_fibonacci(size: u128) -> u128 {
    let mut current = 1;
    let mut previous = 1;
    let mut fib = 1;

    for number in 0..size {
        if number > 1 {
            fib = current + previous;
            previous = current;
            current = fib;
        }
    }

    fib
}
