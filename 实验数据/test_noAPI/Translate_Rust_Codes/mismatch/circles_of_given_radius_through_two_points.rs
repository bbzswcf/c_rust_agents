struct Point {
    x: f64,
    y: f64,
}

// Modified: Simplified the function to compare floating-point numbers with a fixed epsilon
fn almost_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

fn find_circles(p1: &Point, p2: &Point, radius: f64) {
    let separation = distance(p1, p2);

    // Modified: Use the almost_equal function for floating-point comparisons
    if almost_equal(separation, 0.0, 1e-9) {
        if almost_equal(radius, 0.0, 1e-9) {
            println!("\nNo circles can be drawn through ({:.4},{:.4})", p1.x, p1.y);
        } else {
            println!("\nInfinitely many circles can be drawn through ({:.4},{:.4})", p1.x, p1.y);
        }
    } else if almost_equal(separation, 2.0 * radius, 1e-9) {
        println!(
            "\nGiven points are opposite ends of a diameter of the circle with center ({:.4},{:.4}) and radius {:.4}",
            (p1.x + p2.x) / 2.0,
            (p1.y + p2.y) / 2.0,
            radius
        );
    } else if separation > 2.0 * radius {
        println!(
            "\nGiven points are farther away from each other than a diameter of a circle with radius {:.4}",
            radius
        );
    } else {
        let mirror_distance = (radius.powi(2) - (separation / 2.0).powi(2)).sqrt();

        println!("\nTwo circles are possible.");
        // Modified: Corrected the calculation of circle centers
        let center1_x = (p1.x + p2.x) / 2.0 + mirror_distance * (p1.y - p2.y) / separation;
        let center1_y = (p1.y + p2.y) / 2.0 + mirror_distance * (p2.x - p1.x) / separation;
        let center2_x = (p1.x + p2.x) / 2.0 - mirror_distance * (p1.y - p2.y) / separation;
        let center2_y = (p1.y + p2.y) / 2.0 - mirror_distance * (p2.x - p1.x) / separation;

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

    // Modified: Ensure the enumeration correctly matches the number of cases with the number of radii
    for (i, chunk) in cases.chunks(2).enumerate() {
        if i < radii.len() {
            let p1 = &chunk[0];
            let p2 = &chunk[1];
            println!("\nCase {}:", i + 1);
            find_circles(p1, p2, radii[i]);
        } else {
            // Modified: Exit the loop if there are no more radii to process
            break;
        }
    }
}