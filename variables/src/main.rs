fn main() {
    let mut x = 5;
    // variable can be edited
    println!("The value of x is {x}");
    x = 6; 
    println!("The value of x is {x}");



    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
        // this version of y would be true here
    }
    // the previous y = y + 1 would be true here as it is outside the scope of y = y * 2
    println!("The value of y is: {y}");

    // originally a str variable
    let spaces = "   ";
    // now a num variable
    let spaces = spaces.len();
    println!("{spaces}")

}
