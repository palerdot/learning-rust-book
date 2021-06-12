fn main() {
    let mut s = String::from("porumai ... ");
    s.push_str("wait and hope !!!");
    println!("Hello, world! {} ", s);

    let s1 = String::from("porumai");
    // take ownership of string
    takes_ownership(s1);

    let x = 5;

    make_copy(x);
    // our s1 is invalid now since it is borrowed by takes_ownership
    // println!("{}", s1);
    give_and_take_example();
    references_example();
    let word = get_first_word(&s);
    // this will error out since we can have an mutable operation for
    // which there is an immutable reference just above (race conditions prevention!!!)
    // s.clear();
    println!("first word is {}", word);
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // if empty space slice the string till this index
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn references_example() {
    let s1 = String::from("porumai ... ");
    let len = calculate_length(&s1);
    println!("len of {} is {} ", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("string ownership {}", some_string);
}

fn make_copy(x: i32) {
    println!("integer copy {}", x);
}

fn give_and_take_example() {
    let s1 = gives_ownership();
    let s2 = String::from("wait and hope !!!");
    let s3 = takes_and_gives_back_ownership(s2);

    println!("s1 - {}, s3 - {}", s1, s3)
}

fn gives_ownership() -> String {
    let some_string = String::from("porumai");

    // return some_string and ownership is transferred
    some_string
}

fn takes_and_gives_back_ownership(some_string: String) -> String {
    // some_string comes into ownership of this function
    // and ownership is transferred back
    some_string
}
