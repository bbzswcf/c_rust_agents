struct Layer1 {
    a: i32,
}

struct Layer2 {
    l1: Layer1,
    b: f32,
    c: f32,
}

struct Layer3 {
    l2: Layer2,
    l1: Layer1,
    d: i32,
    e: i32,
}

// Modified: Corrected field access and output format
fn show_cake(cake: &Layer3) {
    println!("cake.d = {}", cake.d);
    println!("cake.e = {}", cake.e);
    println!("cake.l1.a = {}", cake.l1.a);
    println!("cake.l2.b = {:.6}", cake.l2.b);
    println!("cake.l2.l1.a = {}", cake.l2.l1.a);
}

fn main() {
    // Modified: Correctly initialized all fields including nested `Layer1` in `Layer2`
    let cake1 = Layer3 {
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
    show_cake(&cake1); // Modified: Passed a reference to `cake1`

    let mut cake2 = Layer3 {
        d: 1,
        e: 2,
        l1: Layer1 { a: 3 },
        l2: Layer2 {
            l1: Layer1 { a: 5 },
            b: 4.0,
            c: 0.0,
        },
    };

    // Modified: Correctly accessed fields for addition
    cake2.l2.b += cake2.l2.l1.a as f32;

    println!("\nCake 2 is : ");
    show_cake(&cake2); // Modified: Passed a reference to `cake2`
}