#pragma once

#include <gtk/gtk.h>
#include <string>

namespace ProtlinTJ {

class IDE;
class ProtlinHighlighter;

class Editor {
public:
    explicit Editor(IDE* ide);
    ~Editor();
    
    GtkWidget* get_widget() const { return scrolled_window; }
    
    // File operations
    void new_file();
    bool load_file(const std::string& filename);
    bool save_file(const std::string& filename);
    
    // Edit operations
    void cut();
    void copy();
    void paste();
    void undo();
    void redo();
    void select_all();
    
    // Search operations
    void show_find_replace();
    void find_next(const std::string& text);
    void find_previous(const std::string& text);
    void replace_current(const std::string& find_text, const std::string& replace_text);
    void replace_all(const std::string& find_text, const std::string& replace_text);
    
    // Text operations
    std::string get_text() const;
    void set_text(const std::string& text);
    void insert_text(const std::string& text);
    
    // Cursor operations
    void goto_line(int line);
    int get_current_line() const;
    int get_current_column() const;
    
    // Settings
    void set_font(const std::string& font_name);
    void set_tab_width(int width);
    void set_show_line_numbers(bool show);
    void set_highlight_current_line(bool highlight);
    void set_word_wrap(bool wrap);
    
private:
    IDE* ide;
    GtkWidget* scrolled_window;
    GtkWidget* text_view;
    GtkTextBuffer* text_buffer;
    
    std::unique_ptr<ProtlinHighlighter> highlighter;
    
    // Find/Replace dialog
    GtkWidget* find_dialog;
    GtkWidget* find_entry;
    GtkWidget* replace_entry;
    
    void setup_text_view();
    void setup_find_dialog();
    void apply_syntax_highlighting();
    
    // Callbacks
    static void on_text_changed(GtkTextBuffer* buffer, gpointer data);
    static void on_cursor_moved(GtkTextBuffer* buffer, GParamSpec* pspec, gpointer data);
    static void on_find_next_clicked(GtkWidget* widget, gpointer data);
    static void on_find_previous_clicked(GtkWidget* widget, gpointer data);
    static void on_replace_clicked(GtkWidget* widget, gpointer data);
    static void on_replace_all_clicked(GtkWidget* widget, gpointer data);
    static void on_find_dialog_close(GtkWidget* widget, gpointer data);
};

} // namespace ProtlinTJ