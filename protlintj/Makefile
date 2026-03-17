# ProtlinTJ Makefile

CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -O2
INCLUDES = -Isrc
LIBS = `pkg-config --libs gtk4`
CFLAGS = `pkg-config --cflags gtk4`

SRCDIR = src
OBJDIR = obj
BINDIR = bin

SOURCES = $(wildcard $(SRCDIR)/*.cpp)
OBJECTS = $(SOURCES:$(SRCDIR)/%.cpp=$(OBJDIR)/%.o)
TARGET = $(BINDIR)/protlintj

.PHONY: all clean install uninstall

all: $(TARGET)

$(TARGET): $(OBJECTS) | $(BINDIR)
	$(CXX) $(OBJECTS) -o $@ $(LIBS)

$(OBJDIR)/%.o: $(SRCDIR)/%.cpp | $(OBJDIR)
	$(CXX) $(CXXFLAGS) $(CFLAGS) $(INCLUDES) -c $< -o $@

$(OBJDIR):
	mkdir -p $(OBJDIR)

$(BINDIR):
	mkdir -p $(BINDIR)

clean:
	rm -rf $(OBJDIR) $(BINDIR)

install: $(TARGET)
	install -D $(TARGET) $(DESTDIR)/usr/bin/protlintj
	install -D resources/protlintj.desktop $(DESTDIR)/usr/share/applications/protlintj.desktop
	install -D resources/icon.png $(DESTDIR)/usr/share/pixmaps/protlintj.png

uninstall:
	rm -f $(DESTDIR)/usr/bin/protlintj
	rm -f $(DESTDIR)/usr/share/applications/protlintj.desktop
	rm -f $(DESTDIR)/usr/share/pixmaps/protlintj.png

# Development targets
debug: CXXFLAGS += -g -DDEBUG
debug: $(TARGET)

run: $(TARGET)
	./$(TARGET)

# Dependencies
$(OBJDIR)/main.o: $(SRCDIR)/main.cpp $(SRCDIR)/ide.h
$(OBJDIR)/ide.o: $(SRCDIR)/ide.cpp $(SRCDIR)/ide.h $(SRCDIR)/editor.h $(SRCDIR)/file_manager.h $(SRCDIR)/output_panel.h $(SRCDIR)/theme_manager.h
$(OBJDIR)/editor.o: $(SRCDIR)/editor.cpp $(SRCDIR)/editor.h $(SRCDIR)/ide.h $(SRCDIR)/protlin_highlighter.h
$(OBJDIR)/file_manager.o: $(SRCDIR)/file_manager.cpp $(SRCDIR)/file_manager.h $(SRCDIR)/ide.h
$(OBJDIR)/output_panel.o: $(SRCDIR)/output_panel.cpp $(SRCDIR)/output_panel.h $(SRCDIR)/ide.h
$(OBJDIR)/theme_manager.o: $(SRCDIR)/theme_manager.cpp $(SRCDIR)/theme_manager.h
$(OBJDIR)/protlin_highlighter.o: $(SRCDIR)/protlin_highlighter.cpp $(SRCDIR)/protlin_highlighter.h