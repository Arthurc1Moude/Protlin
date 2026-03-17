#pragma once

#include <gtk/gtk.h>
#include <memory>
#include <string>

namespace ProtlinTJ {

class Editor;
class FileManager;
class OutputPanel;
class ThemeManager;

class IDE {
public:
    IDE();
    ~IDE();
    
    void run();
    void quit();
    
    // UI Components
    void setup_ui();
    void setup_menu();
    void setup_toolbar();
    void setup_panels();
    
    // File operations
    void new_file();
    void open_file();
    void save_file();
    void save_as_file();
    
    // Edit operations
    void cut();
    void copy();
    void paste();
    void find_replace();
    
    // Run operations
    void run_code();
    void stop_execution();
    
    // Theme operations
    void toggle_theme();
    void set_theme(const std::string& theme);
    
    // Getters
    GtkWidget* get_window() const { return window; }
    Editor* get_editor() const { return editor.get(); }
    FileManager* get_file_manager() const { return file_manager.get(); }
    OutputPanel* get_output_panel() const { return output_panel.get(); }
    ThemeManager* get_theme_manager() const { return theme_manager.get(); }

private:
    // Main window
    GtkWidget* window;
    GtkWidget* main_box;
    GtkWidget* header_bar;
    GtkWidget* toolbar;
    GtkWidget* content_paned;
    GtkWidget* sidebar_paned;
    
    // Components
    std::unique_ptr<Editor> editor;
    std::unique_ptr<FileManager> file_manager;
    std::unique_ptr<OutputPanel> output_panel;
    std::unique_ptr<ThemeManager> theme_manager;
    
    // State
    std::string current_file;
    bool is_modified;
    
    // Callbacks
    static void on_window_destroy(GtkWidget* widget, gpointer data);
    static void on_new_file(GtkWidget* widget, gpointer data);
    static void on_open_file(GtkWidget* widget, gpointer data);
    static void on_save_file(GtkWidget* widget, gpointer data);
    static void on_save_as_file(GtkWidget* widget, gpointer data);
    static void on_quit(GtkWidget* widget, gpointer data);
    static void on_cut(GtkWidget* widget, gpointer data);
    static void on_copy(GtkWidget* widget, gpointer data);
    static void on_paste(GtkWidget* widget, gpointer data);
    static void on_find_replace(GtkWidget* widget, gpointer data);
    static void on_run_code(GtkWidget* widget, gpointer data);
    static void on_stop_execution(GtkWidget* widget, gpointer data);
    static void on_toggle_theme(GtkWidget* widget, gpointer data);
    static void on_about(GtkWidget* widget, gpointer data);
};

} // namespace ProtlinTJ