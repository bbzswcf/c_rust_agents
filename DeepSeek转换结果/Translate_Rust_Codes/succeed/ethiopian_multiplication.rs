fn halve(x: &mut i32) {
    *x >>= 1;
}

fn doublit(x: &mut i32) {
    *x <<= 1;
}

fn iseven(x: i32) -> bool {
    (x & 1) == 0
}

fn ethiopian(mut plier: i32, mut plicand: i32, tutor: bool) -> i32 {
    let mut result = 0;

    if tutor {
        print!("ethiopian multiplication of {} by {}\n", plier, plicand);
    }

    while plier >= 1 {
        if iseven(plier) {
            if tutor {
                print!("{:4} {:6} struck\n", plier, plicand);
            }
        } else {
            if tutor {
                print!("{:4} {:6} kept\n", plier, plicand);
            }
            result += plicand;
        }
        halve(&mut plier);
        doublit(&mut plicand);
    }
    result
}

fn main() {
    print!("{}\n", ethiopian(17, 34, true));
}