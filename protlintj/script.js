// ProtlinTJ IDE JavaScript
class ProtlinTJ {
    constructor() {
        this.currentTheme = 'light';
        this.currentFile = 'main.prot';
        this.files = new Map();
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.setupCodeEditor();
        this.setupFileTree();
        this.setupSnippets();
        this.loadSampleFiles();
        this.updateLineNumbers();
    }

    setupEventListeners() {
        // Theme toggle
        document.getElementById('themeBtn').addEventListener('click', () => {
            this.toggleTheme();
        });

        // Toolbar buttons
        document.getElementById('runBtn').addEventListener('click', () => {
            this.runCode();
        });

        document.getElementById('saveBtn').addEventListener('click', () => {
            this.saveFile();
        });

        document.getElementById('openBtn').addEventListener('click', () => {
            this.openFile();
        });

        document.getElementById('newBtn').addEventListener('click', () => {
            this.newFile();
        });

        // Tab management
        document.querySelectorAll('.tab-close').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                this.closeTab(e.target.closest('.tab'));
            });
        });

        // File tree clicks
        document.querySelectorAll('.tree-item.file').forEach(item => {
            item.addEventListener('click', () => {
                this.openFileFromTree(item);
            });
        });

        // Code editor events
        const editor = document.getElementById('codeEditor');
        editor.addEventListener('input', () => {
            this.updateLineNumbers();
            this.saveCurrentFile();
        });

        editor.addEventListener('scroll', () => {
            this.syncLineNumbers();
        });

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            this.handleKeyboardShortcuts(e);
        });
    }

    setupCodeEditor() {
        const editor = document.getElementById('codeEditor');
        
        // Tab key handling
        editor.addEventListener('keydown', (e) => {
            if (e.key === 'Tab') {
                e.preventDefault();
                const start = editor.selectionStart;
                const end = editor.selectionEnd;
                
                editor.value = editor.value.substring(0, start) + 
                              '    ' + 
                              editor.value.substring(end);
                
                editor.selectionStart = editor.selectionEnd = start + 4;
                this.updateLineNumbers();
            }
        });

        // Auto-closing brackets
        editor.addEventListener('input', (e) => {
            const pos = editor.selectionStart;
            const char = e.data;
            
            if (char === '(' || char === '{' || char === '[' || char === '"') {
                const closing = char === '(' ? ')' : 
                               char === '{' ? '}' : 
                               char === '[' ? ']' : '"';
                
                editor.value = editor.value.substring(0, pos) + 
                              closing + 
                              editor.value.substring(pos);
                
                editor.selectionStart = editor.selectionEnd = pos;
            }
        });
    }

    setupFileTree() {
        // Folder expand/collapse
        document.querySelectorAll('.tree-item.folder').forEach(folder => {
            folder.addEventListener('click', () => {
                folder.classList.toggle('expanded');
            });
        });
    }

    setupSnippets() {
        const snippets = {
            'function': 'function ${1:name}(${2:params}) {\n    ${3:// code here}\n}',
            'canvas': 'canvas = create_canvas(${1:800}, ${2:600}, "${3:Window Title}")\nset_color(canvas, ${4:255}, ${5:255}, ${6:255})\nrender(canvas)',
            'loop': 'for (${1:i} = 0; ${1:i} < ${2:10}; ${1:i}++) {\n    ${3:// code here}\n}'
        };

        document.querySelectorAll('.snippet-item').forEach(item => {
            item.addEventListener('click', () => {
                const snippetType = item.dataset.snippet;
                if (snippets[snippetType]) {
                    this.insertSnippet(snippets[snippetType]);
                }
            });
        });
    }

    loadSampleFiles() {
        this.files.set('main.prot', document.getElementById('codeEditor').value);
        this.files.set('graphics.prot', `// Graphics utilities for Protlin
function create_window(width, height, title) {
    canvas = create_canvas(width, height, title)
    window_set_theme(canvas, "auto")
    return canvas
}

function draw_egg(canvas, x, y, size) {
    // Draw the protein (white)
    set_color(canvas, 255, 255, 255)
    draw_ellipse(canvas, x, y, size, size * 0.8)
    
    // Draw the yolk (golden)
    set_color(canvas, 255, 215, 0)
    draw_ellipse(canvas, x + size * 0.1, y - size * 0.1, size * 0.6, size * 0.5)
}

function animate_egg(canvas, x, y) {
    for (frame = 0; frame < 60; frame++) {
        clear_canvas(canvas)
        bounce_y = y + sin(frame * 0.2) * 10
        draw_egg(canvas, x, bounce_y, 50)
        render(canvas)
        sleep(16) // ~60 FPS
    }
}`);
        
        this.files.set('utils.prot', `// Utility functions for Protlin
function greet(name) {
    println("Hello, " + name + "! 🥚")
    println("Welcome to the Protlin world!")
}

function random_color() {
    r = random(0, 255)
    g = random(0, 255)
    b = random(0, 255)
    return [r, g, b]
}

function measure_time(func) {
    start = current_time()
    result = func()
    end = current_time()
    println("Execution time: " + (end - start) + "ms")
    return result
}`);
    }

    toggleTheme() {
        this.currentTheme = this.currentTheme === 'light' ? 'dark' : 'light';
        document.body.setAttribute('data-theme', this.currentTheme);
        
        const themeBtn = document.getElementById('themeBtn');
        themeBtn.textContent = this.currentTheme === 'light' ? '🌙' : '☀️';
        
        this.addOutputLine('Theme switched to ' + this.currentTheme + ' mode', 'info');
    }

    runCode() {
        const code = document.getElementById('codeEditor').value;
        this.addOutputLine('🚀 Running ' + this.currentFile + '...', 'info');
        
        // Simulate compilation and execution
        setTimeout(() => {
            this.addOutputLine('✅ Compilation successful!', 'success');
            
            // Simulate code execution
            setTimeout(() => {
                if (code.includes('println')) {
                    const matches = code.match(/println\("([^"]+)"\)/g);
                    if (matches) {
                        matches.forEach(match => {
                            const text = match.match(/"([^"]+)"/)[1];
                            this.addOutputLine(text, 'success');
                        });
                    }
                }
                
                if (code.includes('create_canvas')) {
                    this.addOutputLine('🖼️ Canvas created successfully', 'info');
                }
                
                if (code.includes('render')) {
                    this.addOutputLine('🎨 Rendering complete!', 'info');
                }
                
                this.addOutputLine('✨ Program finished successfully', 'success');
                this.updateStatus('🟢 Ready');
            }, 500);
        }, 300);
        
        this.updateStatus('🔄 Running...');
    }

    saveFile() {
        const code = document.getElementById('codeEditor').value;
        this.files.set(this.currentFile, code);
        this.addOutputLine('💾 Saved ' + this.currentFile, 'success');
        this.updateStatus('🟢 Saved');
        
        // Simulate file save
        setTimeout(() => {
            this.updateStatus('🟢 Ready');
        }, 1000);
    }

    openFile() {
        // Create a simple file picker simulation
        const fileNames = Array.from(this.files.keys());
        const fileName = prompt('Enter file name to open:\n\nAvailable files:\n' + fileNames.join('\n'));
        
        if (fileName && this.files.has(fileName)) {
            this.switchToFile(fileName);
            this.addOutputLine('📁 Opened ' + fileName, 'info');
        } else if (fileName) {
            this.addOutputLine('❌ File not found: ' + fileName, 'error');
        }
    }

    newFile() {
        const fileName = prompt('Enter new file name (with .prot extension):');
        if (fileName && fileName.endsWith('.prot')) {
            this.files.set(fileName, '// New Protlin file\n\nfunction main() {\n    println("Hello from ' + fileName + '!")\n}\n\nmain()');
            this.switchToFile(fileName);
            this.addOutputLine('📄 Created new file: ' + fileName, 'success');
        } else if (fileName) {
            this.addOutputLine('❌ Invalid file name. Must end with .prot', 'error');
        }
    }

    switchToFile(fileName) {
        // Save current file
        this.saveCurrentFile();
        
        // Switch to new file
        this.currentFile = fileName;
        document.getElementById('codeEditor').value = this.files.get(fileName) || '';
        this.updateLineNumbers();
        
        // Update active tab (simplified)
        document.querySelectorAll('.tab').forEach(tab => {
            tab.classList.remove('active');
        });
        
        // Update file tree selection
        document.querySelectorAll('.tree-item.file').forEach(item => {
            item.classList.remove('active');
            if (item.querySelector('.tree-label').textContent === fileName) {
                item.classList.add('active');
            }
        });
    }

    openFileFromTree(treeItem) {
        const fileName = treeItem.querySelector('.tree-label').textContent;
        this.switchToFile(fileName);
    }

    closeTab(tab) {
        // Simplified tab closing
        tab.remove();
        this.addOutputLine('🗑️ Closed tab', 'info');
    }

    insertSnippet(snippet) {
        const editor = document.getElementById('codeEditor');
        const pos = editor.selectionStart;
        
        // Simple snippet insertion (without placeholder handling)
        const cleanSnippet = snippet.replace(/\$\{\d+:?([^}]*)\}/g, '$1');
        
        editor.value = editor.value.substring(0, pos) + 
                      cleanSnippet + 
                      editor.value.substring(pos);
        
        editor.focus();
        editor.selectionStart = editor.selectionEnd = pos + cleanSnippet.length;
        this.updateLineNumbers();
    }

    updateLineNumbers() {
        const editor = document.getElementById('codeEditor');
        const lineNumbers = document.querySelector('.line-numbers');
        const lines = editor.value.split('\n').length;
        
        let numbersHtml = '';
        for (let i = 1; i <= Math.max(lines, 15); i++) {
            numbersHtml += `<div class="line-number">${i}</div>`;
        }
        
        lineNumbers.innerHTML = numbersHtml;
    }

    syncLineNumbers() {
        const editor = document.getElementById('codeEditor');
        const lineNumbers = document.querySelector('.line-numbers');
        lineNumbers.scrollTop = editor.scrollTop;
    }

    saveCurrentFile() {
        const code = document.getElementById('codeEditor').value;
        this.files.set(this.currentFile, code);
    }

    addOutputLine(text, type = 'info') {
        const outputContent = document.querySelector('.output-content');
        const time = new Date().toLocaleTimeString();
        
        const line = document.createElement('div');
        line.className = `output-line ${type}`;
        line.innerHTML = `
            <span class="output-time">[${time}]</span>
            <span class="output-text">${text}</span>
        `;
        
        outputContent.appendChild(line);
        outputContent.scrollTop = outputContent.scrollHeight;
    }

    updateStatus(status) {
        const statusElement = document.querySelector('.status-right .status-item');
        if (statusElement) {
            statusElement.textContent = status;
        }
    }

    handleKeyboardShortcuts(e) {
        if (e.ctrlKey || e.metaKey) {
            switch (e.key) {
                case 's':
                    e.preventDefault();
                    this.saveFile();
                    break;
                case 'o':
                    e.preventDefault();
                    this.openFile();
                    break;
                case 'n':
                    e.preventDefault();
                    this.newFile();
                    break;
                case 'r':
                    e.preventDefault();
                    this.runCode();
                    break;
                case '`':
                    e.preventDefault();
                    this.toggleTheme();
                    break;
            }
        }
        
        if (e.key === 'F5') {
            e.preventDefault();
            this.runCode();
        }
    }
}

// Initialize ProtlinTJ when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.protlinTJ = new ProtlinTJ();
    
    // Welcome message
    setTimeout(() => {
        window.protlinTJ.addOutputLine('🥚 Welcome to ProtlinTJ - The Protlin IDE!', 'success');
        window.protlinTJ.addOutputLine('💡 Press F5 or click Run to execute your code', 'info');
        window.protlinTJ.addOutputLine('🎨 Use Ctrl+` to toggle theme', 'info');
    }, 500);
});