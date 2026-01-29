# wilhelm_renderer_imgui

Dear ImGui integration for [wilhelm_renderer](https://crates.io/crates/wilhelm_renderer).

Bundles Dear ImGui v1.91.8 with GLFW and OpenGL3 backends, compiled as a static library with C FFI wrappers for Rust.

## Installation

```toml
[dependencies]
wilhelm_renderer = "0.2"
wilhelm_renderer_imgui = "0.1"
```

### Build Requirements

- C++ compiler and CMake
- Linux: `libgl1-mesa-dev`
- Dear ImGui is bundled, no external dependency needed

## Usage

```rust
use wilhelm_renderer::core::{App, Color, Window};
use wilhelm_renderer_imgui::ImGui;

fn main() {
    let window = Window::new("Demo", 800, 600, Color::from_rgb(0.1, 0.1, 0.1));
    let mut app = App::new(window);

    let imgui = ImGui::new(app.window.glfw_window_ptr(), true);

    app.on_render(move |_renderer| {
        imgui.new_frame();

        imgui.begin("Window", None, 0);
        if imgui.button("Click") {
            println!("Clicked!");
        }
        imgui.end();

        imgui.render();
    });

    app.run();
}
```

## Available Widgets

- **Windows**: `begin`, `end`, `set_next_window_pos/size`
- **Text/Buttons**: `text`, `button`, `checkbox`
- **Sliders/Input**: `slider_float/int`, `input_float/int`
- **Color**: `color_edit3/4`
- **Layout**: `same_line`, `separator`, `spacing`, `indent`
- **Tree**: `tree_node`, `tree_pop`
- **Combo**: `begin_combo`, `end_combo`, `selectable`
- **Menu**: `begin_main_menu_bar`, `begin_menu`, `menu_item`
- **Tables**: `begin_table`, `table_next_row/column`, `table_setup_column`
- **Popups**: `begin_popup`, `open_popup`, `close_current_popup`
- **Demo**: `show_demo_window`

## Example

Run the interactive demo:

```bash
cargo run --example demo
```

## License

MIT
