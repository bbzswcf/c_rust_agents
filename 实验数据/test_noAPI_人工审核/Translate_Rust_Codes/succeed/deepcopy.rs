#[derive(Clone)]
struct Layer1 {
    a: i32,
}

#[derive(Clone)]
struct Layer2 {
    l1: Layer1,
    b: f32,
}

#[derive(Clone)]
struct Layer3 {
    l2: Layer2,
    l1: Layer1,
    d: i32,
    e: i32,
}

fn show_cake(cake: &Layer3) {
    println!("cake.d = {}", cake.d);
    println!("cake.e = {}", cake.e);
    println!("cake.l1.a = {}", cake.l1.a);
    println!("cake.l2.b = {:.6}", cake.l2.b); // Modified: Adjusted floating-point formatting to 6 decimal places
    println!("cake.l2.l1.a = {}", cake.l2.l1.a);
}

fn main() {
    let cake1 = Layer3 {
        d: 1,
        e: 2,
        l1: Layer1 { a: 3 },
        l2: Layer2 {
            l1: Layer1 { a: 5 },
            b: 4.0,
        },
    };

    println!("Cake 1 is : ");
    show_cake(&cake1);

    // Modified: Declare cake2 as mutable to allow modifications
    let mut cake2 = cake1.clone();

    // Modified: Corrected field modification logic to add the correct values together
    cake2.l2.b += cake2.l2.l1.a as f32;

    println!("Cake 2 is : "); // Modified: Removed unnecessary newline before "Cake 2 is :"
    show_cake(&cake2);
}