use std::collections::HashMap;

fn main() {
    let numbers = vec![7, 5, 6, 12, 1, 2, 3, 4, 5];
    let mean = find_mean(&numbers);
    let median = find_median(&numbers);
    let mode = find_mode(&numbers);

    println!(
        "porumai ... mean - {}, median - {}, mode - {}",
        mean, median, mode
    );

    let pig_latinify_words = vec!["apple", "first", "porumai", "wait", "and", "hope"];
    for word in pig_latinify_words {
        println!(
            "porumai ... {} => {}",
            word,
            pig_latinify(String::from(word))
        )
    }
}

fn find_mean(arr: &Vec<i32>) -> f32 {
    let mut total = 0;
    let size = arr.len() as f32;
    for a in arr {
        total = total + a;
    }
    println!("porumai ... meaning ??? {}, {} ", total, size);
    total as f32 / size
}

fn find_median(arr: &Vec<i32>) -> i32 {
    let mut sorted: Vec<i32> = Vec::new();

    // go through each element of the vector
    // and use binary_search to find out where to insert the element
    for elem in arr {
        let position = sorted.binary_search(&elem).unwrap_or_else(|x| x);
        sorted.insert(position, *elem);
    }

    let middle = ((sorted.len() as f32) / 2 as f32).round() as usize;
    println!("porumai ... array sorted {:?} - {}", sorted, middle);
    // sorted.get(middle)
    match sorted.get(middle - 1) {
        Some(i) => *i,
        _ => 0,
    }
}

fn find_mode(arr: &Vec<i32>) -> i32 {
    let mut mode_scores: HashMap<i32, i32> = HashMap::new();

    for elem in arr {
        let count = mode_scores.entry(*elem).or_insert(0);
        *count += 1;
    }

    // we would have the scores by now
    println!("porumai ... mode scores {:?}", mode_scores);
    let mut mode: i32 = 0;
    let mut max_score = 0;

    match arr.get(0) {
        Some(i) => {
            mode = *i;
        }
        _ => (),
    }

    // go through our hash map and find out
    // if some elem has score greater than our max score
    for (num, score) in &mode_scores {
        if *score > max_score {
            // update mode
            mode = *num;
            // update max score
            max_score = *score;
        }
    }

    mode
}

enum Letter {
    Vowel,
    Consonant,
}

// helper function to find out if a word starts with vowel or consonant
fn starting_with(word: &str) -> Letter {
    let vowels = String::from("aeiou").into_bytes();

    let bytes = String::from(word).into_bytes();
    let first_letter = bytes[0];

    match vowels.contains(&first_letter) {
        true => Letter::Vowel,
        false => Letter::Consonant,
    }
}

fn pig_latinify(original: String) -> String {
    let word = String::from(&original);

    let starts_with = starting_with(&word);

    match starts_with {
        // add '-hay' to the end
        // apple => apple-hay
        Letter::Vowel => format!("{}-hay", original),

        // move first letter and add '-<first letter>ay'
        // first => irst-fay
        Letter::Consonant => {
            // making copy of the word
            let mut word_copy = String::from(word);
            // take rest of the word
            // after this word will just have the first letter
            let rest = word_copy.split_off(1);
            let parsed = format!("{}-{}ay", rest, word_copy);
            // we need to take rest of the string
            parsed
        }
    }
}
