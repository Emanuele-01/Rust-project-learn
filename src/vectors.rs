pub fn vectors() {
    // A ‘vector’ is a dynamic or ‘growable’ array, implemented as the standard library type Vec<T>. creation coincides.
    let v = vec![1, 2, 3, 4, 5, 6];

    // creation not coincides.
    let v2 = Vec::from([1, 2, 3, 4, 5, 6]);

    println!("v1: {}", v[0]);
    println!("v2: {}", v2[0]);

    let i: usize = 0;
    // Works:
    v[i];

    // let j: i32 = 0;
    // Doesn’t:
    // v[j];

    let mut x = vec![1, 2, 3, 4, 5];

    // Once you have a vector, you can iterate through its elements with for
    for i in &x {
        println!("A reference to {}", i);
    }

    for i in &mut x {
        println!("A mutable reference to {}", i);
    }

    for i in x {
        println!("Take ownership of the vector and its element {}", i);
    }
}
