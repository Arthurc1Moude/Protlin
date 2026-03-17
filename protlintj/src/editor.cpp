#include "editor.h"
#include "ide.h"
#include "protlin_highlighter.h"
#include <fstream>
#include <sstream>

namespace ProtlinTJ {

Editor::Editor(IDE* ide) : ide(ide), find_dialog(nullptr) {
    // Create scrolled window
    scrolled_window = gtk_scrolled_window_new();
    gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(scrolled_window),
                                  GTK_POLICY_AUTOMATIC, GTK_POLICY_AUTOMATIC);
    
    setup_text_view();
    setup_find_dialog();
    
    // Initialize highlighter
    highlighter = std::make_unique<ProtlinHighlighter>(text_buffer);
}

Editor::~Editor() {
    // GTK widgets are automatically destroyed
}

void Editor::setup_text_view() {
    // Create text buffer
    text_buffer = gtk_text_buffer_new(nullptr);
    
    // Create text view
    text_view = gtk_text_view_new_with_buffer(text_buffer);
    gtk_text_view_set_monospace(GTK_TEXT_VIEW(text_view), TRUE);
    gtk_text_view_set_show_line_numbers(GTK_TEXT_VIEW(text_view), TRUE);
    gtk_text_view_set_highlight_current_line(GTK_TEXT_VIEW(text_view), TRUE);
    gtk_text_view_set_auto_indent(GTK_TEXT_VIEW(text_view), TRUE);
    gtk_text_view_set_indent_width(GTK_TEXT_VIEW(text_view), 4);
    gtk_text_view_set_tab_width(GTK_TEXT_VIEW(text_view), 4);
    gtk_text_view_set_insert_spaces_instead_of_tabs(GTK_TEXT_VIEW(text_view), TRUE);
    
    // Add to scrolled window
    gtk_scrolled_window_set_child(GTK_SCROLLED_WINDOW(scrolled_window), text_view);
    
    // Connect signals
    g_signal_connect(text_buffer, "changed", G_CALLBACK(on_text_changed), this);
    g_signal_connect(text_buffer, "notify::cursor-position", G_CALLBACK(on_cursor_moved), this);
    
    // Set default content
    gtk_text_buffer_set_text(text_buffer, 
        "// Welcome to ProtlinTJ - The Protlin IDE\n"
        "// Start coding your Protlin programs here\n\n"
        "function main() {\n"
        "    println(\"Hello, Protlin!\")\n"
        "}\n\n"
        "main()\n", -1);
}

void Editor::setup_find_dialog() {
    // Create find/replace dialog
    find_dialog = gtk_window_new();
    gtk_window_set_title(GTK_WINDOW(find_dialog), "Find & Replace");
    gtk_window_set_default_size(GTK_WINDOW(find_dialog), 400, 150);
    gtk_window_set_resizable(GTK_WINDOW(find_dialog), FALSE);
    gtk_window_set_modal(GTK_WINDOW(find_dialog), TRUE);
    
    // Create grid layout
    GtkWidget* grid = gtk_grid_new();
    gtk_grid_set_row_spacing(GTK_GRID(grid), 6);
    gtk_grid_set_column_spacing(GTK_GRID(grid), 6);
    gtk_widget_set_margin_start(grid, 12);
    gtk_widget_set_margin_end(grid, 12);
    gtk_widget_set_margin_top(grid, 12);
    gtk_widget_set_margin_bottom(grid, 12);
    gtk_window_set_child(GTK_WINDOW(find_dialog), grid);
    
    // Find label and entry
    GtkWidget* find_label = gtk_label_new("Find:");
    gtk_grid_attach(GTK_GRID(grid), find_label, 0, 0, 1, 1);
    
    find_entry = gtk_entry_new();
    gtk_widget_set_hexpand(find_entry, TRUE);
    gtk_grid_attach(GTK_GRID(grid), find_entry, 1, 0, 2, 1);
    
    // Replace label and entry
    GtkWidget* replace_label = gtk_label_new("Replace:");
    gtk_grid_attach(GTK_GRID(grid), replace_label, 0, 1, 1, 1);
    
    replace_entry = gtk_entry_new();
    gtk_widget_set_hexpand(replace_entry, TRUE);
    gtk_grid_attach(GTK_GRID(grid), replace_entry, 1, 1, 2, 1);
    
    // Buttons
    GtkWidget* find_next_btn = gtk_button_new_with_label("Find Next");
    g_signal_connect(find_next_btn, "clicked", G_CALLBACK(on_find_next_clicked), this);
    gtk_grid_attach(GTK_GRID(grid), find_next_btn, 0, 2, 1, 1);
    
    GtkWidget* find_prev_btn = gtk_button_new_with_label("Find Previous");
    g_signal_connect(find_prev_btn, "clicked", G_CALLBACK(on_find_previous_clicked), this);
    gtk_grid_attach(GTK_GRID(grid), find_prev_btn, 1, 2, 1, 1);
    
    GtkWidget* replace_btn = gtk_button_new_with_label("Replace");
    g_signal_connect(replace_btn, "clicked", G_CALLBACK(on_replace_clicked), this);
    gtk_grid_attach(GTK_GRID(grid), replace_btn, 2, 2, 1, 1);
    
    GtkWidget* replace_all_btn = gtk_button_new_with_label("Replace All");
    g_signal_connect(replace_all_btn, "clicked", G_CALLBACK(on_replace_all_clicked), this);
    gtk_grid_attach(GTK_GRID(grid), replace_all_btn, 0, 3, 1, 1);
    
    GtkWidget* close_btn = gtk_button_new_with_label("Close");
    g_signal_connect(close_btn, "clicked", G_CALLBACK(on_find_dialog_close), this);
    gtk_grid_attach(GTK_GRID(grid), close_btn, 2, 3, 1, 1);
}

