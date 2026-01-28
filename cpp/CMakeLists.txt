cmake_minimum_required(VERSION 3.16)
project(imgui_wrapper CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# ImGui core sources
set(IMGUI_SOURCES
    imgui/imgui.cpp
    imgui/imgui_demo.cpp
    imgui/imgui_draw.cpp
    imgui/imgui_tables.cpp
    imgui/imgui_widgets.cpp
)

# ImGui backend sources
set(IMGUI_BACKEND_SOURCES
    imgui/backends/imgui_impl_glfw.cpp
    imgui/backends/imgui_impl_opengl3.cpp
)

# Our wrapper
set(WRAPPER_SOURCES
    imgui_wrapper.cpp
)

# Create static library
add_library(imgui_wrapper STATIC
    ${IMGUI_SOURCES}
    ${IMGUI_BACKEND_SOURCES}
    ${WRAPPER_SOURCES}
)

# Include directories
target_include_directories(imgui_wrapper PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/imgui
    ${CMAKE_CURRENT_SOURCE_DIR}/imgui/backends
    ${CMAKE_CURRENT_SOURCE_DIR}/glfw/include
)

# Platform-specific settings
if(UNIX AND NOT APPLE)
    # Linux
    find_package(OpenGL REQUIRED)
    target_link_libraries(imgui_wrapper PRIVATE OpenGL::GL)
elseif(APPLE)
    # macOS
    find_library(COCOA_LIBRARY Cocoa)
    find_library(IOKIT_LIBRARY IOKit)
    find_library(COREVIDEO_LIBRARY CoreVideo)
    find_package(OpenGL REQUIRED)
    target_link_libraries(imgui_wrapper PRIVATE
        OpenGL::GL
        ${COCOA_LIBRARY}
        ${IOKIT_LIBRARY}
        ${COREVIDEO_LIBRARY}
    )
elseif(WIN32)
    # Windows
    target_link_libraries(imgui_wrapper PRIVATE opengl32)
endif()

# Install the library
install(TARGETS imgui_wrapper
    ARCHIVE DESTINATION lib
    LIBRARY DESTINATION lib
)
