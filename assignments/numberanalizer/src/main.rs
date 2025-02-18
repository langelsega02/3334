// Function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers of your choice
    let numbers = [3, 10, 15, 7, 8, 25, 40, 33, 20, 18];

    // Iterate through the array and perform checks
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    // Calculate the sum of all numbers in the array using a while loop
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("\nThe sum of all numbers in the array is: {}", sum);

    // Find and print the largest number in the array using a loop
    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("The largest number in the array is: {}", largest);
}
