#pragma once

#include <gtk/gtk.h>
#include <string>
#include <vector>
#include <regex>

namespace ProtlinTJ {

struct HighlightRule {
    std::regex pattern;
    std::string tag_name;
    std::string color;
    bool bold;
    bool italic;
    
    HighlightRule(const std::string& regex_pattern, const std::string& tag,
                 const std::string& color_value, bool is_bold = false, bool is_italic = false)
        : pattern(regex_pattern), tag_name(tag), color(color_value), bold(is_bold), italic(is_italic) {}
};

class ProtlinHighlighter {
public:
    explicit ProtlinHighlighter(GtkTextBuffer* buffer);
    ~ProtlinHighlighter();
    
    void highlight();
    void clear_highlighting();
    
    void set_theme(const std::string& theme);
    
private:
    GtkTextBuffer* text_buffer;
    std::vector<HighlightRule> rules;
    std::vector<GtkTextTag*> tags;
    
    void setup_rules();
    void setup_light_theme_rules();
    void setup_dark_theme_rules();
    void create_tags();
    void apply_highlighting();
    
    GtkTextTag* create_tag(const std::string& name, const std::string& color,
                          bool bold = false, bool italic = false);
};

} // namespace ProtlinTJ