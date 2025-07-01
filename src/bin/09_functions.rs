fn main() {
    // Calling a simple function that just prints "Hello"
    print_hello();

    // Calling a function with parameters, which prints the sum
    add_with_parameters(5, 6);

    // Calling a function that returns a value, and storing it
    let value = add_with_return(10, 15);
    println!("VALUE OF add_with_return is: {} ", value);

    // Calling a function to check if a number is even
    let even_or_odd = is_even(10);
    println!("{}", even_or_odd);
}

// Function with no input or output, just prints a message
fn print_hello() {
    println!("Hello");
}

// Takes two numbers, adds them, and prints the result
fn add_with_parameters(x: i32, y: i32) {
    println!("VALUE OF add_with_parameters() is: {} ", x + y);
}

// Takes two numbers, adds them, and returns the result
fn add_with_return(x: i32, y: i32) -> i32 {
    x + y // No semicolon = return this value
}

// Checks if a number is even, returns true/false
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        println!("NUMBER IS EVEN");
        return true;
    }
    println!("NUMBER IS NOT EVEN");
    false
}
