pub fn variable_print() {
    /*
    This function prints the value of a mutable variable.
    It demonstrates the concept of mutable variables in Rust.
    Mutable variables can be changed after they are initialized.
    */
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    /*
    This function prints the value of an immutable variable.
    It demonstrates the concept of immutable variables in Rust.
    Immutable variables cannot be changed after they are initialized.
    not be changed after they are initialized.
    */
    let y = 10;
    println!("The value of y is: {}", y);

    /*
    This is the method for declaring and assigning two variables in the same line.
    */

    let (a,b) = (1, 2);
    println!("value a is {}, value b is {b:?}", a);
    
    /*
    Even though let is an immutable variable, you can always redeclare the same variable and overwrite it. 
    Here, the “CONST” constants come in handy, where the value cannot be changed and the variable cannot be overwritten.
     */

    // let is overwritten

    let hello= "ciao";

    println!("{}", hello);

    let hello = "hello";

    println!("{}", hello);

    // const cannot be overwritten.

    const G: f64 = 9.81;

    println!("{}", G);



}
