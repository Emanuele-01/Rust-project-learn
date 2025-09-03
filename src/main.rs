mod function;
mod types;
mod variable;

fn main() {
    types::type_print();

    variable::variable_print();

    let res_sum = function::sum(10, 10);
    println!("{res_sum:?}")
}

/*
 * this is a multi-line comment
 */

//  this is a single-line comment
