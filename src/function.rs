/*
In this function, we have used the word ‘pub’, which indicates that this function is public throughout the entire project. Inside the round brackets, we have passed two variables called parameters.
With the syntax ‘->’, we indicate that the function returns a value.
*/

pub fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

/* The special `!` type (never type) marks functions that never return.
Typical cases: `panic!()`, infinite loops, or `std::process::exit()`.
Since `!` represents "no value", the compiler can treat it as any type. */

pub fn hello() -> ! {
    panic!("This function never returns!");
}
