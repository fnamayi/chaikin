# Chaikin's Algorithm Animation

The project implements Chaikin's curve algorithm as an interactive animation. Users can place control points on a canvas, and the application will animate the steps of the curve generation process.

## Team Roles

### steoiro: Window Management and User Input
- Responsible for: `window.rs`
- Handles window creation, events, input processing
- Manages user interactions (mouse clicks, keyboard)
- Tasks:
  - Implement window creation and management
  - Handle mouse input for point placement
  - Process keyboard events (Enter, Escape)
  - Implement the drawing primitives (pixels, lines, circles)

### Person 2: Chaikin's Algorithm 
- Responsible for: `algorithm.rs`
- Implements the core curve generation algorithm
- Tasks:
  - Implement Chaikin's curve generation algorithm
  - Calculate points for each step of the animation
  - Optimize the algorithm for smooth performance
  - Handle edge cases (single point, two points, etc.)

### Person 3: Rendering and Animation
- Responsible for: `renderer.rs`
- Handles the visual representation and animation
- Tasks:
  - Implement the drawing of points and lines
  - Manage animation timing and transitions
  - Add visual feedback elements
  - Handle the animation loop and state

## Getting Started

1. Clone the repository:
```bash
git clone https://learn.zone01kisumu.ke/git/steoiro/chaikin
cd chaikin
```

2. Each team member should work on their assigned file.

3. Run the project:
```bash
cargo run
```

## How It Works

- Left-click on the canvas to place control points
- Press Enter to start the animation (requires at least 3 points)
- The animation will cycle through 7 steps of Chaikin's algorithm
- Press Escape to quit

## Implementation Details

### Chaikin's Algorithm
Chaikin's algorithm creates a smooth curve by:
1. For each line segment between two control points, create two new points:
   - One at 1/4 of the way along the segment
   - One at 3/4 of the way along the segment
2. Replace the original points with these new points
3. Repeat for each step of the animation

### Project Structure
- `main.rs`: Entry point and application loop
- `types.rs`: Shared data structures
- `window.rs`: Window management and input handling
- `algorithm.rs`: Chaikin's algorithm implementation
- `renderer.rs`: Rendering and animation

## Compilation and Execution

```bash
# Build in debug mode
cargo build

# Build in release mode for better performance
cargo build --release

# Run the application
cargo run
```
