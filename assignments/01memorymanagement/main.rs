const FREEZE_POINT_FAHREN: f64 = 32.0;

fn f_to_c(f: f64) -> f64 {
    (f - FREEZE_POINT_FAHREN) * (5.0/9.0)
}


fn c_to_f(c: f64) -> f64 {
    c * (9.0/5.0) + FREEZE_POINT_FAHREN
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main () {
    // Assignment 1
    let mut temp_f: f64 = 45.0;
    let temp_c = f_to_c(temp_f);
    println! {
        "Temperature in celcius = {}", temp_c
    };
    for i in 1..=5 {
        let curr_fahren = temp_f + i as f64;
        let curr_celc = f_to_c(curr_fahren);
        println! {
            "Temperature in celcius = {}", curr_celc
        };
    }

    // Assignment 2
    let numbers = [1,2,3,4,5,6,7,8,9,10];
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{} is Fizzbum", num);
        } else if num % 3 == 0 {
            println!("{}: is Fizz", num);
        } else if num % 5 == 0 {
            println!("{} is Buzz", num);
        } else {
            println!("{} is not Fizzbum, Fizz, or Buzz", num);
        }
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }
    }

    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("The sum of all the numbers equals {}", sum);

    let mut max_val = numbers[0];
    let mut ind = 1;
    loop {
        if ind < numbers.len() {
            break;
        }
        if numbers[ind] > max_val {
            max_val = numbers[ind]
        }
        ind += 1;
    }
    println!("The biggest number of them all is {}", max_val);

    // Assignment 3
    let mut sec = 32;
    let mut num_guesses = 0;
    let mut adivinanza = 8; // adivinanza is guess in spanish

    loop {
        num_guesses += 1;
        let result = check_guess(guess, sec);
        if result == 0 {
            println!("The secret number was indeed {}!", sec);
            break;
        } else if result == 1 {
            println!("You guessed too high!");
        } else {
            println!("You guessed too low!");
        }
        adivinanza += 8; 
    }

    println!("The amount of guesses you needed was {}. Good job!", num_guesses);

}

}