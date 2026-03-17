use minifb::{Window, WindowOptions, Key, MouseButton, MouseMode};
use std::collections::HashMap;

// Helper function to blend two colors based on alpha
fn blend_colors(background: u32, foreground: u32, alpha: u8) -> u32 {
    let alpha_f = alpha as f32 / 255.0;
    let inv_alpha = 1.0 - alpha_f;
    
    let bg_r = ((background >> 16) & 0xFF) as f32;
    let bg_g = ((background >> 8) & 0xFF) as f32;
    let bg_b = (background & 0xFF) as f32;
    
    let fg_r = ((foreground >> 16) & 0xFF) as f32;
    let fg_g = ((foreground >> 8) & 0xFF) as f32;
    let fg_b = (foreground & 0xFF) as f32;
    
    let r = (fg_r * alpha_f + bg_r * inv_alpha) as u32;
    let g = (fg_g * alpha_f + bg_g * inv_alpha) as u32;
    let b = (fg_b * alpha_f + bg_b * inv_alpha) as u32;
    
    (r << 16) | (g << 8) | b
}

#[derive(Clone, Debug, PartialEq)]
pub enum Theme {
    Auto,
    Dark,
    Light,
    Custom(u32), // Custom background color
}

impl Theme {
    pub fn get_background_color(&self) -> u32 {
        match self {
            Theme::Auto => {
                // Detect OS theme
                let mode = dark_light::detect();
                match mode {
                    dark_light::Mode::Dark => 0x000000, // Black for dark mode
                    dark_light::Mode::Light => 0xFFFFFF, // White for light mode
                    dark_light::Mode::Default => {
                        // Fallback: try manual detection via gsettings
                        if let Ok(output) = std::process::Command::new("gsettings")
                            .args(&["get", "org.gnome.desktop.interface", "color-scheme"])
                            .output()
                        {
                            let scheme = String::from_utf8_lossy(&output.stdout);
                            if scheme.contains("dark") {
                                return 0x000000; // Detected dark
                            } else if scheme.contains("light") {
                                return 0xFFFFFF; // Detected light
                            }
                        }
                        // If detection completely fails, default to light (not dark)
                        0xFFFFFF
                    }
                }
            }
            Theme::Dark => 0x000000,
            Theme::Light => 0xFFFFFF,
            Theme::Custom(color) => *color,
        }
    }
}

#[derive(Clone, Debug)]
pub enum UIComponent {
    Button {
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        action: String, // "close", "minimize", "maximize"
        enabled: bool,
    },
    Panel {
        id: String,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color: u32,
    },
    Label {
        id: String,
        x: i32,
        y: i32,
        text: String,
    },
}

