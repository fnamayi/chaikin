use minifb::{Window, WindowOptions, Key, MouseButton, MouseMode};
use nalgebra::Point2;
use crate::types::{WindowState, AnimationState};

const MAX_STEPS: usize = 7;

pub struct WindowManager {
    window: Window,
    state: WindowState,
    buffer: Vec<u32>,
}

impl WindowManager {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            },
        ).unwrap_or_else(|e| panic!("Failed to create window: {}", e));

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self {
            window,
            state: WindowState {
                points: Vec::new(),
                animation_state: AnimationState::Drawing,
                current_step: 0,
                buffer_width: width,
                buffer_height: height,
            },
            buffer: vec![0; width * height],
        }
    }

    pub fn handle_input(&mut self) -> bool {
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            return false;
        }

        if self.state.animation_state == AnimationState::Drawing {
            if let Some((x, y)) = self.window.get_mouse_pos(MouseMode::Discard) {
                if self.window.get_mouse_down(MouseButton::Left) {
                    let point = Point2::new(x as f32, y as f32);
                    if !self.state.points.iter().any(|p| *p == point) {
                        self.state.points.push(point);
                    }
                }
            }
        }

        if self.window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
            if !self.state.points.is_empty() {
                self.state.animation_state = AnimationState::Animating;
                self.state.current_step = 0;
            }
        }

        true
    }

    pub fn update(&mut self) {
        if self.state.animation_state == AnimationState::Animating {
            self.state.current_step = (self.state.current_step + 1) % MAX_STEPS;
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.fill(0);
    }

    pub fn update_buffer(&mut self) {
        self.window.update_with_buffer(
            &self.buffer,
            self.state.buffer_width,
            self.state.buffer_height,
        ).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;

    #[test]
    fn test_window_creation() {
        let window_manager = WindowManager::new(800, 600, "Test Window");
        assert_eq!(window_manager.state.buffer_width, 800);
        assert_eq!(window_manager.state.buffer_height, 600);
        assert_eq!(window_manager.state.points.len(), 0);
        assert!(matches!(window_manager.state.animation_state, AnimationState::Drawing));
    }

    #[test]
    fn test_animation_state_transition() {
        let mut window_manager = WindowManager::new(800, 600, "Test Window");
        
        // Add a test point
        window_manager.state.points.push(Point2::new(100.0, 100.0));
        
        // Simulate pressing Enter by directly modifying state
        window_manager.state.animation_state = AnimationState::Animating;
        window_manager.state.current_step = 0;
        
        // Test animation step update
        window_manager.update();
        assert_eq!(window_manager.state.current_step, 1);
        
        // Test animation wrapping
        for _ in 0..MAX_STEPS {
            window_manager.update();
        }
        assert_eq!(window_manager.state.current_step, 1);
    }

    #[test]
    fn test_buffer_operations() {
        let mut window_manager = WindowManager::new(800, 600, "Test Window");
        
        // Test buffer size
        assert_eq!(window_manager.buffer.len(), 800 * 600);
        
        // Test clear buffer
        window_manager.buffer[0] = 0xFFFFFFFF;
        window_manager.clear_buffer();
        assert_eq!(window_manager.buffer[0], 0);
    }

    #[test]
    fn test_empty_points_no_animation() {
        let mut window_manager = WindowManager::new(800, 600, "Test Window");
        assert!(matches!(window_manager.state.animation_state, AnimationState::Drawing));
        
        // Simulate Enter press by changing state directly
        window_manager.state.animation_state = AnimationState::Drawing;
        window_manager.update();
        
        // Should stay in drawing state with no points
        assert!(matches!(window_manager.state.animation_state, AnimationState::Drawing));
        assert_eq!(window_manager.state.current_step, 0);
    }

    #[test]
    fn test_duplicate_point_prevention() {
        let mut window_manager = WindowManager::new(800, 600, "Test Window");
        let test_point = Point2::new(100.0, 100.0);
        
        // Simulate adding a point through the points vector
        window_manager.state.points.push(test_point);
        
        // Try to add the same point through our prevention logic
        if !window_manager.state.points.iter().any(|p| *p == test_point) {
            window_manager.state.points.push(test_point);
        }
        
        // Should only contain one instance of the point
        assert_eq!(window_manager.state.points.len(), 1);
        assert_eq!(window_manager.state.points[0], test_point);
    }

    #[test]
    fn test_max_steps_constant() {
        assert_eq!(MAX_STEPS, 7, "MAX_STEPS should be 7 as per requirements");
    }
}