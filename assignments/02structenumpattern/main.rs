fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    let mut curr_num = low;
    while curr_num <= high {
        *total += curr_num;
        curr_num += step;
    }
}

fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut max_word = String::new();
    let mut max_count = 0;

    for (i, &word) in words.iter().enumerate() {
        let mut count = 0;
        for &w in &words {
            if w == word {
                count += 1;
            }
        }
        if count > max_count {
            max_word = word.to_string();
            max_count = count;
        }
    } 
    (max_word, max_count)
}


fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result); // i don't think 35 is possible, best i got was 38

    let text = "Peter Piper picked a peck of pickled peppers, a peck of pickled peppers Peter Piper picked yes Peter good job!";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}