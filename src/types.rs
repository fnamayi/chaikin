use nalgebra::Point2;

pub type Point = Point2<f32>;

#[derive(Clone, Copy, PartialEq)]
pub enum AnimationState {
    Drawing,      // User is placing points
    Animating,    // Animation is running
}

pub struct WindowState {
    pub points: Vec<Point>,
    pub animation_state: AnimationState,
    pub current_step: usize,
    pub buffer_width: usize,
    pub buffer_height: usize,
}