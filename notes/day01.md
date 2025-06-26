📅 Day 01: Rust Basics – Input/Output, Variables & Shadowing
📥 Input & 📤 Output in Rust
Rust’s standard I/O lives in the std::io module.
For console programs, the most common actions are:

Printing output using println!, print!, or eprintln!

Taking input using io::stdin().read_line(&mut buffer)

🖨️ 1. Printing Output
println! – Adds a newline after the text
println!("Hello, world!");
println!("Number: {}", 21);
println!("Name: {name}", name = "Kartik");

print! – Does not add a newline
print!("Enter a value: ");

eprintln! – Prints to standard error
eprintln!("Invalid input, please try again.");

format! – Returns a String instead of printing
let greeting = format!("Hi, {}!", user_name);

⌨️ 2. Taking Input from User
Sample code:

use std::io;
fn main() {
let mut buffer = String::new();
print!("Enter a number: ");
io::stdin().read_line(&mut buffer).expect("Failed to read line");
let num: i32 = buffer.trim().parse().expect("Please enter a valid number");
println!("Square: {}", num \* num);
}

🧠 Key Points
read_line stores user input (with newline) into the buffer.

trim() removes unwanted whitespaces like \n.

parse() converts the string into a number or other type.

Use .expect() or match to handle invalid input properly.

📘 Variables in Rust
Variables are immutable by default.

Use mut to make a variable mutable.

let age = 21; → Immutable
let mut score = 90; → Mutable
score += 10;
println!("Score: {}", score);

🔄 Shadowing in Rust
Shadowing lets you redeclare the same variable name with new values or even different types.

Example:

fn main() {
let x = 5;
println!("x: {x}");
let x = x + 1;
{
let x = x \* 5;
println!("Inner x: {x}");
}
println!("x after inner block: {x}");
}

Output:
x: 5
Inner x: 30
x after inner block: 6

✅ Why Shadowing Is Useful
Allows clean transformations without needing mut

Keeps code safe and immutable by default

Helpful for formatting, scoping, or type conversion
