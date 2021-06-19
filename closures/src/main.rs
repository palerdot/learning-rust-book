use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    let simulated_user_specified_value = 32;
    let simulated_random_value = 4;

    println!("Hello, world!");
    generate_workout(simulated_user_specified_value, simulated_random_value);
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let existing = self.value.get(&arg);

        match existing {
            Some(i) => *i,
            _ => {
                // calculate the value
                let v = (self.calculation)(arg);
                // update the hash map
                self.value.insert(arg, v);

                // return the calculated value
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_calculation = Cacher::new(|num| {
        println!("porumai ... expensive calculation");
        thread::sleep(Duration::from_secs(2));

        intensity
    });

    if intensity < 24 {
        println!(
            "porumai ... pushups => {}",
            expensive_calculation.value(intensity)
        );
        println!(
            "porumai ... situps => {}",
            expensive_calculation.value(intensity)
        );
    } else {
        if random_number == 4 {
            println!("porumai ... wait and hope ...");
        } else {
            println!(
                "porumai ... Run => {} kms ",
                expensive_calculation.value(intensity)
            );
        }
    }
}

#[test]
fn calculate_cache_with_different_values() {
    let mut c = Cacher::new(|x| x);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
