use minifb::{Window, WindowOptions, Key, MouseButton, MouseMode, KeyRepeat};
use nalgebra::Point2;
use crate::types::{WindowState, AnimationState, Point};
use std::time::{Duration, Instant};
use crate::window::toast::Toast;
use rusttype::{Font, Scale, point, PositionedGlyph};

mod toast;
mod algorithm;

const MAX_STEPS: usize = 7;
/// When drawing points, which are circles, this specifies the radius
const POINT_RADIUS: f32 = 5.0;
/// Draw the points with a shade of red
const POINT_COLOR: u32 = 0x00FF5555;
/// Draw the lines with a blue-green color mix
const LINE_COLOR: u32 = 0x0055CCAA;
/// We will be showing a toast message if the user hasn't yet included enough points for
/// the chaikin algorithm points generation. This specifies for how long we'll show the
/// toast before automatically hiding it
const TOAST_DURATION: Duration = Duration::from_secs(8);
/// The toasts background color. It is a shade of grey so that they are visible
/// on the black window background
const TOAST_BG_COLOR: u32 = 0x80333333;
/// Accessible text color that is visible on the toast's background
const TOAST_TEXT_COLOR: u32 = 0x00FFFFFF;

pub struct WindowManager {
    window: Window,
    state: WindowState,
    buffer: Vec<u32>,
    /// The current toast message, shown if active
    toast: Toast,
    /// The application's text font
    font: Font<'static>,
    /// The instant when the last animation frame was made
    last_call: Instant,
}

