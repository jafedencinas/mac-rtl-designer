use std::io;

fn main() {
    println!("Enter a list of numbers separated by spaces:");

    // Read a line of input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input into words, parse them as numbers, and collect them into a vector
    let mut numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|word| word.parse().expect("Please enter valid numbers"))
        .collect();

    // Sort the numbers
    numbers.sort();

    // Print the sorted numbers
    println!("Sorted numbers: {:?}", numbers);
}

