#[derive(Clone)] // Derived Clone trait for Layer1
struct Layer1 {
    a: i32,
}

#[derive(Clone)] // Derived Clone trait for Layer2
struct Layer2 {
    l1: Layer1,
    b: f32,
    c: f32,
}

#[derive(Clone)] // Derived Clone trait for Layer3
struct Layer3 {
    l1: Layer1, // Added: Include the `l1` field in `Layer3`
    l2: Layer2,
    d: i32,
    e: i32,
}

fn show_cake(cake: Layer3) {
    println!("\ncake.d = {}", cake.d); // Ensured correct field access for `cake.d`
    println!("cake.e = {}", cake.e); // Ensured correct field access for `cake.e`
    println!("cake.l1.a = {}", cake.l1.a); // Ensured correct field access for `cake.l1.a`
    println!("cake.l2.b = {:.6}", cake.l2.b); // Ensured correct field access for `cake.l2.b`
    println!("cake.l2.l1.a = {}", cake.l2.l1.a); // Ensured correct field access for `cake.l2.l1.a`
}

fn main() {
    let mut cake1 = Layer3 {
        l1: Layer1 { a: 3 }, // Ensured correct initialization of `l1` field in `Layer3`
        d: 1,
        e: 2,
        l2: Layer2 {
            l1: Layer1 { a: 5 }, // Ensured correct initialization of `l1` field in `Layer2`
            b: 4.0,
            c: 0.0,
        },
    };

    println!("Cake 1 is : ");
    show_cake(cake1.clone()); // Cloned cake1 to avoid moving it

    let mut cake2 = cake1.clone(); // Cloned cake1 to cake2 to keep cake1 intact

    // Ensured correct field access to cake2.l2.l1.a with correct value `5`
    cake2.l2.b += cake2.l2.l1.a as f32;

    println!("\nCake 2 is : ");
    show_cake(cake2);
}