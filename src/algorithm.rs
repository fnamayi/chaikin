use std::time::Duration;

// Structure to represent a point
#[derive(Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

// Chaikin Algorithm Function
fn chaikin_algorithm(points: &[Point]) -> Vec<Point> {
    if points.len() < 2 {
        return points.to_vec();
    }

    let mut new_points = Vec::new();

    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        // Generate the new points
        let q = Point {
            x: 0.75 * p1.x + 0.25 * p2.x,
            y: 0.75 * p1.y + 0.25 * p2.y,
        };
        let r = Point {
            x: 0.25 * p1.x + 0.75 * p2.x,
            y: 0.25 * p1.y + 0.75 * p2.y,
        };

        new_points.push(q);
        new_points.push(r);
    }

    new_points
}