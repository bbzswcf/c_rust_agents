use std::io;

fn halve(x: &mut i32) { *x >>= 1; }
fn doublit(x: &mut i32) { *x <<= 1; }
fn iseven(x: i32) -> bool { (x & 1) == 0 }

fn ethiopian(mut plier: i32, mut plicand: i32, tutor: bool) -> i32 {
    let mut result = 0;

    if tutor {
        println!("ethiopian multiplication of {} by {}", plier, plicand);
    }

    while plier >= 1 {
        if iseven(plier) {
            if tutor {
                println!("{:4} {:6} struck", plier, plicand);
            }
        } else {
            if tutor {
                println!("{:4} {:6} kept", plier, plicand);
            }
            result += plicand;
        }
        halve(&mut plier);
        doublit(&mut plicand);
    }
    result
}

fn main() {
    println!("{}", ethiopian(17, 34, true));
}