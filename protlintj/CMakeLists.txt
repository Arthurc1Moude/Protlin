cmake_minimum_required(VERSION 3.16)
project(ProtlinTJ VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find required packages
find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK4 REQUIRED gtk4)

# Include directories
include_directories(${GTK4_INCLUDE_DIRS})
include_directories(src)

# Source files
set(SOURCES
    src/main.cpp
    src/ide.cpp
    src/editor.cpp
    src/file_manager.cpp
    src/output_panel.cpp
    src/theme_manager.cpp
    src/protlin_highlighter.cpp
)

# Headers
set(HEADERS
    src/ide.h
    src/editor.h
    src/file_manager.h
    src/output_panel.h
    src/theme_manager.h
    src/protlin_highlighter.h
)

# Create executable
add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

# Link libraries
target_link_libraries(${PROJECT_NAME} ${GTK4_LIBRARIES})
target_compile_options(${PROJECT_NAME} PRIVATE ${GTK4_CFLAGS_OTHER})

# Install
install(TARGETS ${PROJECT_NAME} DESTINATION bin)
install(FILES resources/protlintj.desktop DESTINATION share/applications)
install(FILES resources/icon.png DESTINATION share/pixmaps)