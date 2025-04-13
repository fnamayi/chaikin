mod types;
mod window;

use window::WindowManager;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let title = "Chaikin's Algorithm - [Ctrl + R]: Reset - [Escape]: Close";
    let mut window_manager = WindowManager::new(WIDTH, HEIGHT, title);

    while window_manager.handle_input() {
        window_manager.redraw();
        window_manager.update();
        window_manager.update_buffer();
    }
}