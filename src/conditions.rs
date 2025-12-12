pub fn condition() -> bool {
    let x = 5;

    // This if statement checks if x equals 5, and if true, executes the println! macro to print "x is five!" to the console.
    if x == 5 {
        println!("x is five!");
    }

    // This conditional checks if x equals 5, printing "x is five!" if true, otherwise it prints "x is not five :(" as the fallback.
    if x == 5 {
        println!("x is five!");
    } else {
        println!("x is not five :(");
    }

    // This is a multi-branch conditional statement in Rust that checks the value of x and executes the corresponding code block based on which condition evaluates to true.
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    // This if expression in Rust assigns either 10 or 15 to y based on the condition x == 5, demonstrating how if can be used as an expression that returns a value from each branch, with the result bound to a variable.
    let y = if x == 5 { 10 } else { 15 };
    println!("Y: {}", y);

    return true;
}
