#[derive(Debug, Clone, Copy)]
struct Layer1 {
    a: i32,
}

#[derive(Debug, Clone, Copy)]
struct Layer2 {
    l1: Layer1,
    b: f32,
    c: f32,
}

#[derive(Debug, Clone, Copy)]
struct Layer3 {
    l2: Layer2,
    l1: Layer1,
    d: i32,
    e: i32,
}

fn show_cake(cake: Layer3) {
    println!("\ncake.d = {}", cake.d);
    println!("cake.e = {}", cake.e);
    println!("cake.l1.a = {}", cake.l1.a);
    println!("cake.l2.b = {}", cake.l2.b);
    println!("cake.l2.l1.a = {}", cake.l2.l1.a);
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
    show_cake(cake1);

    let mut cake2 = cake1;

    cake2.l2.b += cake2.l2.l1.a as f32;

    println!("\nCake 2 is : ");
    show_cake(cake2);
}