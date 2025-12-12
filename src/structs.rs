pub fn structs() {
    // structs are a way of creating more complex data types.
    // The values in structs are immutable by default, like other bindings in Rust.
    struct Point {
        x: i32,
        y: i32,
    }
    struct Point2 {
        x: i32,
        y: i32,
        z: i32
    }

    let origin = Point { x: 0, y: 0 }; // origin: Point

    if origin.x == 0 {
        println!("x è uguale a 0")
    };

    if origin.y == 0 {
        println!("y è uguale a 0")
    };

    let point = Point { x: 0, y: 0 };

    let point = point; // `point` is now immutable.

    println!("point z: {}", point.x);
    // Your structure can still contain &mut references
    // The struct PointRef<'a> { x: &'a mut i32, y: &'a mut i32 } is a mutable reference-based Point struct in Rust with a lifetime annotation 'a, ensuring that the references x and y cannot outlive the data they point to, thereby enforcing memory safety and preventing dangling references.
    struct PointRef<'a> {
        x: &'a mut i32,
        y: &'a mut i32,
    }

    let mut point2 = Point { x: 0, y: 0 };

    // This code creates a struct r that holds mutable references to the fields of point2, allowing you to modify the original point2 data through the struct's references.
    let r = PointRef {
        x: &mut point2.x,
        y: &mut point2.y,
    };

    *r.x = 5;
    *r.y = 6;

    let point3 = Point2{z: 1, x: 10, y: 30};
    let point4 = Point2{z:5, .. point3};

    println!("z di Point4: {}", point4.z);
}
