struct Point {
    x: f64,
    y: f64,
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    f64::sqrt((p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y))
}

fn find_circles(p1: &Point, p2: &Point, radius: f64) {
    let separation = distance(p1, p2);
    let mirror_distance;

    if separation == 0.0 {
        if radius > 0.0 {
            println!("\nInfinitely many circles can be drawn through ({:.4},{:.4})", p1.x, p1.y); // Corrected: Mention the point through which infinitely many circles can be drawn
        } else {
            println!("\nNo circles can be drawn through ({:.4},{:.4})", p1.x, p1.y); // Corrected: Removed mention of a specific radius
        }
    } else if separation == 2.0 * radius {
        println!(
            "\nGiven points are opposite ends of a diameter of the circle with center ({:.4},{:.4}) and radius {:.4}",
            (p1.x + p2.x) / 2.0,
            (p1.y + p2.y) / 2.0,
            radius // Corrected: Use the provided radius
        );
    } else if separation > 2.0 * radius {
        println!(
            "\nGiven points are farther away from each other than a diameter of a circle with radius {:.4}",
            radius
        );
    } else {
        mirror_distance = f64::sqrt(radius * radius - (separation / 2.0) * (separation / 2.0));

        println!("\nTwo circles are possible.");
        println!(
            "\nCircle C1 with center ({:.4},{:.4}), radius {:.4} and Circle C2 with center ({:.4},{:.4}), radius {:.4}",
            (p1.x + p2.x) / 2.0 + mirror_distance * (p2.y - p1.y) / separation, // Corrected: Corrected center calculation for Circle C1
            (p1.y + p2.y) / 2.0 - mirror_distance * (p2.x - p1.x) / separation, // Corrected: Corrected center calculation for Circle C1
            radius,
            (p1.x + p2.x) / 2.0 - mirror_distance * (p2.y - p1.y) / separation, // Corrected: Corrected center calculation for Circle C2
            (p1.y + p2.y) / 2.0 + mirror_distance * (p2.x - p1.x) / separation, // Corrected: Corrected center calculation for Circle C2
            radius
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
        find_circles(&cases[2 * i], &cases[2 * i + 1], radii[i]);
    }
}