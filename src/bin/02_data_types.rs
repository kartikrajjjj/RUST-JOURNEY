fn main() {
    // ─── Numbers ───

    let coins: u8 = 255; // unsigned 8-bit → only positive numbers (0 to 255)
    let views: u32 = 1_000_000; // unsigned 32-bit → large positive numbers

    let temperature: i32 = -10; // signed 32-bit → can be negative or positive
    let score: i32 = 42; // typical integer value

    let price_f32: f32 = 99.99; // 32-bit float → decimal but not very precise
    let balance_f64: f64 = 123456.789; // 64-bit float → more precise decimals

    // ─── Booleans ───

    let is_logged_in: bool = true; // true or false
    let is_night: bool = false;

    // ─── Characters ───

    let first_letter: char = 'R'; // single character
    let emoji: char = '🔥'; // emojis are valid too

    // ─── Strings ───

    let name: &str = "Kartik"; // string slice (read-only, fixed)
    let message = String::from("Welcome"); // growable string (owned)

    // ─── Arrays ───

    let marks: [i32; 5] = [88, 76, 90, 95, 82]; // fixed size array

    // ─── Tuples ───

    let user: (i32, &str, bool) = (1, "Kartik", true); // different types in one value

    // ─── Print Everything ───

    println!("Coins: {}", coins);
    println!("Views: {}", views);
    println!("Temperature: {}", temperature);
    println!("Score: {}", score);
    println!("Price (f32): {}", price_f32);
    println!("Balance (f64): {}", balance_f64);
    println!("Logged In?: {}", is_logged_in);
    println!("Is Night?: {}", is_night);
    println!("First Letter: {}", first_letter);
    println!("Emoji: {}", emoji);
    println!("Name: {}", name);
    println!("Message: {}", message);
    println!("Marks: {:?}", marks);
    println!("User Tuple: {:?}", user);
}