impl UIComponent {
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        match self {
            UIComponent::Button { x: bx, y: by, width: bw, height: bh, .. } => {
                x >= *bx as f32 && x <= (*bx + *bw) as f32 &&
                y >= *by as f32 && y <= (*by + *bh) as f32
            }
            UIComponent::Panel { x: px, y: py, width: pw, height: ph, .. } => {
                x >= *px as f32 && x <= (*px + *pw) as f32 &&
                y >= *py as f32 && y <= (*py + *ph) as f32
            }
            _ => false,
        }
    }
    
    pub fn get_action(&self) -> Option<String> {
        match self {
            UIComponent::Button { action, enabled, .. } => {
                if *enabled {
                    Some(action.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WindowControls {
    pub close_enabled: bool,
    pub minimize_enabled: bool,
    pub maximize_enabled: bool,
    pub resize_enabled: bool,
    pub drag_enabled: bool,
    pub theme: Theme,
}

impl WindowControls {
    pub fn new() -> Self {
        WindowControls {
            close_enabled: true,
            minimize_enabled: true,
            maximize_enabled: true,
            resize_enabled: true,
            drag_enabled: true,
            theme: Theme::Auto,
        }
    }
    
    pub fn disable_all() -> Self {
        WindowControls {
            close_enabled: false,
            minimize_enabled: false,
            maximize_enabled: false,
            resize_enabled: false,
            drag_enabled: false,
            theme: Theme::Auto,
        }
    }
}

pub struct WindowState {
    pub window: Window,
    pub controls: WindowControls,
    pub components: Vec<UIComponent>,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub original_size: (usize, usize),
    pub drag_offset: Option<(i32, i32)>,
    pub last_mouse_down: bool,
}

impl WindowState {
    pub fn new(window: Window, controls: WindowControls) -> Self {
        let size = window.get_size();
        let width = size.0 as i32;
        
        // Create default UI components based on window size
        let mut components = Vec::new();
        
        // Close button (rightmost)
        if controls.close_enabled {
            components.push(UIComponent::Button {
                id: "close".to_string(),
                x: width - 40,
                y: 5,
                width: 35,
                height: 25,
                action: "close".to_string(),
                enabled: true,
            });
        }
        
        // Maximize button (middle)
        if controls.maximize_enabled {
            components.push(UIComponent::Button {
                id: "maximize".to_string(),
                x: width - 80,
                y: 5,
                width: 35,
                height: 25,
                action: "maximize".to_string(),
                enabled: true,
            });
        }
        
        // Minimize button (leftmost of the three)
        if controls.minimize_enabled {
            components.push(UIComponent::Button {
                id: "minimize".to_string(),
                x: width - 120,
                y: 5,
                width: 35,
                height: 25,
                action: "minimize".to_string(),
                enabled: true,
            });
        }
        
        WindowState {
            window,
            controls,
            components,
            is_minimized: false,
            is_maximized: false,
            original_size: size,
            drag_offset: None,
            last_mouse_down: false,
        }
    }
    
    pub fn find_component_at(&self, x: f32, y: f32) -> Option<&UIComponent> {
        // Check components in reverse order (top layer first)
        for component in self.components.iter().rev() {
            if component.contains_point(x, y) {
                return Some(component);
            }
        }
        None
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub color: u32,
    pub theme: Theme,
    pub alpha: u8, // 0 = fully transparent, 255 = fully opaque
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let theme = Theme::Auto;
        let bg_color = theme.get_background_color();
        Canvas {
            width,
            height,
            buffer: vec![bg_color; width * height],  // Fill with theme color
            color: 0xFFFFFF, // white
            theme,
            alpha: 255, // fully opaque by default - no blending with window theme
        }
    }
    
    pub fn new_with_theme(width: usize, height: usize, theme: Theme) -> Self {
        let bg_color = theme.get_background_color();
        Canvas {
            width,
            height,
            buffer: vec![bg_color; width * height],
            color: 0xFFFFFF,
            theme,
            alpha: 255, // fully opaque by default
        }
    }
    
    pub fn set_alpha(&mut self, alpha: u8) {
        self.alpha = alpha;
    }
    
    pub fn set_theme(&mut self, theme: Theme) {
        let bg_color = theme.get_background_color();
        self.theme = theme;
        self.clear(bg_color);
    }

    pub fn clear(&mut self, color: u32) {
        for pixel in &mut self.buffer {
            *pixel = color;
        }
    }
    
    pub fn clear_transparent(&mut self) {
        // Clear to theme color
        let bg_color = self.theme.get_background_color();
        self.clear(bg_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = (y as usize * self.width) + x as usize;
            if idx < self.buffer.len() {
                // Blend with existing pixel based on alpha
                if self.alpha == 255 {
                    self.buffer[idx] = self.color;
                } else {
                    let existing = self.buffer[idx];
                    self.buffer[idx] = blend_colors(existing, self.color, self.alpha);
                }
            }
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x1;
        let mut y = y1;

        loop {
            self.set_pixel(x, y);
            if x == x2 && y == y2 { break; }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.draw_line(x, y, x + w, y);
        self.draw_line(x + w, y, x + w, y + h);
        self.draw_line(x + w, y + h, x, y + h);
        self.draw_line(x, y + h, x, y);
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        for dy in 0..h {
            for dx in 0..w {
                self.set_pixel(x + dx, y + dy);
            }
        }
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;

        while x <= y {
            self.set_pixel(cx + x, cy + y);
            self.set_pixel(cx - x, cy + y);
            self.set_pixel(cx + x, cy - y);
            self.set_pixel(cx - x, cy - y);
            self.set_pixel(cx + y, cy + x);
            self.set_pixel(cx - y, cy + x);
            self.set_pixel(cx + y, cy - x);
            self.set_pixel(cx - y, cy - x);

            if d < 0 {
                d = d + 4 * x + 6;
            } else {
                d = d + 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }

    pub fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) {
        self.draw_line(x1, y1, x2, y2);
        self.draw_line(x2, y2, x3, y3);
        self.draw_line(x3, y3, x1, y1);
    }
}

pub struct GraphicsState {
    windows: HashMap<String, WindowState>,
    canvases: HashMap<String, Canvas>,
    next_id: usize,
}

impl GraphicsState {
    pub fn new() -> Self {
        GraphicsState {
            windows: HashMap::new(),
            canvases: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn create_window(&mut self, width: usize, height: usize) -> String {
        self.create_window_with_controls(width, height, WindowControls::new())
    }

    pub fn create_window_with_controls(&mut self, width: usize, height: usize, controls: WindowControls) -> String {
        let id = format!("window_{}", self.next_id);
        self.next_id += 1;

        let mut opts = WindowOptions::default();
        opts.borderless = false;
        opts.title = true;
        opts.resize = controls.resize_enabled;
        
        let window = Window::new(
            &format!("Protlin Graphics - Window {}", self.next_id - 1),
            width,
            height,
            opts,
        ).unwrap_or_else(|e| {
            panic!("Unable to create window: {}", e);
        });

        let window_state = WindowState::new(window, controls);
        self.windows.insert(id.clone(), window_state);
        id
    }
    
    pub fn set_window_control(&mut self, window_id: &str, control: &str, enabled: bool) -> bool {
        if let Some(win_state) = self.windows.get_mut(window_id) {
            match control {
                "close" => win_state.controls.close_enabled = enabled,
                "minimize" => win_state.controls.minimize_enabled = enabled,
                "maximize" => win_state.controls.maximize_enabled = enabled,
                "resize" => win_state.controls.resize_enabled = enabled,
                "drag" => win_state.controls.drag_enabled = enabled,
                _ => return false,
            }
            true
        } else {
            false
        }
    }
    
    pub fn set_window_theme(&mut self, window_id: &str, theme: Theme) -> bool {
        if let Some(win_state) = self.windows.get_mut(window_id) {
            win_state.controls.theme = theme;
            true
        } else {
            false
        }
    }

    pub fn create_canvas(&mut self, width: usize, height: usize) -> String {
        let id = format!("canvas_{}", self.next_id);
        self.next_id += 1;

        let canvas = Canvas::new(width, height);
        self.canvases.insert(id.clone(), canvas);
        id
    }
    
    pub fn create_canvas_with_theme(&mut self, width: usize, height: usize, theme: Theme) -> String {
        let id = format!("canvas_{}", self.next_id);
        self.next_id += 1;

        let canvas = Canvas::new_with_theme(width, height, theme);
        self.canvases.insert(id.clone(), canvas);
        id
    }

    pub fn get_canvas_mut(&mut self, id: &str) -> Option<&mut Canvas> {
        self.canvases.get_mut(id)
    }

    pub fn update_window(&mut self, window_id: &str, canvas_id: &str) {
        if let Some(canvas) = self.canvases.get(canvas_id) {
            if let Some(win_state) = self.windows.get_mut(window_id) {
                if !win_state.is_minimized {
                    let theme_color = win_state.controls.theme.get_background_color();
                    
                    // Handle different alpha levels
                    if canvas.alpha == 0 {
                        // 0% transparent = 100% theme background
                        let theme_buffer = vec![theme_color; canvas.width * canvas.height];
                        win_state.window.update_with_buffer(&theme_buffer, canvas.width, canvas.height)
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to update window: {}", e);
                            });
                    } else if canvas.alpha < 255 {
                        // Blend canvas with window theme based on alpha
                        let mut blended_buffer = canvas.buffer.clone();
                        
                        for pixel in blended_buffer.iter_mut() {
                            *pixel = blend_colors(theme_color, *pixel, canvas.alpha);
                        }
                        
                        win_state.window.update_with_buffer(&blended_buffer, canvas.width, canvas.height)
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to update window: {}", e);
                            });
                    } else {
                        // alpha = 255: fully opaque, show canvas only
                        win_state.window.update_with_buffer(&canvas.buffer, canvas.width, canvas.height)
                            .unwrap_or_else(|e| {
                                eprintln!("Failed to update window: {}", e);
                            });
                    }
                }
            }
        }
    }

    pub fn update_all_windows(&mut self) {
        let window_ids: Vec<String> = self.windows.keys().cloned().collect();
        let canvas_ids: Vec<String> = self.canvases.keys().cloned().collect();
        
        // Try to match windows with canvases by index
        for (i, window_id) in window_ids.iter().enumerate() {
            if i < canvas_ids.len() {
                self.update_window(window_id, &canvas_ids[i]);
            }
        }
    }

    pub fn keep_windows_open(&mut self) {
        loop {
            let mut all_closed = true;
            let mut windows_to_close = Vec::new();
            let mut windows_to_minimize = Vec::new();
            let mut windows_to_maximize = Vec::new();
            
            for (window_id, win_state) in self.windows.iter_mut() {
                if win_state.window.is_open() && !win_state.window.is_key_down(Key::Escape) {
                    let mouse_down = win_state.window.get_mouse_down(MouseButton::Left);
                    
                    // Detect click: mouse was down last frame, now released
                    if win_state.last_mouse_down && !mouse_down {
                        if let Some((x, y)) = win_state.window.get_mouse_pos(MouseMode::Clamp) {
                            // Use component system for hit detection
                            if let Some(component) = win_state.find_component_at(x, y) {
                                if let Some(action) = component.get_action() {
                                    match action.as_str() {
                                        "close" => windows_to_close.push(window_id.clone()),
                                        "minimize" => windows_to_minimize.push(window_id.clone()),
                                        "maximize" => windows_to_maximize.push(window_id.clone()),
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    
                    // Update mouse state for next frame
                    win_state.last_mouse_down = mouse_down;
                    
                    win_state.window.update();
                    all_closed = false;
                }
            }
            
            // Process window actions
            for window_id in windows_to_close {
                self.windows.remove(&window_id);
            }
            
            for window_id in windows_to_minimize {
                if let Some(win_state) = self.windows.get_mut(&window_id) {
                    win_state.is_minimized = !win_state.is_minimized;
                }
            }
            
            for window_id in windows_to_maximize {
                if let Some(win_state) = self.windows.get_mut(&window_id) {
                    win_state.is_maximized = !win_state.is_maximized;
                }
            }
            
            if all_closed {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    pub fn close_window(&mut self, window_id: &str) -> bool {
        self.windows.remove(window_id).is_some()
    }

    pub fn close_all_windows(&mut self) {
        self.windows.clear();
    }

    pub fn is_window_open(&self, window_id: &str) -> bool {
        self.windows.get(window_id).map_or(false, |ws| ws.window.is_open())
    }

    pub fn update_single_window(&mut self, window_id: &str, canvas_id: &str) -> bool {
        if let Some(canvas) = self.canvases.get(canvas_id) {
            if let Some(win_state) = self.windows.get_mut(window_id) {
                if win_state.window.is_open() && !win_state.is_minimized {
                    win_state.window.update_with_buffer(&canvas.buffer, canvas.width, canvas.height)
                        .unwrap_or_else(|e| {
                            eprintln!("Failed to update window: {}", e);
                        });
                    return true;
                }
            }
        }
        false
    }
}

// Global mutable state - not thread-safe but works for single-threaded interpreter
static mut GRAPHICS_STATE: Option<GraphicsState> = None;

fn get_graphics_state() -> &'static mut GraphicsState {
    unsafe {
        if GRAPHICS_STATE.is_none() {
            GRAPHICS_STATE = Some(GraphicsState::new());
        }
        GRAPHICS_STATE.as_mut().unwrap()
    }
}

pub fn create_window(width: usize, height: usize) -> String {
    get_graphics_state().create_window(width, height)
}

pub fn create_canvas(width: usize, height: usize) -> String {
    get_graphics_state().create_canvas(width, height)
}

pub fn create_canvas_with_theme(width: usize, height: usize, theme: &str) -> String {
    let theme_enum = match theme {
        "auto" => Theme::Auto,
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        _ => Theme::Auto,
    };
    get_graphics_state().create_canvas_with_theme(width, height, theme_enum)
}

pub fn set_canvas_theme(canvas_id: &str, theme: &str) {
    let theme_enum = match theme {
        "auto" => Theme::Auto,
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        _ => Theme::Auto,
    };
    let state = get_graphics_state();
    if let Some(canvas) = state.get_canvas_mut(canvas_id) {
        canvas.set_theme(theme_enum);
    }
}

pub fn set_canvas_theme_custom(canvas_id: &str, color: u32) {
    let state = get_graphics_state();
    if let Some(canvas) = state.get_canvas_mut(canvas_id) {
        canvas.set_theme(Theme::Custom(color));
    }
}

pub fn draw_on_canvas<F>(canvas_id: &str, f: F) 
where F: FnOnce(&mut Canvas) {
    let state = get_graphics_state();
    if let Some(canvas) = state.get_canvas_mut(canvas_id) {
        f(canvas);
    }
}

pub fn set_canvas_color(canvas_id: &str, r: u8, g: u8, b: u8) {
    let state = get_graphics_state();
    if let Some(canvas) = state.get_canvas_mut(canvas_id) {
        canvas.color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
    }
}

pub fn update_window(window_id: &str, canvas_id: &str) {
    get_graphics_state().update_window(window_id, canvas_id);
}

pub fn keep_windows_open() {
    get_graphics_state().keep_windows_open();
}

pub fn close_window(window_id: &str) -> bool {
    get_graphics_state().close_window(window_id)
}

pub fn close_all_windows() {
    get_graphics_state().close_all_windows();
}

pub fn is_window_open(window_id: &str) -> bool {
    get_graphics_state().is_window_open(window_id)
}

pub fn update_single_window(window_id: &str, canvas_id: &str) -> bool {
    get_graphics_state().update_single_window(window_id, canvas_id)
}

pub fn update_all_windows() {
    get_graphics_state().update_all_windows();
}

pub fn set_window_control(window_id: &str, control: &str, enabled: bool) -> bool {
    get_graphics_state().set_window_control(window_id, control, enabled)
}

pub fn set_window_theme(window_id: &str, theme: &str) {
    let theme_enum = match theme {
        "auto" => Theme::Auto,
        "dark" => Theme::Dark,
        "light" => Theme::Light,
        _ => Theme::Auto,
    };
    get_graphics_state().set_window_theme(window_id, theme_enum);
}

pub fn set_window_theme_custom(window_id: &str, color: u32) {
    get_graphics_state().set_window_theme(window_id, Theme::Custom(color));
}
