# ProtlinTJ - Native Protlin IDE

A native, cross-platform IDE for the Protlin programming language built with C++ and GTK4.

## Features

### Clean, Simple UI
- Modern GTK4 interface
- 4-panel layout: File Manager, Editor, Output, Tools
- Minimal, distraction-free design
- Professional appearance

### Theme Support
- **Light Theme**: Clean, bright interface
- **Dark Theme**: Easy on the eyes for long coding sessions  
- **Auto Theme**: Automatically follows system theme preference
- Seamless theme switching with Ctrl+T

### Advanced Editor
- Syntax highlighting for Protlin language
- Line numbers and current line highlighting
- Auto-indentation and smart tabs
- Find and replace functionality
- Cut, copy, paste operations
- Monospace font optimized for coding

### File Management
- Integrated file explorer
- Project tree navigation
- File operations (new, open, save, save as)
- Context menu support
- Protlin file type recognition (.prot)

### Output Panel
- Real-time compilation output
- Error and warning display
- Integrated terminal
- Problems panel for diagnostics

### Protlin Integration
- Native Protlin syntax highlighting
- Keyword recognition for all 471 Protlin keywords
- Function and variable highlighting
- Comment and string highlighting
- Built-in Protlin execution support

## Building

### Prerequisites

- C++17 compatible compiler (GCC 7+ or Clang 5+)
- GTK4 development libraries
- pkg-config
- CMake 3.16+ (optional)

### Ubuntu/Debian
```bash
sudo apt install build-essential libgtk-4-dev pkg-config cmake
```

### Fedora/RHEL
```bash
sudo dnf install gcc-c++ gtk4-devel pkgconfig cmake
```

### Arch Linux
```bash
sudo pacman -S base-devel gtk4 pkgconf cmake
```

### Build with Make
```bash
cd protlintj
make
```

### Build with CMake
```bash
cd protlintj
mkdir build && cd build
cmake ..
make
```

### Run
```bash
# With Make
make run

# With CMake
./protlintj

# Direct execution
./bin/protlintj
```

## Installation

### System Installation
```bash
sudo make install
```

### User Installation
```bash
make install DESTDIR=$HOME/.local
```

### Uninstall
```bash
sudo make uninstall
```

## Usage

### Basic Operations
- **New File**: Ctrl+N or File → New
- **Open File**: Ctrl+O or File → Open  
- **Save File**: Ctrl+S or File → Save
- **Run Code**: F5 or Run → Execute
- **Find/Replace**: Ctrl+F or Edit → Find & Replace
- **Toggle Theme**: Ctrl+T or View → Toggle Theme

### File Types
ProtlinTJ recognizes and provides syntax highlighting for:
- `.prot` - Protlin source files
- All text files with basic editing support

### Keyboard Shortcuts
- **Ctrl+N** - New file
- **Ctrl+O** - Open file
- **Ctrl+S** - Save file
- **Ctrl+Shift+S** - Save as
- **Ctrl+Q** - Quit
- **Ctrl+X** - Cut
- **Ctrl+C** - Copy
- **Ctrl+V** - Paste
- **Ctrl+A** - Select all
- **Ctrl+F** - Find and replace
- **Ctrl+T** - Toggle theme
- **F5** - Run code

## Architecture

### Components

1. **IDE Core** (`ide.cpp/h`)
   - Main application window
   - Menu and toolbar management
   - Component coordination
   - Event handling

2. **Editor** (`editor.cpp/h`)
   - Text editing functionality
   - Syntax highlighting integration
   - Find/replace operations
   - File I/O operations

3. **File Manager** (`file_manager.cpp/h`)
   - Project tree display
   - File system navigation
   - File operations
   - Context menus

4. **Output Panel** (`output_panel.cpp/h`)
   - Compilation output display
   - Terminal integration
   - Problems/diagnostics panel
   - Message logging

5. **Theme Manager** (`theme_manager.cpp/h`)
   - Theme switching logic
   - CSS style management
   - System theme detection
   - Color scheme application

6. **Protlin Highlighter** (`protlin_highlighter.cpp/h`)
   - Syntax highlighting engine
   - Protlin language rules
   - Theme-aware coloring
   - Performance optimization

### Design Principles

- **Native Performance**: C++ with GTK4 for optimal speed
- **Clean Architecture**: Modular design with clear separation
- **Memory Efficient**: Smart pointers and RAII patterns
- **Cross-Platform**: Works on Linux, macOS, Windows
- **Extensible**: Plugin-ready architecture

## Development

### Code Style
- C++17 standard
- RAII and smart pointers
- Consistent naming conventions
- Comprehensive error handling

### Contributing
1. Fork the repository
2. Create feature branch
3. Follow existing code style
4. Add tests if applicable
5. Submit pull request

### Debugging
```bash
make debug
gdb ./bin/protlintj
```

## License

Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.

Part of the Protlin™ programming language ecosystem.

## Support

- **Issues**: GitHub Issues
- **Documentation**: Built-in help system
- **Community**: Protlin community forums

---

**Built with C++ and GTK4 for the Protlin community**