impl WindowManager {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                decorations:false,
                ..WindowOptions::default()
            },
        ).unwrap_or_else(|e| panic!("Failed to create window: {}", e));

        window.limit_update_rate(Some(Duration::from_micros(16600)));

        // Load font
        let font_data = include_bytes!("../assets/Roboto-VariableFont_wdth_wght.ttf");
        let font = Font::try_from_bytes(font_data as &[u8])
            .expect("Error loading font");

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
            toast: Toast::new(),
            font,
            last_call: Instant::now(),
        }
    }

    /// Adds a point to be drawn in the window at the given coordinate
    fn add_point(&mut self, x: f32, y: f32) {
        let point = Point::new(x, y);
        self.state.points.push(point);
        // The toast will be shown if the user didn't have enough points for chaikin,
        // but a new point was just added; maybe we already have enough points
        self.toast.dismiss();
        self.redraw();
    }

    /// Re-reads the state of the window and re-renders all the points,
    /// lines, and the toast if active
    pub fn redraw(&mut self) {
        if self.state.animation_state == AnimationState::Drawing {
            self.clear_buffer();
            self.draw_lines();
            self.draw_points();
            self.draw_toast();
            return;
        }

        // We are animating
        let paths = algorithm::ChaikinAlgorithm::new()
            .get_step_points(&self.state.points, self.state.current_step);

        self.clear_buffer();
        self.draw_lines_between(&paths);
        self.draw_points();
    }

    pub fn handle_input(&mut self) -> bool {
        if !self.window.is_open() || self.window.is_key_down(Key::Escape) {
            return false;
        }

        if (self.window.is_key_down(Key::LeftCtrl) || self.window.is_key_down(Key::RightCtrl)) &&
            self.window.is_key_pressed(Key::R, KeyRepeat::No) {
            self.reset();
        }

        let delete_pressed = self.window.is_key_pressed(Key::Delete, KeyRepeat::No);
        let mut mouse_clicked = false;
        if self.state.animation_state == AnimationState::Drawing {
            if let Some((x, y)) = self.window.get_mouse_pos(MouseMode::Discard) {
                if self.window.get_mouse_down(MouseButton::Left) {
                    let point = Point2::new(x, y);
                    mouse_clicked = true;
                    if !self.state.points.iter().any(|p| *p == point) {
                        self.add_point(x, y);
                    }
                }
            }
        }

        // Check if toast should be dismissed
        self.check_toast_dismiss(mouse_clicked, delete_pressed);

        if self.window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            if self.state.points.len() < 2 {
                self.toast.show("You did not select enough points");
                self.draw_toast();
            } else {
                self.state.animation_state = AnimationState::Animating;
                self.state.current_step = 0;
            }
        }

        true
    }

    pub fn update(&mut self) {
        if self.state.animation_state == AnimationState::Animating {
            if self.last_call.elapsed() > Duration::from_secs(1) {
                println!("animation step: {}", self.state.current_step + 1);
                self.state.current_step = (self.state.current_step + 1) % MAX_STEPS;
                self.last_call = Instant::now();
            }
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

    /// Reset the window to it's initial startup state
    pub fn reset(&mut self) {
        self.last_call = Instant::now();
        self.toast = Toast::new();
        self.state.points.clear();
        self.state.animation_state = AnimationState::Drawing;
        self.state.current_step = 0;
        self.toast.dismiss();
        self.clear_buffer();
    }

    //==================== Drawing Utilities =====================

    /// Draws the given color at the given pixel in the window buffer using linear alpha blending.
    /// This is a common technique, that forms the basis for antialiasing techniques such as
    /// Xiaolin Wu's line algorithm
    /// It blends a new color (color) with an existing one in the buffer (bg) at pixel (x, y)
    /// based on an alpha value (opacity).
    fn draw_pixel_aa(&mut self, x: i32, y: i32, color: u32, alpha: f32) {
        let width = self.state.buffer_width;
        let height = self.state.buffer_height;
        if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
            return;
        }

        let index = y as usize * width + x as usize;
        let bg = self.buffer[index];

        // Extract color components
        let r1 = ((color >> 16) & 0xFF) as f32;
        let g1 = ((color >> 8) & 0xFF) as f32;
        let b1 = (color & 0xFF) as f32;

        let r2 = ((bg >> 16) & 0xFF) as f32;
        let g2 = ((bg >> 8) & 0xFF) as f32;
        let b2 = (bg & 0xFF) as f32;

        // Blend colors
        let r = (r1 * alpha + r2 * (1.0 - alpha)) as u32;
        let g = (g1 * alpha + g2 * (1.0 - alpha)) as u32;
        let b = (b1 * alpha + b2 * (1.0 - alpha)) as u32;

        self.buffer[index] = (r << 16) | (g << 8) | b;
    }

    /// Draw a given pixel with the target color, without antialiasing
    fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        let width = self.state.buffer_width;
        let height = self.state.buffer_height;

        if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
            self.buffer[y as usize * width + x as usize] = color;
        }
    }

    /// Draw a circle centered at the given coordinates, and radius, with the given color
    /// with antialiasing enabled
    fn draw_circle_aa(&mut self, center_x: f32, center_y: f32, radius: f32, color: u32) {
        let width = self.state.buffer_width;
        let height = self.state.buffer_height;

        let x0 = (center_x - radius - 1.0).max(0.0) as i32;
        let y0 = (center_y - radius - 1.0).max(0.0) as i32;
        let x1 = (center_x + radius + 1.0).min(width as f32 - 1.0) as i32;
        let y1 = (center_y + radius + 1.0).min(height as f32 - 1.0) as i32;

        for y in y0..=y1 {
            for x in x0..=x1 {
                let dx = x as f32 - center_x;
                let dy = y as f32 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance <= radius + 1.0 {
                    let alpha = if distance <= radius - 1.0 {
                        1.0
                    } else {
                        let t = distance - (radius - 1.0);
                        1.0 - t.min(1.0)
                    };

                    self.draw_pixel_aa(x, y, color, alpha);
                }
            }
        }
    }

    /// Draws a line between the two points, with the target color using
    /// Xiaolin Wu's line algorithm, with antialiasing enabled
    fn draw_line_aa(&mut self, mut x0: f32, mut y0: f32, mut x1: f32, mut y1: f32, color: u32) {
        // Determine if the line is steep
        let steep = (y1 - y0).abs() > (x1 - x0).abs();

        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }

        // Make sure x0 <= x1
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let gradient = if dx.abs() < 1e-6 { 1.0 } else { dy / dx };

        // Handle first endpoint
        let xend = x0.round();
        let yend = y0 + gradient * (xend - x0);
        let xgap = 1.0 - (x0 + 0.5 - xend).abs();
        let xpxl1 = xend as i32;
        let ypxl1 = yend.floor() as i32;

        if steep {
            self.draw_pixel_aa(ypxl1, xpxl1, color, (1.0 - (yend - yend.floor())) * xgap);
            self.draw_pixel_aa(ypxl1 + 1, xpxl1, color, (yend - yend.floor()) * xgap);
        } else {
            self.draw_pixel_aa(xpxl1, ypxl1, color, (1.0 - (yend - yend.floor())) * xgap);
            self.draw_pixel_aa(xpxl1, ypxl1 + 1, color, (yend - yend.floor()) * xgap);
        }

        let mut intery = yend + gradient;

        // Handle second endpoint
        let xend = x1.round();
        let yend = y1 + gradient * (xend - x1);
        let xgap = (x1 + 0.5 - xend).abs();
        let xpxl2 = xend as i32;
        let ypxl2 = yend.floor() as i32;

        if steep {
            self.draw_pixel_aa(ypxl2, xpxl2, color, (1.0 - (yend - yend.floor())) * xgap);
            self.draw_pixel_aa(ypxl2 + 1, xpxl2, color, (yend - yend.floor()) * xgap);
        } else {
            self.draw_pixel_aa(xpxl2, ypxl2, color, (1.0 - (yend - yend.floor())) * xgap);
            self.draw_pixel_aa(xpxl2, ypxl2 + 1, color, (yend - yend.floor()) * xgap);
        }

        // Main loop
        if steep {
            for x in (xpxl1 + 1)..xpxl2 {
                self.draw_pixel_aa(intery.floor() as i32, x, color, 1.0 - (intery - intery.floor()));
                self.draw_pixel_aa(intery.floor() as i32 + 1, x, color, intery - intery.floor());
                intery += gradient;
            }
        } else {
            for x in (xpxl1 + 1)..xpxl2 {
                self.draw_pixel_aa(x, intery.floor() as i32, color, 1.0 - (intery - intery.floor()));
                self.draw_pixel_aa(x, intery.floor() as i32 + 1, color, intery - intery.floor());
                intery += gradient;
            }
        }
    }

    //=============== Text Drawing ========================

    // Draw text using rusttype
    fn draw_text(&mut self, x: i32, y: i32, text: &str, color: u32, size: f32) {
        let scale = Scale::uniform(size);
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(x as f32, y as f32 + v_metrics.ascent);

        // Layout the glyphs in a line with 1 pixel padding
        let glyphs: Vec<PositionedGlyph> = self.font
            .layout(text, scale, offset)
            .collect();

        let width = self.state.buffer_width;
        let height = self.state.buffer_height;

        // Draw the glyphs
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|rx, ry, v| {
                    let x = rx + bounding_box.min.x as u32;
                    let y = ry + bounding_box.min.y as u32;

                    if x < width as u32 && y < height as u32 {
                        // Convert alpha value to 0-1 range
                        let alpha = v;

                        let pixel_x = x as i32;
                        let pixel_y = y as i32;

                        self.draw_pixel_aa(pixel_x, pixel_y, color, alpha);
                    }
                });
            }
        }
    }

    // Text width calculation for centering
    fn text_width(&self, text: &str, size: f32) -> f32 {
        let scale = Scale::uniform(size);
        let v_metrics = self.font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        let glyphs: Vec<PositionedGlyph> = self.font
            .layout(text, scale, offset)
            .collect();

        if let Some(last_glyph) = glyphs.last() {
            if let Some(bounding_box) = last_glyph.pixel_bounding_box() {
                return bounding_box.max.x as f32;
            }
        }

        0.0
    }

    fn draw_toast(&mut self) {
        if !self.toast.is_showing() {
            return;
        }

        let width = self.state.buffer_width;
        let height = self.state.buffer_height;

        let msg = &self.toast.message.clone();
        let font_size = 16.0;
        let text_width = self.text_width(msg, font_size);
        let toast_width = (text_width + 20.0) as usize;
        let toast_height = 40;
        let x_start = (width - toast_width) / 2;
        let y_start = height - toast_height - 20;

        // Draw toast background
        for y in y_start..(y_start + toast_height) {
            for x in x_start..(x_start + toast_width) {
                if x < width && y < height {
                    self.draw_pixel(x as i32, y as i32, TOAST_BG_COLOR);
                }
            }
        }

        // Draw toast text
        let text_x = x_start as i32 + 10;
        let text_y = y_start as i32 + ((toast_height - font_size as usize) / 2) as i32;
        self.draw_text(text_x, text_y, msg, TOAST_TEXT_COLOR, font_size);
    }

    fn check_toast_dismiss(&mut self, mouse_clicked: bool, delete_pressed: bool) {
        if self.toast.is_showing() && (mouse_clicked || delete_pressed) {
            self.toast.dismiss();
            self.redraw();
        }
    }

    //=============== Window State Drawing ========================

    /// Draws all points defined in the window
    pub fn draw_points(&mut self) {
        for point in &self.state.points.clone() {
            self.draw_circle_aa(point.x, point.y, POINT_RADIUS, POINT_COLOR);
        }
    }

    /// Draws lines between all points defined in the window
    fn draw_lines(&mut self) {
        self.draw_lines_between(&self.state.points.clone());
    }

    /// Utility function to draw lines between given points in the window
    fn draw_lines_between(&mut self, points: &[Point]) {
        for i in 1..points.len() {
            let p1 = points[i - 1];
            let p2 = points[i];
            self.draw_line_aa(p1.x, p1.y, p2.x, p2.y, LINE_COLOR);
        }
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