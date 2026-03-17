#pragma once

#include <gtk/gtk.h>
#include <string>
#include <vector>

namespace ProtlinTJ {

class IDE;

class FileManager {
public:
    explicit FileManager(IDE* ide);
    ~FileManager();
    
    GtkWidget* get_widget() const { return scrolled_window; }
    
    void refresh();
    void set_root_path(const std::string& path);
    std::string get_root_path() const { return root_path; }
    
    // File operations
    void create_new_file();
    void create_new_folder();
    void delete_selected();
    void rename_selected();
    
private:
    IDE* ide;
    GtkWidget* scrolled_window;
    GtkWidget* tree_view;
    GtkTreeStore* tree_store;
    GtkTreeSelection* selection;
    
    std::string root_path;
    
    void setup_tree_view();
    void populate_tree();
    void add_directory_to_tree(const std::string& path, GtkTreeIter* parent);
    
    // Callbacks
    static void on_row_activated(GtkTreeView* tree_view, GtkTreePath* path,
                                GtkTreeViewColumn* column, gpointer data);
    static void on_selection_changed(GtkTreeSelection* selection, gpointer data);
    static gboolean on_button_press(GtkWidget* widget, GdkEventButton* event, gpointer data);
    static void on_context_menu_new_file(GtkWidget* widget, gpointer data);
    static void on_context_menu_new_folder(GtkWidget* widget, gpointer data);
    static void on_context_menu_delete(GtkWidget* widget, gpointer data);
    static void on_context_menu_rename(GtkWidget* widget, gpointer data);
};

} // namespace ProtlinTJ