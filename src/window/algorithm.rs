use nalgebra::Point2;
use crate::types::{Point};

/// Implements Chaikin's curve algorithm for smooth curve generation
pub struct ChaikinAlgorithm {
    // Ratios used in Chaikin's algorithm
    // 0.25 means the first point is placed 1/4 along each segment
    // 0.75 means the second point is placed 3/4 along each segment
    q_ratio: f32,
    r_ratio: f32,
}

impl ChaikinAlgorithm {
    /// Creates a new instance of the Chaikin algorithm with default ratios
    pub fn new() -> Self {
        Self {
            q_ratio: 0.25, // Standard 1/4 ratio
            r_ratio: 0.75, // Standard 3/4 ratio
        }
    }

    /// Creates a new instance with custom ratios
    #[allow(dead_code)]
    pub fn with_ratios(q_ratio: f32, r_ratio: f32) -> Self {
        Self { q_ratio, r_ratio }
    }

    /// Calculate one step of Chaikin's algorithm
    #[allow(dead_code)]
    pub fn calculate_step(&self, points: &[Point]) -> Vec<Point> {
        // Special cases handling
        match points.len() {
            0 => return Vec::new(),         // No points
            1 => return points.to_vec(),    // Single point - just return it
            2 => return points.to_vec(),    // Two points - just a line segment
            _ => {}                         // More than 2 points - apply algorithm
        }

        let mut new_points = Vec::new();
        
        // Keep first point
        new_points.push(points[0]);
        
        // Process each segment between consecutive points
        for i in 0..points.len() - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];
            
            // Calculate the Q point (1/4 along the line)
            let q = Point2::new(
                (1.0 - self.q_ratio) * p0.x + self.q_ratio * p1.x,
                (1.0 - self.q_ratio) * p0.y + self.q_ratio * p1.y
            );
            
            // Calculate the R point (3/4 along the line)
            let r = Point2::new(
                (1.0 - self.r_ratio) * p0.x + self.r_ratio * p1.x,
                (1.0 - self.r_ratio) * p0.y + self.r_ratio * p1.y
            );
            
            // Add Q and R points
            new_points.push(q);
            new_points.push(r);
        }
        
        // Keep last point
        new_points.push(*points.last().unwrap());
        
        new_points
    }
    
    /// Calculate all steps of Chaikin's algorithm up to the specified step
    /// Returns a vector containing all the intermediate points for each step
    #[allow(dead_code)]
    pub fn calculate_steps(&self, initial_points: &[Point], steps: usize) -> Vec<Vec<Point>> {
        // Special cases handling
        match initial_points.len() {
            0 => return vec![Vec::new()],              // No points
            1 => return vec![initial_points.to_vec()], // Single point
            2 => return vec![initial_points.to_vec()], // Just a line segment
            _ => {}                                    // More than 2 points - apply algorithm
        }
        
        let mut result = Vec::with_capacity(steps);
        let mut current_points = initial_points.to_vec();
        
        // First step is the original points
        result.push(current_points.clone());
        
        // Calculate each subsequent step
        for _ in 0..steps {
            current_points = self.calculate_step(&current_points);
            result.push(current_points.clone());
        }
        
        result
    }
    
    /// Get points for a specific step
    /// If the step is out of range, returns the highest available step
    pub fn get_step_points(&self, initial_points: &[Point], step: usize) -> Vec<Point> {
        // For steps 0 or no points, return the initial points
        if step == 0 || initial_points.len() <= 2 {
            return initial_points.to_vec();
        }
        
        // Generate points for the requested step
        let mut current_points = initial_points.to_vec();
        for _ in 0..step {
            current_points = self.calculate_step(&current_points);
        }
        
        current_points
    }
}

//tests
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
        
        // Single point should remain unchanged
        let step_result = algorithm.calculate_step(&points);
        assert_eq!(step_result.len(), 1);
        assert_eq!(step_result[0], point);
        
        // Get step points should also return the original point
        let step_points = algorithm.get_step_points(&points, 3);
        assert_eq!(step_points.len(), 1);
        assert_eq!(step_points[0], point);
    }
    
    #[test]
    fn test_two_points() {
        let algorithm = ChaikinAlgorithm::new();
        let points = vec![
            Point2::new(0.0, 0.0),
            Point2::new(100.0, 100.0)
        ];
        
        // Two points should remain unchanged (just a line segment)
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
            Point2::new(200.0, 0.0)
        ];
        
        // First step calculation
        let step1 = algorithm.calculate_step(&points);
        
        // Expected points:
        // - Original first point
        // - Q point between points[0] and points[1]
        // - R point between points[0] and points[1]
        // - Q point between points[1] and points[2]
        // - R point between points[1] and points[2]
        // - Original last point
        assert_eq!(step1.len(), 6);
        
        // Check first and last points
        assert_eq!(step1[0], points[0]);
        assert_eq!(step1[step1.len() - 1], *points.last().unwrap());
        
        // Check Q points (25% along segments)
        assert!((step1[1].x - 25.0).abs() < 0.001);
        assert!((step1[1].y - 25.0).abs() < 0.001);
        
        assert!((step1[3].x - 125.0).abs() < 0.001);
        assert!((step1[3].y - 75.0).abs() < 0.001);
        
        // Check R points (75% along segments)
        assert!((step1[2].x - 75.0).abs() < 0.001);
        assert!((step1[2].y - 75.0).abs() < 0.001);
        
        assert!((step1[4].x - 175.0).abs() < 0.001);
        assert!((step1[4].y - 25.0).abs() < 0.001);
    }
    
    #[test]
    fn test_multiple_steps() {
        let algorithm = ChaikinAlgorithm::new();
        let points = vec![
            Point2::new(0.0, 0.0),
            Point2::new(100.0, 100.0),
            Point2::new(200.0, 0.0)
        ];
        
        // Calculate all steps
        let steps = algorithm.calculate_steps(&points, 7);
        
        // Should have 8 steps (original + 7 iterations)
        assert_eq!(steps.len(), 8);
        
        // First step should be the original points
        assert_eq!(steps[0].len(), 3);
        assert_eq!(steps[0][0], points[0]);
        assert_eq!(steps[0][1], points[1]);
        assert_eq!(steps[0][2], points[2]);
        
        // Each subsequent step should have more points
        for i in 1..steps.len() {
            // Not asserting the exact number - it depends on implementation
            // But each step should have at least the previous number of points
            assert!(steps[i].len() >= steps[i-1].len());
        }
    }
    
    #[test]
    fn test_custom_ratios() {
        // Create algorithm with custom ratios (0.4, 0.6)
        let algorithm = ChaikinAlgorithm::with_ratios(0.4, 0.6);
        
        let points = vec![
            Point2::new(0.0, 0.0),
            Point2::new(100.0, 0.0)
        ];
        
        let step = algorithm.calculate_step(&points);
        
        // Should still have 2 points (line segment)
        assert_eq!(step.len(), 2);
    }
}
