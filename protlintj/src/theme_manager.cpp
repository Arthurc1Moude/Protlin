#include "theme_manager.h"
#include <iostream>

namespace ProtlinTJ {

ThemeManager::ThemeManager() : current_theme(Theme::AUTO) {
    css_provider = gtk_css_provider_new();
    
    // Add CSS provider to default display
    GdkDisplay* display = gdk_display_get_default();
    gtk_style_context_add_provider_for_display(display,
                                              GTK_STYLE_PROVIDER(css_provider),
                                              GTK_STYLE_PROVIDER_PRIORITY_APPLICATION);
}

ThemeManager::~ThemeManager() {
    if (css_provider) {
        g_object_unref(css_provider);
    }
}

void ThemeManager::apply_theme(const std::string& theme_name) {
    if (theme_name == "light") {
        apply_theme(Theme::LIGHT);
    } else if (theme_name == "dark") {
        apply_theme(Theme::DARK);
    } else {
        apply_theme(Theme::AUTO);
    }
}

void ThemeManager::apply_theme(Theme theme) {
    current_theme = theme;
    
    Theme actual_theme = theme;
    if (theme == Theme::AUTO) {
        actual_theme = is_system_dark_theme() ? Theme::DARK : Theme::LIGHT;
    }
    
    load_theme_css(actual_theme);
    apply_css();
    
    // Set GTK theme preference
    GtkSettings* settings = gtk_settings_get_default();
    if (actual_theme == Theme::DARK) {
        g_object_set(settings, "gtk-application-prefer-dark-theme", TRUE, nullptr);
    } else {
        g_object_set(settings, "gtk-application-prefer-dark-theme", FALSE, nullptr);
    }
}

void ThemeManager::cycle_theme() {
    switch (current_theme) {
        case Theme::LIGHT:
            apply_theme(Theme::DARK);
            break;
        case Theme::DARK:
            apply_theme(Theme::AUTO);
            break;
        case Theme::AUTO:
            apply_theme(Theme::LIGHT);
            break;
    }
}

std::string ThemeManager::get_current_theme_name() const {
    switch (current_theme) {
        case Theme::LIGHT:
            return "light";
        case Theme::DARK:
            return "dark";
        case Theme::AUTO:
            return "auto";
        default:
            return "auto";
    }
}

bool ThemeManager::is_system_dark_theme() const {
    GtkSettings* settings = gtk_settings_get_default();
    gboolean prefer_dark = FALSE;
    g_object_get(settings, "gtk-application-prefer-dark-theme", &prefer_dark, nullptr);
    
    // Also check for system color scheme
    gchar* theme_name = nullptr;
    g_object_get(settings, "gtk-theme-name", &theme_name, nullptr);
    
    bool is_dark = prefer_dark;
    if (theme_name) {
        std::string theme_str(theme_name);
        is_dark = is_dark || (theme_str.find("dark") != std::string::npos) ||
                            (theme_str.find("Dark") != std::string::npos);
        g_free(theme_name);
    }
    
    return is_dark;
}

void ThemeManager::detect_system_theme() {
    if (current_theme == Theme::AUTO) {
        apply_theme(Theme::AUTO);
    }
}

void ThemeManager::load_theme_css(Theme theme) {
    std::string css_content;
    
    if (theme == Theme::DARK) {
        css_content = get_dark_theme_css();
    } else {
        css_content = get_light_theme_css();
    }
    
    gtk_css_provider_load_from_data(css_provider, css_content.c_str(), -1);
}

void ThemeManager::apply_css() {
    // CSS is automatically applied when loaded into the provider
}

std::string ThemeManager::get_light_theme_css() const {
    return R"(
/* ProtlinTJ Light Theme */

window {
    background-color: #ffffff;
    color: #2e3440;
}

headerbar {
    background: linear-gradient(to bottom, #f8f9fa, #e9ecef);
    border-bottom: 1px solid #dee2e6;
    color: #2e3440;
}

headerbar button {
    background: transparent;
    border: 1px solid transparent;
    color: #2e3440;
}

headerbar button:hover {
    background: rgba(46, 52, 64, 0.1);
    border-color: rgba(46, 52, 64, 0.2);
}

headerbar button.suggested-action {
    background: linear-gradient(to bottom, #ffc107, #ffb300);
    color: #000000;
    border: 1px solid #ffb300;
}

headerbar button.suggested-action:hover {
    background: linear-gradient(to bottom, #ffb300, #ff8f00);
}

headerbar button.destructive-action {
    background: linear-gradient(to bottom, #dc3545, #c82333);
    color: #ffffff;
    border: 1px solid #c82333;
}

textview {
    background-color: #ffffff;
    color: #2e3440;
    font-family: 'JetBrains Mono', 'Consolas', 'Monaco', monospace;
    font-size: 11pt;
}

textview text {
    background-color: #ffffff;
    color: #2e3440;
}

textview text selection {
    background-color: #88c0d0;
    color: #2e3440;
}

scrolledwindow {
    background-color: #ffffff;
}

paned {
    background-color: #f8f9fa;
}

paned separator {
    background-color: #dee2e6;
    min-width: 1px;
    min-height: 1px;
}

treeview {
    background-color: #f8f9fa;
    color: #2e3440;
}

treeview:selected {
    background-color: #88c0d0;
    color: #2e3440;
}

entry {
    background-color: #ffffff;
    color: #2e3440;
    border: 1px solid #ced4da;
}

entry:focus {
    border-color: #88c0d0;
    box-shadow: 0 0 0 2px rgba(136, 192, 208, 0.25);
}

button {
    background: linear-gradient(to bottom, #ffffff, #f8f9fa);
    color: #2e3440;
    border: 1px solid #ced4da;
}

button:hover {
    background: linear-gradient(to bottom, #f8f9fa, #e9ecef);
    border-color: #adb5bd;
}

button:active {
    background: #e9ecef;
}

/* Syntax highlighting colors for light theme */
.protlin-keyword {
    color: #5e81ac;
    font-weight: bold;
}

.protlin-string {
    color: #a3be8c;
}

.protlin-comment {
    color: #616e88;
    font-style: italic;
}

.protlin-number {
    color: #b48ead;
}

.protlin-function {
    color: #88c0d0;
    font-weight: bold;
}

.protlin-operator {
    color: #81a1c1;
}
)";
}

std::string ThemeManager::get_dark_theme_css() const {
    return R"(
/* ProtlinTJ Dark Theme */

window {
    background-color: #2e3440;
    color: #d8dee9;
}

headerbar {
    background: linear-gradient(to bottom, #3b4252, #2e3440);
    border-bottom: 1px solid #434c5e;
    color: #d8dee9;
}

headerbar button {
    background: transparent;
    border: 1px solid transparent;
    color: #d8dee9;
}

headerbar button:hover {
    background: rgba(216, 222, 233, 0.1);
    border-color: rgba(216, 222, 233, 0.2);
}

headerbar button.suggested-action {
    background: linear-gradient(to bottom, #ebcb8b, #d08770);
    color: #2e3440;
    border: 1px solid #d08770;
}

headerbar button.suggested-action:hover {
    background: linear-gradient(to bottom, #d08770, #bf616a);
}

headerbar button.destructive-action {
    background: linear-gradient(to bottom, #bf616a, #a54247);
    color: #d8dee9;
    border: 1px solid #a54247;
}

textview {
    background-color: #2e3440;
    color: #d8dee9;
    font-family: 'JetBrains Mono', 'Consolas', 'Monaco', monospace;
    font-size: 11pt;
}

textview text {
    background-color: #2e3440;
    color: #d8dee9;
}

textview text selection {
    background-color: #434c5e;
    color: #d8dee9;
}

scrolledwindow {
    background-color: #2e3440;
}

paned {
    background-color: #3b4252;
}

paned separator {
    background-color: #434c5e;
    min-width: 1px;
    min-height: 1px;
}

treeview {
    background-color: #3b4252;
    color: #d8dee9;
}

treeview:selected {
    background-color: #434c5e;
    color: #d8dee9;
}

entry {
    background-color: #3b4252;
    color: #d8dee9;
    border: 1px solid #434c5e;
}

entry:focus {
    border-color: #88c0d0;
    box-shadow: 0 0 0 2px rgba(136, 192, 208, 0.25);
}

button {
    background: linear-gradient(to bottom, #434c5e, #3b4252);
    color: #d8dee9;
    border: 1px solid #4c566a;
}

button:hover {
    background: linear-gradient(to bottom, #4c566a, #434c5e);
    border-color: #5e81ac;
}

button:active {
    background: #3b4252;
}

/* Syntax highlighting colors for dark theme */
.protlin-keyword {
    color: #81a1c1;
    font-weight: bold;
}

.protlin-string {
    color: #a3be8c;
}

.protlin-comment {
    color: #616e88;
    font-style: italic;
}

.protlin-number {
    color: #b48ead;
}

.protlin-function {
    color: #88c0d0;
    font-weight: bold;
}

.protlin-operator {
    color: #81a1c1;
}
)";
}

} // namespace ProtlinTJ