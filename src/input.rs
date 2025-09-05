/*
To use libraries, we can write ‘use’ followed immediately by the name of the library.
In Rust, std is the standard library where we find Rust's native libraries.

We can use ‘::’ as a path to access an internal feature.
For example, from a library to a module, or from a module to a function or method.
*/
use std::io;


pub fn new_input() {
    let mut input = String::new();

    /*
    In Rust the operator :: is used to access items by name inside modules or types, for example to call an associated
    function like String::new, to use a constant like Option::None, or to reach something inside a namespace like std::io::stdin.
    On the other hand the operator . is used on an actual value to call its methods or to access the fields of a struct.
    For example after creating a string you can call s.push_str("hello") on it, or if you have a struct p you can read its field
    with p.x. In short :: is for finding something in a module or type, while . is for working with the value you already have.
     */

    /*
    In this line io::stdin() uses :: to access the function stdin inside the module io, then .read_line(&mut input) calls the method read_line on the value returned by stdin(), 
    and finally .expect("failed to read line") is another method call that either gives back the result or prints the error message if reading fails.
    */

    io::stdin().read_line(&mut input).expect("failed to read line");

    println!("{}", input)
}