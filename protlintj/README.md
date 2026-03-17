# ProtlinTJ - The Protlin IDE

**Born from an Egg, Maximum Myriad** 🥚

ProtlinTJ is a modern, web-based Integrated Development Environment (IDE) specifically designed for the Protlin programming language. It features a beautiful, intuitive interface with four main sections optimized for Protlin development.

## 🎯 Features

### 4 Main Sections

1. **📁 Explorer & Project Tree**
   - File and folder navigation
   - Project structure overview
   - Code outline view
   - Quick file access

2. **💻 Code Editor**
   - Syntax highlighting for Protlin
   - Auto-completion and snippets
   - Line numbers and code folding
   - Multi-tab support
   - Auto-closing brackets and quotes

3. **📺 Output & Terminal**
   - Real-time code execution output
   - Compilation messages
   - Debug information
   - Terminal integration

4. **🔧 Tools & Properties**
   - Graphics tools for canvas operations
   - Code refactoring utilities
   - Protlin-specific snippets
   - Performance monitoring
   - Property inspector

### 🎨 UI Features

- **Modern Design**: Clean, professional interface with egg-themed branding
- **Dark/Light Theme**: Toggle between themes with Ctrl+`
- **Responsive Layout**: Adapts to different screen sizes
- **Smooth Animations**: Polished user experience
- **Keyboard Shortcuts**: Efficient workflow with hotkeys

### 🥚 Protlin Integration

- **Native Protlin Support**: Built specifically for Protlin development
- **Graphics Preview**: Visual canvas operations
- **Code Snippets**: Pre-built Protlin code templates
- **Syntax Highlighting**: Protlin-aware code coloring
- **Error Detection**: Real-time syntax checking

## 🚀 Getting Started

### Quick Start

1. Open `index.html` in your web browser
2. Start coding in the editor (main.prot is loaded by default)
3. Press **F5** or click **Run** to execute your code
4. Use the file explorer to manage your project

### Sample Code

The IDE comes with sample Protlin files:

- `main.prot` - Basic hello world with graphics
- `graphics.prot` - Graphics utilities and egg drawing
- `utils.prot` - Common utility functions

### Keyboard Shortcuts

- **F5** - Run code
- **Ctrl+S** - Save file
- **Ctrl+O** - Open file
- **Ctrl+N** - New file
- **Ctrl+R** - Run code
- **Ctrl+`** - Toggle theme

## 🎨 Graphics Tools

ProtlinTJ includes specialized tools for Protlin's graphics capabilities:

- **Shape Tools**: Rectangle, Circle, Line, Text
- **Canvas Operations**: Create, render, and manipulate canvases
- **Color Picker**: Visual color selection
- **Animation Preview**: Real-time graphics preview

## 📦 Code Snippets

Built-in snippets for common Protlin patterns:

- **function** - Create a new function
- **canvas** - Set up a graphics canvas
- **for** - Create a for loop
- **Custom snippets** - Add your own templates

## 🔧 Technical Details

### Architecture

- **Frontend**: Pure HTML5, CSS3, JavaScript (ES6+)
- **No Dependencies**: Runs entirely in the browser
- **Responsive**: CSS Grid and Flexbox layout
- **Modern Fonts**: JetBrains Mono for code, Inter for UI

### Browser Support

- Chrome 80+
- Firefox 75+
- Safari 13+
- Edge 80+

### File Structure

```
protlintj/
├── index.html          # Main IDE interface
├── styles.css          # Complete styling system
├── script.js           # IDE functionality
└── README.md           # This file
```

## 🎯 Usage Examples

### Creating a Graphics Program

```protlin
// Create a window with golden egg
function main() {
    canvas = create_canvas(800, 600, "My Protlin App")
    
    // Draw protein (white ellipse)
    set_color(canvas, 255, 255, 255)
    draw_ellipse(canvas, 350, 250, 100, 80)
    
    // Draw yolk (golden ellipse)
    set_color(canvas, 255, 215, 0)
    draw_ellipse(canvas, 370, 270, 60, 50)
    
    render(canvas)
}

main()
```

### Using Code Snippets

1. Click on a snippet in the Tools panel
2. The code template is inserted at cursor position
3. Modify the placeholder values
4. Run your code with F5

## 🎨 Customization

### Themes

- **Light Theme**: Clean, bright interface
- **Dark Theme**: Easy on the eyes for long coding sessions
- **Auto-switching**: Remembers your preference

### Layout

- **Responsive**: Automatically adapts to screen size
- **Resizable Panels**: Drag to resize sections
- **Collapsible**: Hide panels you don't need

## 🔮 Future Features

- **Language Server**: Advanced code intelligence
- **Git Integration**: Version control support
- **Plugin System**: Extensible architecture
- **Cloud Sync**: Save projects online
- **Collaborative Editing**: Real-time collaboration
- **Mobile Support**: Touch-optimized interface

## 🤝 Contributing

ProtlinTJ is part of the Protlin ecosystem. Contributions welcome!

1. Fork the repository
2. Create your feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

Copyright © 2026 Moude AI LLC and Moude Corp. All Rights Reserved.

Part of the Protlin™ programming language ecosystem.

---

**Built with ❤️ and 🥚 for the Protlin community**

*"Every great program starts as a simple egg of an idea"*