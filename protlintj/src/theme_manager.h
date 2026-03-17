#pragma once

#include <gtk/gtk.h>
#include <string>

namespace ProtlinTJ {

enum class Theme {
    LIGHT,
    DARK,
    AUTO
};

class ThemeManager {
public:
    ThemeManager();
    ~ThemeManager();
    
    void apply_theme(const std::string& theme_name);
    void apply_theme(Theme theme);
    void cycle_theme();
    
    Theme get_current_theme() const { return current_theme; }
    std::string get_current_theme_name() const;
    
    // System theme detection
    bool is_system_dark_theme() const;
    void detect_system_theme();
    
private:
    Theme current_theme;
    GtkCssProvider* css_provider;
    
    void load_theme_css(Theme theme);
    void apply_css();
    
    // Theme CSS content
    std::string get_light_theme_css() const;
    std::string get_dark_theme_css() const;
};

} // namespace ProtlinTJ