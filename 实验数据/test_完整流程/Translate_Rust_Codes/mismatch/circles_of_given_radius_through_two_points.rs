#[derive(Copy, Clone)] // Added: Implement the Copy trait for Point
struct Point {
    x: f64,
    y: f64,
}

fn distance(p1: Point, p2: Point) -> f64 {
    // Modified: Use absolute values to avoid negative subtraction
    let dx = (p1.x - p2.x).abs();
    let dy = (p1.y - p2.y).abs();
    f64::sqrt(dx.powf(2.0) + dy.powf(2.0))
}

fn find_circles(p1: &Point, p2: &Point, radius: f64) { // Modified: Borrow the elements instead of moving them
    let separation = distance(*p1, *p2);
    let epsilon = f64::EPSILON; // Small epsilon value for floating-point comparison

    // Modified: Handle the case where separation is exactly zero more accurately
    if (separation - 0.0).abs() < epsilon {
        if radius.abs() < epsilon {
            println!("\nNo circles can be drawn through ({:.4},{:.4})", p1.x, p1.y);
        } else {
            println!("\nInfinitely many circles can be drawn through ({:.4},{:.4})", p1.x, p1.y);
        }
    } else if (separation - 2.0 * radius).abs() < epsilon {
        println!(
            "\nGiven points are opposite ends of a diameter of the circle with center ({:.4},{:.4}) and radius {:.4}",
            (p1.x + p2.x) / 2.0,
            (p1.y + p2.y) / 2.0,
            radius
        );
    } else if separation > 2.0 * radius {
        println!("\nGiven points are farther away from each other than a diameter of a circle with radius {:.4}", radius);
    } else {
        let mirror_distance = f64::sqrt(radius.powf(2.0) - (separation / 2.0).powf(2.0));

        // Modified: Correct the calculation of circle centers using the correct formula
        let center1_x = (p1.x + p2.x) / 2.0 + mirror_distance * (p2.y - p1.y) / separation;
        let center1_y = (p1.y + p2.y) / 2.0 - mirror_distance * (p2.x - p1.x) / separation;
        let center2_x = (p1.x + p2.x) / 2.0 - mirror_distance * (p2.y - p1.y) / separation;
        let center2_y = (p1.y + p2.y) / 2.0 + mirror_distance * (p2.x - p1.x) / separation;

        println!("\nTwo circles are possible.");
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
        println!("\nCase {})", i + 1);
        // Ensure the indices do not exceed the bounds of the cases array
        if 2 * i + 1 < cases.len() {
            find_circles(&cases[2 * i], &cases[2 * i + 1], radii[i]); // Modified: Borrow the elements instead of moving them
        } else {
            println!("Index out of bounds for cases array");
        }
    }
}