use nalgebra::Point2;
use crate::types::Point;

/// Implements Chaikin's curve algorithm for smooth curve generation
pub struct ChaikinAlgorithm {
    q_ratio: f32,
    r_ratio: f32,
}

impl ChaikinAlgorithm {
    /// Creates a new instance of the Chaikin algorithm with default ratios
    pub fn new() -> Self {
        Self {
            q_ratio: 0.25,
            r_ratio: 0.75,
        }
    }

    /// Calculate one step of Chaikin's algorithm
    pub fn calculate_step(&self, points: &[Point]) -> Vec<Point> {
        match points.len() {
            0 => return Vec::new(),
            1 | 2 => return points.to_vec(),
            _ => {}
        }

        let mut new_points = Vec::new();
        new_points.push(points[0]);

        for i in 0..points.len() - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];

            let q = Point2::new(
                (1.0 - self.q_ratio) * p0.x + self.q_ratio * p1.x,
                (1.0 - self.q_ratio) * p0.y + self.q_ratio * p1.y,
            );

            let r = Point2::new(
                (1.0 - self.r_ratio) * p0.x + self.r_ratio * p1.x,
                (1.0 - self.r_ratio) * p0.y + self.r_ratio * p1.y,
            );

            new_points.push(q);
            new_points.push(r);
        }

        new_points.push(*points.last().unwrap());
        new_points
    }

    /// Get points for a specific step
    /// If the step is out of range, returns the highest available step
    pub fn get_step_points(&self, initial_points: &[Point], step: usize) -> Vec<Point> {
        if step == 0 || initial_points.len() <= 2 {
            return initial_points.to_vec();
        }

        let mut current_points = initial_points.to_vec();
        for _ in 0..step {
            current_points = self.calculate_step(&current_points);
        }

        current_points
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
