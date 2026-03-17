# Getting Started with Protlin™

**Official Quick Start Guide** | **Version 1.0.0** | **Copyright © 2026 Moude AI LLC and Moude Corp**

---

## Legal Notice

⚠️ **IMPORTANT**: Protlin is proprietary software. By using this software, you agree to the terms specified in the [LICENSE](../LICENSE) file. Unauthorized copying or distribution is prohibited.

---

## Table of Contents

1. [System Requirements](#system-requirements)
2. [Installation](#installation)
3. [Your First Program](#your-first-program)
4. [Graphics Programming](#graphics-programming)
5. [Theme System](#theme-system)
6. [Next Steps](#next-steps)

---

## System Requirements

### Minimum Requirements

- **Operating System**: Linux, macOS, or Windows
- **Rust**: Version 1.70 or higher
- **RAM**: 2GB minimum
- **Disk Space**: 500MB available
- **Display**: 1024x768 resolution

### Recommended Requirements

- **Operating System**: Latest stable Linux/macOS/Windows
- **Rust**: Version 1.75 or higher (latest stable)
- **RAM**: 4GB or more
- **Disk Space**: 1GB available
- **Display**: 1920x1080 resolution
- **GPU**: OpenGL 3.3+ support

---

## Installation

### Step 1: Install Rust

If you don't have Rust installed:

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Download from: https://rustup.rs/
```

### Step 2: Clone Repository

```bash
git clone https://github.com/Arthurc1Moude/Protlin.git
cd Protlin
```

### Step 3: Build Protlin

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release
```

### Step 4: Verify Installation

```bash
cargo run examples/hello.prot
```

Expected output:
```
Hello, Protlin!
```

---

## Your First Program

### Hello World

Create a file named `hello.prot`:

```protlin
println("Hello, Protlin!")
```

Run the program:

```bash
cargo run hello.prot
```

### Variables and Types

```protlin
// Variable declaration
name = "Protlin"
version = 1.0
active = true

// Output
println(name)
println(version)
println(active)
```

### Functions

```protlin
// Function definition
function greet(name) {
    return "Hello, " + name + "!"
}

// Function call
message = greet("World")
println(message)
```

---

## Graphics Programming

### Basic Window

Create a file named `window.prot`:

```protlin
// Create window (800x600)
window myWindow 800 600

// Create canvas (same size)
canvas myCanvas 800 600

// Set drawing color (RGB: 255, 0, 0 = Red)
set_color(myCanvas, 255, 0, 0)

// Draw rectangle (x, y, width, height)
draw myCanvas rectangle 100 100 600 400

// Render window with canvas
render myWindow myCanvas
```

Run:
```bash
cargo run window.prot
```

Press ESC to close the window.

### Drawing Shapes

```protlin
window win 800 600
canvas canvas 800 600

// Red rectangle
set_color(canvas, 255, 0, 0)
draw canvas rectangle 50 50 200 150

// Green circle
set_color(canvas, 0, 255, 0)
draw canvas circle 400 300 100

// Blue line
set_color(canvas, 0, 0, 255)
draw canvas line 100 500 700 500

// Yellow triangle
set_color(canvas, 255, 255, 0)
draw canvas triangle 600 100 700 200 650 250

render win canvas
```

---

## Theme System

### Auto Theme Detection

```protlin
window win 400 300
canvas canvas 400 300

// Automatically detect OS theme (dark/light)
window_set_theme(win, "auto")

// Make canvas transparent to show theme
canvas_set_alpha(canvas, 0)

render win canvas
```

### Manual Theme Control

```protlin
window darkWin 400 300
canvas darkCanvas 400 300

// Force dark theme
window_set_theme(darkWin, "dark")
canvas_set_alpha(darkCanvas, 0)

window lightWin 400 300
canvas lightCanvas 400 300

// Force light theme
window_set_theme(lightWin, "light")
canvas_set_alpha(lightCanvas, 0)

render darkWin darkCanvas
render lightWin lightCanvas
```

### Custom Theme Colors

```protlin
window customWin 400 300
canvas customCanvas 400 300

// Custom RGB color (R, G, B)
window_set_theme(customWin, 50, 50, 100)
canvas_set_alpha(customCanvas, 0)

render customWin customCanvas
```

### Alpha Blending

```protlin
window win 400 300
canvas canvas 400 300

// Set theme
window_set_theme(win, "dark")

// Set transparency (0-255)
// 0 = fully transparent (shows theme)
// 128 = 50% transparent
// 255 = fully opaque (hides theme)
canvas_set_alpha(canvas, 128)

// Draw content
set_color(canvas, 255, 100, 100)
draw canvas rectangle 50 50 300 200

render win canvas
```

---

## Next Steps

### Explore Examples

```bash
# Run official examples
cargo run examples/hello.prot
cargo run examples/graphics_demo.prot
cargo run examples/theme_demo.prot
```

### Read Documentation

- [Language Specification](LANGUAGE_SPEC.md) - Complete language reference
- [API Reference](API_REFERENCE.md) - Built-in functions
- [Graphics Guide](GRAPHICS_GUIDE.md) - Advanced graphics
- [Theme System](THEME_SYSTEM.md) - Theme management

### Join Community

- Report bugs: [GitHub Issues](https://github.com/Arthurc1Moude/Protlin/issues)
- Ask questions: [GitHub Discussions](https://github.com/Arthurc1Moude/Protlin/discussions)
- Contribute: See [CONTRIBUTING.md](../CONTRIBUTING.md)

---

## Troubleshooting

### Build Errors

**Problem**: Cargo build fails

**Solution**:
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

### Window Not Appearing

**Problem**: Graphics window doesn't show

**Solution**:
- Ensure you have a display server running (X11/Wayland on Linux)
- Check GPU drivers are installed
- Try running with `--release` flag for better performance

### Theme Not Detecting

**Problem**: Auto theme shows wrong color

**Solution**:
- Theme detection may not work on all systems
- Use manual theme: `window_set_theme(win, "dark")` or `"light"`
- Check system theme settings

---

## Support

For additional help:
- Check [README.md](../README.md)
- Open an issue on GitHub
- Contact @Arthurc1Moude

---

**Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.**

Protlin™ is a registered trademark of Moude AI LLC and Moude Corp.
