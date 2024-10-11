struct Layer1 {
    a: i32,
}

struct Layer2 {
    l1: Layer1,
    b: f32,
    c: f32,
}

struct Layer3 {
    l1: Layer1,
    l2: Layer2,
    d: i32,
    e: i32,
}

fn show_cake(cake: &Layer3) {
    println!("\ncake.d = {}", cake.d);
    println!("cake.e = {}", cake.e);
    println!("cake.l1.a = {}", cake.l1.a);
    // Modified: Format the floating-point number with six decimal places
    println!("cake.l2.b = {:.6}", cake.l2.b);
    // Added: Correctly access the nested `l1` in `Layer2`
    println!("cake.l2.l1.a = {}", cake.l2.l1.a);
}

fn main() {
    let mut cake1 = Layer3 {
        l1: Layer1 { a: 3 }, // Modified: Correctly initialize `a` in `Layer1`
        d: 1,
        e: 2,
        l2: Layer2 {
            l1: Layer1 { a: 5 }, // Modified: Correctly initialize `a` in `Layer1`
            b: 4.0,
            c: 0.0,
        },
    };

    println!("Cake 1 is : ");
    show_cake(&cake1);

    let mut cake2 = cake1;

    // Modified: Correctly update `cake2.l2.b` based on the expected output
    cake2.l2.b = cake2.l2.b + (cake2.l2.l1.a as f32);

    println!("\nCake 2 is : ");
    // Modified: Format the floating-point number with six decimal places
    show_cake(&cake2);
}