void Editor::new_file() {
    gtk_text_buffer_set_text(text_buffer, "", -1);
}

bool Editor::load_file(const std::string& filename) {
    std::ifstream file(filename);
    if (!file.is_open()) {
        return false;
    }
    
    std::stringstream buffer;
    buffer << file.rdbuf();
    std::string content = buffer.str();
    
    gtk_text_buffer_set_text(text_buffer, content.c_str(), -1);
    
    return true;
}

bool Editor::save_file(const std::string& filename) {
    std::string content = get_text();
    
    std::ofstream file(filename);
    if (!file.is_open()) {
        return false;
    }
    
    file << content;
    return true;
}

void Editor::cut() {
    GtkClipboard* clipboard = gtk_clipboard_get(GDK_SELECTION_CLIPBOARD);
    gtk_text_buffer_cut_clipboard(text_buffer, clipboard, TRUE);
}

void Editor::copy() {
    GtkClipboard* clipboard = gtk_clipboard_get(GDK_SELECTION_CLIPBOARD);
    gtk_text_buffer_copy_clipboard(text_buffer, clipboard);
}

void Editor::paste() {
    GtkClipboard* clipboard = gtk_clipboard_get(GDK_SELECTION_CLIPBOARD);
    gtk_text_buffer_paste_clipboard(text_buffer, clipboard, nullptr, TRUE);
}

void Editor::undo() {
    // TODO: Implement undo functionality
}

void Editor::redo() {
    // TODO: Implement redo functionality
}

void Editor::select_all() {
    GtkTextIter start, end;
    gtk_text_buffer_get_bounds(text_buffer, &start, &end);
    gtk_text_buffer_select_range(text_buffer, &start, &end);
}

void Editor::show_find_replace() {
    gtk_window_set_transient_for(GTK_WINDOW(find_dialog), 
                                GTK_WINDOW(ide->get_window()));
    gtk_window_present(GTK_WINDOW(find_dialog));
    gtk_widget_grab_focus(find_entry);
}

void Editor::find_next(const std::string& text) {
    if (text.empty()) return;
    
    GtkTextIter start, match_start, match_end;
    gtk_text_buffer_get_iter_at_mark(text_buffer, &start,
                                    gtk_text_buffer_get_insert(text_buffer));
    
    if (gtk_text_iter_forward_search(&start, text.c_str(),
                                    GTK_TEXT_SEARCH_TEXT_ONLY,
                                    &match_start, &match_end, nullptr)) {
        gtk_text_buffer_select_range(text_buffer, &match_start, &match_end);
        gtk_text_view_scroll_to_iter(GTK_TEXT_VIEW(text_view), &match_start,
                                    0.0, FALSE, 0.0, 0.0);
    }
}

void Editor::find_previous(const std::string& text) {
    if (text.empty()) return;
    
    GtkTextIter start, match_start, match_end;
    gtk_text_buffer_get_iter_at_mark(text_buffer, &start,
                                    gtk_text_buffer_get_insert(text_buffer));
    
    if (gtk_text_iter_backward_search(&start, text.c_str(),
                                     GTK_TEXT_SEARCH_TEXT_ONLY,
                                     &match_start, &match_end, nullptr)) {
        gtk_text_buffer_select_range(text_buffer, &match_start, &match_end);
        gtk_text_view_scroll_to_iter(GTK_TEXT_VIEW(text_view), &match_start,
                                    0.0, FALSE, 0.0, 0.0);
    }
}

void Editor::replace_current(const std::string& find_text, const std::string& replace_text) {
    GtkTextIter start, end;
    if (gtk_text_buffer_get_selection_bounds(text_buffer, &start, &end)) {
        gchar* selected_text = gtk_text_buffer_get_text(text_buffer, &start, &end, FALSE);
        if (find_text == selected_text) {
            gtk_text_buffer_delete(text_buffer, &start, &end);
            gtk_text_buffer_insert(text_buffer, &start, replace_text.c_str(), -1);
        }
        g_free(selected_text);
    }
}

void Editor::replace_all(const std::string& find_text, const std::string& replace_text) {
    if (find_text.empty()) return;
    
    GtkTextIter start, end;
    gtk_text_buffer_get_bounds(text_buffer, &start, &end);
    
    gchar* text = gtk_text_buffer_get_text(text_buffer, &start, &end, FALSE);
    std::string content(text);
    g_free(text);
    
    size_t pos = 0;
    int count = 0;
    while ((pos = content.find(find_text, pos)) != std::string::npos) {
        content.replace(pos, find_text.length(), replace_text);
        pos += replace_text.length();
        count++;
    }
    
    if (count > 0) {
        gtk_text_buffer_set_text(text_buffer, content.c_str(), -1);
    }
}

