# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

wilhelm_renderer_imgui provides Dear ImGui integration for wilhelm_renderer. It bundles Dear ImGui v1.91.8 with GLFW and OpenGL3 backends, compiled as a static library with C FFI wrappers for Rust.

### Intended Use Case

This crate serves as the **GUI chrome layer** for Air Traffic Management (ATM) radar view applications built on wilhelm_renderer. The layered architecture:

```
┌─────────────────────────────────────────────┐
│  ATM Radar Application                      │
├─────────────────────────────────────────────┤
│  Radar Components    │  UI Chrome           │
│  (tracks, maps,      │  (panels, dialogs,   │
│   range rings, etc.) │   menus, settings)   │
├──────────────────────┴──────────────────────┤
│  wilhelm_renderer    │  wilhelm_renderer_   │
│  (OpenGL, geometry,  │  imgui (this crate)  │
│   shaders, camera)   │                      │
└─────────────────────────────────────────────┘
```

- **wilhelm_renderer**: Core rendering (geometry, shaders, camera, OpenGL context)
- **wilhelm_renderer_imgui**: ImGui glue layer for UI controls and dialogs
- **Radar components** (future): Track rendering, map layers, data blocks, range rings
- **ATM application**: Domain logic, data feeds, interaction handling

ImGui handles control panels, settings dialogs, menus, and overlays. Custom radar-specific rendering (tracks, maps, measurement tools) is built directly on wilhelm_renderer for precise control over the critical display elements.

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

## Keyboard Handling

The GLFW backend handles keyboard input through callback chaining, allowing custom GLFW callbacks to coexist with ImGui.

### How It Works

1. **Callback Installation**: When `ImGui::new(window_ptr, true)` is called, the GLFW backend installs `ImGui_ImplGlfw_KeyCallback` and stores any previously registered callback.

2. **Callback Chaining**: On each key event, ImGui first calls the previous user callback (if any), then processes the key itself via `io.AddKeyEvent()`.

3. **Input Capture Query**: Use `want_capture_keyboard()` to check if ImGui wants keyboard input (e.g., a text field is focused).

### Coexisting with Custom Callbacks

Custom GLFW key callbacks do not interfere with ImGui controls. Both receive all key events.

**Option 1: Set callback before ImGui init**
```rust
// Your callback set first - ImGui will chain to it
glfwSetKeyCallback(window, my_key_callback);
let imgui = ImGui::new(window_ptr, true);
```

**Option 2: Check capture state in your handler**
```rust
// In your input handling:
if !imgui.want_capture_keyboard() {
    // Handle your shortcuts (Ctrl+S, etc.)
}
// Otherwise ImGui handles it (user typing in InputText, etc.)
```

This ensures shortcuts like `Ctrl+S` don't trigger while the user is typing in an ImGui text field.
