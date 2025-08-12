#![allow(unused)]


// Struct
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// 2nde way to create a struct
struct Point3d(f32, f32, f32);

// Creating a empty struct
struct Empty;

// Nested struct
#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    // Create
    let p = Point { x: 1.0, y: 2.0};
    println!("Point.x = {}, Point.y = {}", p.x, p.y);

    let p = Point3d(1.0, 2.0, 3.0);
    println!("Point3d.0 = {}, Point3d.1 = {}, Point3d.2 = {}", p.0, p.1, p.2);

    let empty = Empty;
    
    let circle = Circle {
        center: Point{x: 1.0, y: 2.0},
        radius: 15,
    };

    // Debug
    // Read
    println!("Circle {:#?}", circle);


    // Shortcut
    let x = 1.0;
    let y = 2.0;
    let p = Point { x, y };

    // Copy fields
    let p0 = Point { x: 1.0, y: 2.0 };
    let p1 = Point { x: 2.0, ..p0 };
    println!("Struct copy {:?}", p1);

    // Update
    let mut p = Point { x: 1.0, y: 2.0};
    p.x += 1.0;
    p.y += 1.0;
    println!("struct updated {:?}", p);

}