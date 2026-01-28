# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

wilhelm_renderer_imgui provides Dear ImGui integration for wilhelm_renderer. It bundles Dear ImGui v1.91.8 with GLFW and OpenGL3 backends, compiled as a static library with C FFI wrappers for Rust.

## Build Commands

```bash
# Build the library
cargo build

# Run the demo example
cargo run --example demo
```

### Build Requirements

- C++ compiler and CMake (cmake crate invokes CMake during build)
- Linux: OpenGL development libraries (`libgl1-mesa-dev`)
- Dear ImGui is bundled, no external dependency needed

## Architecture

### Two-Layer Design

1. **C++ FFI Layer** (`cpp/`)
   - `imgui_wrapper.h/cpp`: C FFI wrapper exposing ImGui functions
   - `imgui/`: Bundled Dear ImGui source (core + GLFW/OpenGL3 backends only)
   - `glfw/include/`: GLFW headers for compilation
   - `CMakeLists.txt`: Builds static `libimgui_wrapper.a`

2. **Rust Safe Wrapper** (`src/lib.rs`)
   - `ffi` module: Raw extern "C" declarations
   - `ImGui` struct: Safe wrapper with idiomatic Rust API
   - Flag modules: `window_flags`, `cond`, `table_flags`, `col`

### Key Patterns

- **Static Compilation**: ImGui is built as a static library, no runtime dependencies
- **C FFI Bridge**: ImGui C++ API wrapped in C functions for Rust FFI
- **RAII Cleanup**: `ImGui` struct implements `Drop` for proper shutdown
- **Thread Safety**: `ImGui` is `!Send` and `!Sync` (raw pointer to context)

### Build System

`build.rs` uses CMake to compile the C++ layer:
- Linux: Links `imgui_wrapper`, `GL`, `stdc++`
- macOS: Links OpenGL, Cocoa, IOKit, CoreVideo frameworks, `c++`
- Windows: Links `opengl32`, `gdi32`, `shell32`

## Usage Pattern

```rust
use wilhelm_renderer::core::{App, Window, Color};
use wilhelm_renderer_imgui::ImGui;

let window = Window::new("Demo", 800, 600, Color::from_rgb(0.1, 0.1, 0.1));
let mut app = App::new(window);

let imgui = ImGui::new(app.window.glfw_window_ptr(), true);

app.on_render(move |_renderer| {
    imgui.new_frame();
    imgui.begin("Window", None, 0);
    if imgui.button("Click") { /* handle click */ }
    imgui.end();
    imgui.render();
});

app.run();
```

## Key Files

- `src/lib.rs`: FFI bindings and safe `ImGui` wrapper (~690 lines)
- `cpp/imgui_wrapper.cpp`: C wrapper implementation
- `cpp/imgui_wrapper.h`: C wrapper declarations
- `cpp/CMakeLists.txt`: CMake build configuration
- `build.rs`: Rust/CMake integration
- `examples/demo.rs`: Working example with shape + ImGui

## Available Widgets

The wrapper exposes common ImGui widgets:
- Windows: `begin`, `end`, `set_next_window_pos/size`
- Text/Buttons: `text`, `button`, `checkbox`
- Sliders/Input: `slider_float/int`, `input_float/int`
- Color: `color_edit3/4`
- Layout: `same_line`, `separator`, `spacing`, `indent`
- Tree: `tree_node`, `tree_pop`
- Combo: `begin_combo`, `end_combo`, `selectable`
- Menu: `begin_main_menu_bar`, `begin_menu`, `menu_item`
- Tables: `begin_table`, `table_next_row/column`, `table_setup_column`
- Popups: `begin_popup`, `open_popup`, `close_current_popup`
- Tooltips: `set_tooltip`, `begin_tooltip`
- Demo: `show_demo_window`

## Platform Notes

- Uses OpenGL 3.3 Core Profile (matching wilhelm_renderer)
- GLSL version `#version 330`
- ImGui callbacks can be auto-installed or manual (`install_callbacks` parameter)
