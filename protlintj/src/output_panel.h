#pragma once

#include <gtk/gtk.h>
#include <string>

namespace ProtlinTJ {

class IDE;

class OutputPanel {
public:
    explicit OutputPanel(IDE* ide);
    ~OutputPanel();
    
    GtkWidget* get_widget() const { return notebook; }
    
    void add_message(const std::string& message);
    void add_error(const std::string& error);
    void add_warning(const std::string& warning);
    void clear();
    
    void show_output_tab();
    void show_terminal_tab();
    void show_problems_tab();
    
private:
    IDE* ide;
    GtkWidget* notebook;
    
    // Output tab
    GtkWidget* output_scrolled;
    GtkWidget* output_text_view;
    GtkTextBuffer* output_buffer;
    
    // Terminal tab
    GtkWidget* terminal_scrolled;
    GtkWidget* terminal_text_view;
    GtkTextBuffer* terminal_buffer;
    GtkWidget* terminal_entry;
    
    // Problems tab
    GtkWidget* problems_scrolled;
    GtkWidget* problems_tree_view;
    GtkListStore* problems_store;
    
    void setup_output_tab();
    void setup_terminal_tab();
    void setup_problems_tab();
    
    // Terminal functionality
    void execute_command(const std::string& command);
    void add_terminal_output(const std::string& output);
    
    // Callbacks
    static void on_terminal_entry_activate(GtkEntry* entry, gpointer data);
    static gboolean on_terminal_key_press(GtkWidget* widget, GdkEventKey* event, gpointer data);
};

} // namespace ProtlinTJ