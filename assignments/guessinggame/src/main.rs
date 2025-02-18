// Function to check the guess
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess
    } else if guess > secret {
        1 // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    // Declare a mutable variable to store the secret number
    let secret_number = 42; // You can change this number to any secret number you'd like

    // Initialize variables
    let mut guess;
    let mut guess_count = 0;

    // Start the loop for the guessing game
    loop {
        // Simulate a random guess (for the sake of example, we manually change guesses here)
        // This is where you would normally prompt for user input, but here we simulate with hardcoded guesses
        guess_count += 1;
        guess = match guess_count {
            1 => 10,  // First guess is too low
            2 => 30,  // Second guess is too low
            3 => 50,  // Third guess is too high
            4 => 42,  // Correct guess
            _ => break, // If we reach more than 4 guesses, exit the loop (just for safety)
        };

        // Call the check_guess function
        let result = check_guess(guess, secret_number);

        // Use if-else expression to print feedback on the guess
        if result == 0 {
            println!("Correct! The secret number was: {}", secret_number);
            break; // Exit the loop if the guess is correct
        } else if result == 1 {
            println!("Your guess of {} was too high!", guess);
        } else {
            println!("Your guess of {} was too low!", guess);
        }
    }

    // After the loop ends, print how many guesses it took
    println!("It took you {} guesses to find the correct number.", guess_count);
}
