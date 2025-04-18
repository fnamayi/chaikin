# Chaikin's Algorithm Animation

An interactive visualization tool that demonstrates Chaikin's curve algorithm, allowing users to create smooth curves from control points through an animated process.

## Features

- Interactive point placement with left-click
- Real-time curve generation
- Step-by-step animation visualization
- Support for multiple iteration steps
- Clean and intuitive interface

## Prerequisites

- Rust (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository:
   ```bash
   git clone https://learn.zone01kisumu.ke/git/steoiro/chaikin
   cd chaikin
   ```

## Usage

Run the application:
```bash
cargo run
```

## Algorithm Overview

Chaikin's algorithm generates a smooth curve by repeatedly replacing each line segment with two shorter ones, creating a progressively smoother curve with each iteration. The implementation uses 7 iterations for optimal smoothness.

## Building

**Debug Build**
```bash
cargo build
```

**Release Build**
```bash
cargo build --release
```

## Performance Considerations

- The algorithm is optimized for real-time interaction
- Release builds provide better performance for smooth animations