std::string Editor::get_text() const {
    GtkTextIter start, end;
    gtk_text_buffer_get_bounds(text_buffer, &start, &end);
    gchar* text = gtk_text_buffer_get_text(text_buffer, &start, &end, FALSE);
    std::string result(text);
    g_free(text);
    return result;
}

void Editor::set_text(const std::string& text) {
    gtk_text_buffer_set_text(text_buffer, text.c_str(), -1);
}

void Editor::insert_text(const std::string& text) {
    GtkTextIter iter;
    gtk_text_buffer_get_iter_at_mark(text_buffer, &iter,
                                    gtk_text_buffer_get_insert(text_buffer));
    gtk_text_buffer_insert(text_buffer, &iter, text.c_str(), -1);
}

void Editor::goto_line(int line) {
    GtkTextIter iter;
    gtk_text_buffer_get_iter_at_line(text_buffer, &iter, line - 1);
    gtk_text_buffer_place_cursor(text_buffer, &iter);
    gtk_text_view_scroll_to_iter(GTK_TEXT_VIEW(text_view), &iter,
                                0.0, FALSE, 0.0, 0.0);
}

int Editor::get_current_line() const {
    GtkTextIter iter;
    gtk_text_buffer_get_iter_at_mark(text_buffer, &iter,
                                    gtk_text_buffer_get_insert(text_buffer));
    return gtk_text_iter_get_line(&iter) + 1;
}

int Editor::get_current_column() const {
    GtkTextIter iter;
    gtk_text_buffer_get_iter_at_mark(text_buffer, &iter,
                                    gtk_text_buffer_get_insert(text_buffer));
    return gtk_text_iter_get_line_offset(&iter) + 1;
}

void Editor::set_font(const std::string& font_name) {
    PangoFontDescription* font_desc = pango_font_description_from_string(font_name.c_str());
    gtk_widget_override_font(text_view, font_desc);
    pango_font_description_free(font_desc);
}

void Editor::set_tab_width(int width) {
    gtk_text_view_set_tab_width(GTK_TEXT_VIEW(text_view), width);
}

void Editor::set_show_line_numbers(bool show) {
    gtk_text_view_set_show_line_numbers(GTK_TEXT_VIEW(text_view), show);
}

void Editor::set_highlight_current_line(bool highlight) {
    gtk_text_view_set_highlight_current_line(GTK_TEXT_VIEW(text_view), highlight);
}

void Editor::set_word_wrap(bool wrap) {
    gtk_text_view_set_wrap_mode(GTK_TEXT_VIEW(text_view), 
                               wrap ? GTK_WRAP_WORD : GTK_WRAP_NONE);
}

// Static callbacks
void Editor::on_text_changed(GtkTextBuffer* buffer, gpointer data) {
    Editor* editor = static_cast<Editor*>(data);
    editor->apply_syntax_highlighting();
}

void Editor::on_cursor_moved(GtkTextBuffer* buffer, GParamSpec* pspec, gpointer data) {
    Editor* editor = static_cast<Editor*>(data);
    // Update status bar with cursor position
    int line = editor->get_current_line();
    int column = editor->get_current_column();
    // TODO: Update status bar
}

void Editor::on_find_next_clicked(GtkWidget* widget, gpointer data) {
    Editor* editor = static_cast<Editor*>(data);
    const gchar* text = gtk_editable_get_text(GTK_EDITABLE(editor->find_entry));
    editor->find_next(text);
}

void Editor::on_find_previous_clicked(GtkWidget* widget, gpointer data) {
    Editor* editor = static_cast<Editor*>(data);
    const gchar* text = gtk_editable_get_text(GTK_EDITABLE(editor->find_entry));
    editor->find_previous(text);
}

void Editor::on_replace_clicked(GtkWidget* widget, gpointer data) {
    Editor* editor = static_cast<Editor*>(data);
    const gchar* find_text = gtk_editable_get_text(GTK_EDITABLE(editor->find_entry));
    const gchar* replace_text = gtk_editable_get_text(GTK_EDITABLE(editor->replace_entry));
    editor->replace_current(find_text, replace_text);
}

void Editor::on_replace_all_clicked(GtkWidget* widget, gpointer data) {
    Editor* editor = static_cast<Editor*>(data);
    const gchar* find_text = gtk_editable_get_text(GTK_EDITABLE(editor->find_entry));
    const gchar* replace_text = gtk_editable_get_text(GTK_EDITABLE(editor->replace_entry));
    editor->replace_all(find_text, replace_text);
}

void Editor::on_find_dialog_close(GtkWidget* widget, gpointer data) {
    Editor* editor = static_cast<Editor*>(data);
    gtk_widget_hide(editor->find_dialog);
}

void Editor::apply_syntax_highlighting() {
    if (highlighter) {
        highlighter->highlight();
    }
}

} // namespace ProtlinTJ