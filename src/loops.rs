pub fn loops() {
    let mut x = 5;
    let mut is_loop = true;

    // This if statement checks whether the value of x is divisible by 5 (i.e., when x divided by 5 leaves a remainder of 0), and if true, it sets the boolean variable isLoop to true, which will terminate the enclosing while loop.
    while !is_loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            is_loop = true;
        }
    }

    // This for loop in Rust iterates through each integer from 0 to 9 (inclusive of 0, exclusive of 10), binding each value to the variable x of type i32 and executing the println! macro for each iteration to print the current value.
    for x in 0..10 {
        println!("{}", x); // x: i32
    }

    // The for (index, value) in (5..10).enumerate() loop iterates over the range 5 to 9, where enumerate() provides the zero-based iteration count as index and the current range element as value in each cycle.
    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    // The if statement checks whether x is even by verifying if the remainder of x divided by 2 is zero, and when true, the continue keyword skips the current iteration, moving to the next number, effectively printing only odd numbers between 0 and 10.
    for x in 0..10 {
        if x % 2 == 0 {
            continue;
        }

        println!("{}", x);
    }

    // The if statements here use labeled continue to control which nested loop level to skip to the next iteration of, either jumping back to the 'outer' x-loop or the 'inner' y-loop based on whether x or y is even.
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            } // Continues the loop over `x`.
            if y % 2 == 0 {
                continue 'inner;
            } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }

    // The if x % 5 == 0 { break; } statement checks if the current value of x is divisible by 5, and if it is, the break command terminates the loop immediately.
    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            break;
        }
    }

    // The loop keyword creates an infinite loop that will repeatedly execute the block's code until explicitly terminated with a break statement.
    loop {
        println!("Loop forever!");
    }
}
