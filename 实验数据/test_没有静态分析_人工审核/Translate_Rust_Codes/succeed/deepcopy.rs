// Removed: Unused import
// use std::fmt;

#[derive(Clone)] // Added: Derive Clone trait for Layer1
struct Layer1 {
    a: i32,
}

#[derive(Clone)] // Added: Derive Clone trait for Layer2
struct Layer2 {
    l1: Layer1,
    b: f32,
    c: f32,
}

#[derive(Clone)] // Added: Derive Clone trait for Layer3
struct Layer3 {
    l2: Layer2,
    l1: Layer1,
    d: i32,
    e: i32,
}

impl Layer3 {
    fn show_cake(&self) {
        println!("\ncake.d = {}", self.d);
        println!("cake.e = {}", self.e);
        println!("cake.l1.a = {}", self.l1.a);
        // Modified: Format floating-point number to match the precision of the C output
        println!("cake.l2.b = {:.6}", self.l2.b);
        println!("cake.l2.l1.a = {}", self.l2.l1.a);
    }
}

fn main() {
    let mut cake1 = Layer3 {
        d: 1,
        e: 2,
        l1: Layer1 { a: 3 },
        l2: Layer2 {
            l1: Layer1 { a: 5 },
            b: 4.0,
            c: 0.0,
        },
    };

    println!("Cake 1 is : ");
    cake1.show_cake();

    let mut cake2 = cake1.clone(); // Fixed: Method clone not found for Layer3

    cake2.l2.b += cake2.l2.l1.a as f32;

    // Modified: Ensure the output format matches the C output by adding a newline before "Cake 2 is :"
    println!("\n\nCake 2 is : ");
    cake2.show_cake();
}