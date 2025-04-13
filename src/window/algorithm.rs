use nalgebra::Point2;
use crate::types::Point;

/// Smooths out a series of points to create a nice curve
pub struct ChaikinAlgorithm {
    /// First point ratio (how far the new point is along the line)
    q_ratio: f32,
    /// Second point ratio (how far the other new point is along the line)
    r_ratio: f32,
}

impl ChaikinAlgorithm {
    /// Creates a new smoothing tool with standard settings
    pub fn new() -> Self {
        Self {
            q_ratio: 0.25, // Place first point 25% along each line segment
            r_ratio: 0.75, // Place second point 75% along each line segment
        }
    }

    /// Does one round of smoothing to make the curve nicer
    ///
    /// Input:
    /// - A list of points (the original shape)
    ///
    /// Output:
    /// - A new list of points (a smoother shape)
    ///
    /// Special cases:
    /// - No points: returns an empty list
    /// - One or two points: no changes, just return them
    pub fn calculate_step(&self, points: &[Point]) -> Vec<Point> {
        match points.len() {
            0 => return Vec::new(), // If no points, return an empty list
            1 | 2 => return points.to_vec(), // If one or two points, no smoothing needed
            _ => {} // If more than two points, start smoothing
        }

        let mut new_points = Vec::new();

        // Keep the first point as is
        new_points.push(points[0]);

        // Go through every pair of points and smooth the curve
        for i in 0..points.len() - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];

            // Find the first new point (closer to the first point)
            let q = Point2::new(
                (1.0 - self.q_ratio) * p0.x + self.q_ratio * p1.x,
                (1.0 - self.q_ratio) * p0.y + self.q_ratio * p1.y,
            );

            // Find the second new point (closer to the second point)
            let r = Point2::new(
                (1.0 - self.r_ratio) * p0.x + self.r_ratio * p1.x,
                (1.0 - self.r_ratio) * p0.y + self.r_ratio * p1.y,
            );

            // Add both new points to the list
            new_points.push(q);
            new_points.push(r);
        }

        // Keep the last point as is
        new_points.push(*points.last().unwrap());

        new_points
    }

    /// Smooth the curve over several rounds
    ///
    /// Input:
    /// - A list of points (the original shape)
    /// - Number of smoothing steps to apply
    ///
    /// Output:
    /// - The final smoothed points after the steps
    pub fn get_step_points(&self, initial_points: &[Point], step: usize) -> Vec<Point> {
        // If step is 0 or not enough points, just return the original points
        if step == 0 || initial_points.len() <= 2 {
            return initial_points.to_vec();
        }

        let mut current_points = initial_points.to_vec();
        for _ in 0..step {
            current_points = self.calculate_step(&current_points); // Smooth one step at a time
        }

        current_points // Return the final smoothed points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_points() {
        let algorithm = ChaikinAlgorithm::new();
        let empty: Vec<Point> = Vec::new();

        assert_eq!(algorithm.calculate_step(&empty).len(), 0);
        assert_eq!(algorithm.get_step_points(&empty, 1).len(), 0);
    }

    #[test]
    fn test_single_point() {
        let algorithm = ChaikinAlgorithm::new();
        let point = Point2::new(100.0, 100.0);
        let points = vec![point];

        let step_result = algorithm.calculate_step(&points);
        assert_eq!(step_result.len(), 1);
        assert_eq!(step_result[0], point);

        let step_points = algorithm.get_step_points(&points, 3);
        assert_eq!(step_points.len(), 1);
        assert_eq!(step_points[0], point);
    }

    #[test]
    fn test_two_points() {
        let algorithm = ChaikinAlgorithm::new();
        let points = vec![
            Point2::new(0.0, 0.0),
            Point2::new(100.0, 100.0),
        ];

        let step_result = algorithm.calculate_step(&points);
        assert_eq!(step_result.len(), 2);
        assert_eq!(step_result[0], points[0]);
        assert_eq!(step_result[1], points[1]);
    }

    #[test]
    fn test_three_points() {
        let algorithm = ChaikinAlgorithm::new();
        let points = vec![
            Point2::new(0.0, 0.0),
            Point2::new(100.0, 100.0),
            Point2::new(200.0, 0.0),
        ];

        let step1 = algorithm.calculate_step(&points);
        assert_eq!(step1.len(), 6);
        assert_eq!(step1[0], points[0]);
        assert_eq!(step1[step1.len() - 1], *points.last().unwrap());

        assert!((step1[1].x - 25.0).abs() < 0.001);
        assert!((step1[1].y - 25.0).abs() < 0.001);

        assert!((step1[3].x - 125.0).abs() < 0.001);
        assert!((step1[3].y - 75.0).abs() < 0.001);

        assert!((step1[2].x - 75.0).abs() < 0.001);
        assert!((step1[2].y - 75.0).abs() < 0.001);

        assert!((step1[4].x - 175.0).abs() < 0.001);
        assert!((step1[4].y - 25.0).abs() < 0.001);
    }
}
