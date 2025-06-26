fn main() {
    let x = 5; //initial value of 'x'.
    println!("THE VALUE OF X: {x}");

    let x = x + 1; //shadowing initial value of 'x'.
    {
        let x = x * 5;
        println!("THE VALUE OF X IN THE INNER SCOPE: {x}");
    }
    println!("THE VALUE OF X: {x}");
}
