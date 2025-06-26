//DAY 1: VARIABLES,MUTABILITY vs. IMMUTABILITY.

fn main() {
    println!(
        "Day 1: Understanding file and folder structures along with mutability and immutability"
    );
    let age = 21; //immutable variable i.e. its value cannot be changed throughout the program
    println!("My current age is: {age}");
    //if I reassign some other value to age variable it will show error

    let mut age2 = 22; //'mut' keyword makes variable mutable i.e. its value can be changed.
    println!("My age after 1 year: {age2}");
    age2 += 1; //value of age2 variable changed. It will show no error.
    println!("My age after 2 year: {age2}");
}
