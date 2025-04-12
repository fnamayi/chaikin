mod types;
mod window;

use window::WindowManager;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut window_manager = WindowManager::new(WIDTH, HEIGHT, "Chaikin's Algorithm");

    while window_manager.handle_input() {
        window_manager.clear_buffer();
        window_manager.update();
        window_manager.update_buffer();
    }
}