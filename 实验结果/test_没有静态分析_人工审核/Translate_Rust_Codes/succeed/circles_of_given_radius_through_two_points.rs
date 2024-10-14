struct Point {
    x: f64,
    y: f64,
}

// Modified: Changed parameter type to borrow instead of taking ownership
fn distance(p1: &Point, p2: &Point) -> f64 {
    f64::sqrt((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y))
}

// Modified: Changed parameter type to borrow instead of taking ownership
fn find_circles(p1: &Point, p2: &Point, radius: f64) {
    let separation = distance(p1, p2);
    let mirror_distance; // Modified: Removed `mut` keyword since the variable does not need to be mutable

    // Modified: Corrected the condition to handle separation exactly zero
    if separation == 0.0 && radius > 0.0 {
        println!("\nInfinitely many circles can be drawn through ({:.4},{:.4})", p1.x, p1.y);
    } else if separation == 0.0 && radius == 0.0 {
        println!("\nNo circles can be drawn through ({:.4},{:.4})", p1.x, p1.y);
    } else if separation == 2.0 * radius {
        // Modified: Corrected the condition to handle the case where the points are exactly the ends of a diameter of the circle
        println!(
            "\nGiven points are opposite ends of a diameter of the circle with center ({:.4},{:.4}) and radius {:.4}",
            (p1.x + p2.x) / 2.0,
            (p1.y + p2.y) / 2.0,
            radius
        );
    } else if separation > 2.0 * radius {
        // Modified: Corrected the condition to handle the case where the points are farther apart than the diameter of the circle
        println!("\nGiven points are farther away from each other than a diameter of a circle with radius {:.4}", radius);
    } else {
        mirror_distance = f64::sqrt(f64::powf(radius, 2.0) - f64::powf(separation / 2.0, 2.0));

        println!("\nTwo circles are possible.");
        // Modified: Corrected the calculation of circle centers
        let center1_x = (p1.x + p2.x) / 2.0 + mirror_distance * (p1.y - p2.y) / separation;
        let center1_y = (p1.y + p2.y) / 2.0 + mirror_distance * (p2.x - p1.x) / separation;
        let center2_x = (p1.x + p2.x) / 2.0 - mirror_distance * (p1.y - p2.y) / separation;
        let center2_y = (p1.y + p2.y) / 2.0 - mirror_distance * (p2.x - p1.x) / separation;

        // Modified: Corrected the output format to match the expected output
        println!(
            "\nCircle C1 with center ({:.4},{:.4}), radius {:.4} and Circle C2 with center ({:.4},{:.4}), radius {:.4}",
            center1_x, center1_y, radius, center2_x, center2_y, radius
        );
    }
}

fn main() {
    let cases = [
        Point { x: 0.1234, y: 0.9876 },
        Point { x: 0.8765, y: 0.2345 },
        Point { x: 0.0000, y: 2.0000 },
        Point { x: 0.0000, y: 0.0000 },
        Point { x: 0.1234, y: 0.9876 },
        Point { x: 0.1234, y: 0.9876 },
        Point { x: 0.1234, y: 0.9876 },
        Point { x: 0.8765, y: 0.2345 },
        Point { x: 0.1234, y: 0.9876 },
        Point { x: 0.1234, y: 0.9876 },
    ];

    let radii = [2.0, 1.0, 2.0, 0.5, 0.0];

    for i in 0..5 {
        println!("\nCase {}:", i + 1);
        // Modified: Pass references to the `find_circles` function instead of moving the values
        find_circles(&cases[2 * i], &cases[2 * i + 1], radii[i]);
    }
}