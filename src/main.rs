mod function;
mod types;
mod variable;
mod input;
mod conditions;

fn main() {
    types::type_print();

    variable::variable_print();

    let res_sum = function::sum(10, 10);
    println!("{res_sum:?}");

    input::new_input();

    conditions::condition();
}

/*
this is a multi-line comment
 */

//  this is a single-line comment

/*
We have four commands for using Cargo during our development workflow with Rust.

- Cargo new: to create a new project in Rust.
- Cargo build: to compile our project. Usually, a target folder is created and the compiled code is placed inside it.
- Cargo run: to run the project. It mainly executes a Cargo Build first and then executes the compiled code, all in a single command.
- Cargo Check: to verify if the project can be compiled without errors.

*/
