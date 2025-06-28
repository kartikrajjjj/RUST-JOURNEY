use std::io;
fn main() {
    let mut buffer = String::new();
    println!("ENTER NUMBER: ");
    io::stdin().read_line(&mut buffer).expect("FAILED!!");

    let num: i32 = buffer.trim().parse().expect("PLEASE TYPE VALID INTEGER!");
    println!("SQUARE OF THIS NUMBER IS: {} ", num * num);
    //Taken input manually from user in this program
}
