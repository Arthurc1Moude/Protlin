#include "ide.h"
#include "editor.h"
#include "file_manager.h"
#include "output_panel.h"
#include "theme_manager.h"
#include <iostream>

namespace ProtlinTJ {

IDE::IDE() : window(nullptr), is_modified(false) {
    // Initialize components
    theme_manager = std::make_unique<ThemeManager>();
    editor = std::make_unique<Editor>(this);
    file_manager = std::make_unique<FileManager>(this);
    output_panel = std::make_unique<OutputPanel>(this);
    
    setup_ui();
}

IDE::~IDE() {
    // GTK widgets are automatically destroyed
}

void IDE::run() {
    gtk_widget_show(window);
    
    GMainLoop* loop = g_main_loop_new(nullptr, FALSE);
    g_main_loop_run(loop);
    g_main_loop_unref(loop);
}

void IDE::quit() {
    gtk_window_destroy(GTK_WINDOW(window));
}

void IDE::setup_ui() {
    // Create main window
    window = gtk_window_new();
    gtk_window_set_title(GTK_WINDOW(window), "ProtlinTJ - Protlin IDE");
    gtk_window_set_default_size(GTK_WINDOW(window), 1200, 800);
    gtk_window_set_icon_name(GTK_WINDOW(window), "text-editor");
    
    // Connect destroy signal
    g_signal_connect(window, "destroy", G_CALLBACK(on_window_destroy), this);
    
    // Create header bar
    header_bar = gtk_header_bar_new();
    gtk_header_bar_set_show_title_buttons(GTK_HEADER_BAR(header_bar), TRUE);
    gtk_header_bar_set_title_widget(GTK_HEADER_BAR(header_bar), 
                                   gtk_label_new("ProtlinTJ"));
    gtk_window_set_titlebar(GTK_WINDOW(window), header_bar);
    
    setup_menu();
    setup_toolbar();
    setup_panels();
    
    // Apply initial theme
    theme_manager->apply_theme("auto");
}

void IDE::setup_menu() {
    // Create menu button
    GtkWidget* menu_button = gtk_menu_button_new();
    gtk_menu_button_set_icon_name(GTK_MENU_BUTTON(menu_button), "open-menu-symbolic");
    gtk_header_bar_pack_end(GTK_HEADER_BAR(header_bar), menu_button);
    
    // Create menu model
    GMenu* menu = g_menu_new();
    
    // File menu
    GMenu* file_menu = g_menu_new();
    g_menu_append(file_menu, "New", "app.new");
    g_menu_append(file_menu, "Open", "app.open");
    g_menu_append(file_menu, "Save", "app.save");
    g_menu_append(file_menu, "Save As", "app.save-as");
    g_menu_append_separator(file_menu);
    g_menu_append(file_menu, "Quit", "app.quit");
    g_menu_append_submenu(menu, "File", G_MENU_MODEL(file_menu));
    
    // Edit menu
    GMenu* edit_menu = g_menu_new();
    g_menu_append(edit_menu, "Cut", "app.cut");
    g_menu_append(edit_menu, "Copy", "app.copy");
    g_menu_append(edit_menu, "Paste", "app.paste");
    g_menu_append_separator(edit_menu);
    g_menu_append(edit_menu, "Find & Replace", "app.find-replace");
    g_menu_append_submenu(menu, "Edit", G_MENU_MODEL(edit_menu));
    
    // Run menu
    GMenu* run_menu = g_menu_new();
    g_menu_append(run_menu, "Run Code", "app.run");
    g_menu_append(run_menu, "Stop", "app.stop");
    g_menu_append_submenu(menu, "Run", G_MENU_MODEL(run_menu));
    
    // View menu
    GMenu* view_menu = g_menu_new();
    g_menu_append(view_menu, "Toggle Theme", "app.toggle-theme");
    g_menu_append_submenu(menu, "View", G_MENU_MODEL(view_menu));
    
    // Help menu
    GMenu* help_menu = g_menu_new();
    g_menu_append(help_menu, "About", "app.about");
    g_menu_append_submenu(menu, "Help", G_MENU_MODEL(help_menu));
    
    gtk_menu_button_set_menu_model(GTK_MENU_BUTTON(menu_button), G_MENU_MODEL(menu));
    
    // Create application and add actions
    GtkApplication* app = gtk_application_new("com.moudeai.protlintj", G_APPLICATION_FLAGS_NONE);
    
    // Add actions
    GSimpleAction* new_action = g_simple_action_new("new", nullptr);
    g_signal_connect(new_action, "activate", G_CALLBACK(on_new_file), this);
    g_action_map_add_action(G_ACTION_MAP(app), G_ACTION(new_action));
    
    GSimpleAction* open_action = g_simple_action_new("open", nullptr);
    g_signal_connect(open_action, "activate", G_CALLBACK(on_open_file), this);
    g_action_map_add_action(G_ACTION_MAP(app), G_ACTION(open_action));
    
    GSimpleAction* save_action = g_simple_action_new("save", nullptr);
    g_signal_connect(save_action, "activate", G_CALLBACK(on_save_file), this);
    g_action_map_add_action(G_ACTION_MAP(app), G_ACTION(save_action));
    
    GSimpleAction* quit_action = g_simple_action_new("quit", nullptr);
    g_signal_connect(quit_action, "activate", G_CALLBACK(on_quit), this);
    g_action_map_add_action(G_ACTION_MAP(app), G_ACTION(quit_action));
    
    GSimpleAction* run_action = g_simple_action_new("run", nullptr);
    g_signal_connect(run_action, "activate", G_CALLBACK(on_run_code), this);
    g_action_map_add_action(G_ACTION_MAP(app), G_ACTION(run_action));
    
    GSimpleAction* theme_action = g_simple_action_new("toggle-theme", nullptr);
    g_signal_connect(theme_action, "activate", G_CALLBACK(on_toggle_theme), this);
    g_action_map_add_action(G_ACTION_MAP(app), G_ACTION(theme_action));
}

void IDE::setup_toolbar() {
    // Create toolbar
    toolbar = gtk_box_new(GTK_ORIENTATION_HORIZONTAL, 6);
    gtk_widget_set_margin_start(toolbar, 6);
    gtk_widget_set_margin_end(toolbar, 6);
    gtk_header_bar_pack_start(GTK_HEADER_BAR(header_bar), toolbar);
    
    // New button
    GtkWidget* new_btn = gtk_button_new_from_icon_name("document-new-symbolic");
    gtk_widget_set_tooltip_text(new_btn, "New File (Ctrl+N)");
    g_signal_connect(new_btn, "clicked", G_CALLBACK(on_new_file), this);
    gtk_box_append(GTK_BOX(toolbar), new_btn);
    
    // Open button
    GtkWidget* open_btn = gtk_button_new_from_icon_name("document-open-symbolic");
    gtk_widget_set_tooltip_text(open_btn, "Open File (Ctrl+O)");
    g_signal_connect(open_btn, "clicked", G_CALLBACK(on_open_file), this);
    gtk_box_append(GTK_BOX(toolbar), open_btn);
    
    // Save button
    GtkWidget* save_btn = gtk_button_new_from_icon_name("document-save-symbolic");
    gtk_widget_set_tooltip_text(save_btn, "Save File (Ctrl+S)");
    g_signal_connect(save_btn, "clicked", G_CALLBACK(on_save_file), this);
    gtk_box_append(GTK_BOX(toolbar), save_btn);
    
    // Separator
    GtkWidget* separator = gtk_separator_new(GTK_ORIENTATION_VERTICAL);
    gtk_box_append(GTK_BOX(toolbar), separator);
    
    // Run button
    GtkWidget* run_btn = gtk_button_new_from_icon_name("media-playback-start-symbolic");
    gtk_widget_set_tooltip_text(run_btn, "Run Code (F5)");
    gtk_button_set_has_frame(GTK_BUTTON(run_btn), TRUE);
    gtk_widget_add_css_class(run_btn, "suggested-action");
    g_signal_connect(run_btn, "clicked", G_CALLBACK(on_run_code), this);
    gtk_box_append(GTK_BOX(toolbar), run_btn);
    
    // Stop button
    GtkWidget* stop_btn = gtk_button_new_from_icon_name("media-playback-stop-symbolic");
    gtk_widget_set_tooltip_text(stop_btn, "Stop Execution");
    gtk_widget_add_css_class(stop_btn, "destructive-action");
    g_signal_connect(stop_btn, "clicked", G_CALLBACK(on_stop_execution), this);
    gtk_box_append(GTK_BOX(toolbar), stop_btn);
}

void IDE::setup_panels() {
    // Create main container
    main_box = gtk_box_new(GTK_ORIENTATION_VERTICAL, 0);
    gtk_window_set_child(GTK_WINDOW(window), main_box);
    
    // Create main paned (horizontal split)
    content_paned = gtk_paned_new(GTK_ORIENTATION_HORIZONTAL);
    gtk_box_append(GTK_BOX(main_box), content_paned);
    
    // Create sidebar paned (vertical split for left side)
    sidebar_paned = gtk_paned_new(GTK_ORIENTATION_VERTICAL);
    gtk_paned_set_start_child(GTK_PANED(content_paned), sidebar_paned);
    gtk_paned_set_position(GTK_PANED(content_paned), 250);
    
    // Add file manager to top of sidebar
    gtk_paned_set_start_child(GTK_PANED(sidebar_paned), file_manager->get_widget());
    
    // Add output panel to bottom of sidebar
    gtk_paned_set_end_child(GTK_PANED(sidebar_paned), output_panel->get_widget());
    gtk_paned_set_position(GTK_PANED(sidebar_paned), 400);
    
    // Add editor to right side
    gtk_paned_set_end_child(GTK_PANED(content_paned), editor->get_widget());
}

// File operations
void IDE::new_file() {
    editor->new_file();
    current_file.clear();
    is_modified = false;
    gtk_header_bar_set_title_widget(GTK_HEADER_BAR(header_bar), 
                                   gtk_label_new("ProtlinTJ - Untitled"));
}

void IDE::open_file() {
    GtkFileChooserNative* dialog = gtk_file_chooser_native_new(
        "Open File", GTK_WINDOW(window), GTK_FILE_CHOOSER_ACTION_OPEN,
        "Open", "Cancel");
    
    // Add file filters
    GtkFileFilter* filter = gtk_file_filter_new();
    gtk_file_filter_set_name(filter, "Protlin Files");
    gtk_file_filter_add_pattern(filter, "*.prot");
    gtk_file_chooser_add_filter(GTK_FILE_CHOOSER(dialog), filter);
    
    GtkFileFilter* all_filter = gtk_file_filter_new();
    gtk_file_filter_set_name(all_filter, "All Files");
    gtk_file_filter_add_pattern(all_filter, "*");
    gtk_file_chooser_add_filter(GTK_FILE_CHOOSER(dialog), all_filter);
    
    if (gtk_native_dialog_run(GTK_NATIVE_DIALOG(dialog)) == GTK_RESPONSE_ACCEPT) {
        GFile* file = gtk_file_chooser_get_file(GTK_FILE_CHOOSER(dialog));
        char* filename = g_file_get_path(file);
        
        if (editor->load_file(filename)) {
            current_file = filename;
            is_modified = false;
            
            char* basename = g_path_get_basename(filename);
            std::string title = "ProtlinTJ - " + std::string(basename);
            gtk_header_bar_set_title_widget(GTK_HEADER_BAR(header_bar), 
                                           gtk_label_new(title.c_str()));
            g_free(basename);
        }
        
        g_free(filename);
        g_object_unref(file);
    }
    
    g_object_unref(dialog);
}

void IDE::save_file() {
    if (current_file.empty()) {
        save_as_file();
        return;
    }
    
    if (editor->save_file(current_file)) {
        is_modified = false;
        output_panel->add_message("File saved: " + current_file);
    }
}

void IDE::save_as_file() {
    GtkFileChooserNative* dialog = gtk_file_chooser_native_new(
        "Save File", GTK_WINDOW(window), GTK_FILE_CHOOSER_ACTION_SAVE,
        "Save", "Cancel");
    
    gtk_file_chooser_set_current_name(GTK_FILE_CHOOSER(dialog), "untitled.prot");
    
    if (gtk_native_dialog_run(GTK_NATIVE_DIALOG(dialog)) == GTK_RESPONSE_ACCEPT) {
        GFile* file = gtk_file_chooser_get_file(GTK_FILE_CHOOSER(dialog));
        char* filename = g_file_get_path(file);
        
        if (editor->save_file(filename)) {
            current_file = filename;
            is_modified = false;
            
            char* basename = g_path_get_basename(filename);
            std::string title = "ProtlinTJ - " + std::string(basename);
            gtk_header_bar_set_title_widget(GTK_HEADER_BAR(header_bar), 
                                           gtk_label_new(title.c_str()));
            g_free(basename);
            
            output_panel->add_message("File saved: " + current_file);
        }
        
        g_free(filename);
        g_object_unref(file);
    }
    
    g_object_unref(dialog);
}

void IDE::run_code() {
    if (!current_file.empty()) {
        save_file();
    }
    
    output_panel->clear();
    output_panel->add_message("Running Protlin code...");
    
    // TODO: Implement actual Protlin execution
    output_panel->add_message("Compilation successful");
    output_panel->add_message("Program executed successfully");
}

void IDE::toggle_theme() {
    theme_manager->cycle_theme();
}

// Static callbacks
void IDE::on_window_destroy(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->quit();
}

void IDE::on_new_file(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->new_file();
}

void IDE::on_open_file(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->open_file();
}

void IDE::on_save_file(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->save_file();
}

void IDE::on_save_as_file(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->save_as_file();
}

void IDE::on_quit(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->quit();
}

void IDE::on_run_code(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->run_code();
}

void IDE::on_stop_execution(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->output_panel->add_message("Execution stopped");
}

void IDE::on_toggle_theme(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->toggle_theme();
}

void IDE::on_cut(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->cut();
}

void IDE::on_copy(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->copy();
}

void IDE::on_paste(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->paste();
}

void IDE::on_find_replace(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    ide->find_replace();
}

void IDE::on_about(GtkWidget* widget, gpointer data) {
    IDE* ide = static_cast<IDE*>(data);
    
    GtkWidget* about = gtk_about_dialog_new();
    gtk_about_dialog_set_program_name(GTK_ABOUT_DIALOG(about), "ProtlinTJ");
    gtk_about_dialog_set_version(GTK_ABOUT_DIALOG(about), "1.0.0");
    gtk_about_dialog_set_comments(GTK_ABOUT_DIALOG(about), 
                                 "The Protlin Programming Language IDE");
    gtk_about_dialog_set_copyright(GTK_ABOUT_DIALOG(about), 
                                  "Copyright © 2026 Moude AI LLC");
    gtk_about_dialog_set_website(GTK_ABOUT_DIALOG(about), 
                                "https://github.com/Arthurc1Moude/Protlin");
    
    gtk_window_set_transient_for(GTK_WINDOW(about), GTK_WINDOW(ide->window));
    gtk_window_present(GTK_WINDOW(about));
}

// Placeholder implementations for edit operations
void IDE::cut() {
    editor->cut();
}

void IDE::copy() {
    editor->copy();
}

void IDE::paste() {
    editor->paste();
}

void IDE::find_replace() {
    editor->show_find_replace();
}

} // namespace ProtlinTJ