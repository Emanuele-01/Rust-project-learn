#[allow(dead_code)]
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

pub fn type_print() {
    // this is a number primitive. the insteger type is: i8, i16, i32, i64, i128
    let num: i8 = 10;

    // this is a unsigned number primitive. the insteger type is: u8, u16, u32, u64, u128
    let u_num: u8 = 10;

    // this is a float primitive. the float type is: f32, f64
    let float: f32 = 10.0;

    // this is a boolean primitive. the boolean type is: bool
    let bool: bool = true;

    // this is a string primitive. the string type is: &str
    let str: &str = "Hello, world!";

    // this is a char primitive. the char type is: char
    let char: char = 'a';

    // this is a tuple primitive. the tuple type is: (Type, T, T, T, T, T, T, T, T, T)
    let tuple: (i8, i8, i8, i8, i8, i8, i8, i8, i8, i8) = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

    /*
    Tuples are lists of immutable values, which means that once they are filled with values, those values can no longer be changed.
    We can read the tuple with ‘.’ followed by the index of the value to be read.
    */

    // this is a vector primitive. the vector type is: Vec<Type>
    let vector: Vec<i8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // this is an array primitive. the array type is: [Type; length]
    let array: [i8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // this is a print with format string
    println!("num: {}", num);

    // this is a print with format string
    println!("unsigned number: {}", u_num);

    // this is a print with format string
    println!("float: {float:?}");

    // this is a print with format string
    println!("bool: {bool:?}");

    // this is a print with format string
    println!("str: {str:?}");

    // this is a print with format string
    println!("char: {char:?}");

    // this is a print with format string
    println!("tuple: {tuple:?}");

    // this is a print with format string
    println!("single value tuple: {}", tuple.2);

    // this is a print with format string
    println!("vector: {vector:?}");

    // this is a print with format string
    println!("array: {array:?}");

    let person = Person {
        name: "John",
        age: 30,
    };

    // this is a print with format pretty string
    println!("person: {:#?}", person);
}
