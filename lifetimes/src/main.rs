fn main() {
    {
        let str1 = String::from("porumai ...");
        {
            let str2 = String::from("wait and hope !!!");
            let l = longest(&str1, &str2);
            println!("porumai ... longest => {} ", l);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
