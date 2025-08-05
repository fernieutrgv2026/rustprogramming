fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [1,2,3,4,5,6,7,8,9,10];
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            if is_even(num) {
                println!("{} is even", num);
            } else {
                println!("{} is odd", num);
            }
        }
    }

    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("The sum of all the numbers = {}", sum);

    let mut max_value = numbers[0];
    let mut idx = 1;
    loop {
        if idx >= numbers.len() {
            break;
        }
        if numbers[idx] > max_value {
            max_value = numbers[idx];
        }
        idx += 1;
    }
    println!("The largest number in the array is {}", max_value);
}