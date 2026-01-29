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
use std::cell::RefCell;
use std::rc::Rc;

use wilhelm_renderer::core::{App, Color, Window};
use wilhelm_renderer::graphics2d::shapes::{ShapeKind, ShapeRenderable, ShapeStyle, Triangle};
use wilhelm_renderer_imgui::ImGui;

fn main() {
    let window = Window::new("Demo", 800, 600, Color::from_rgb(0.1, 0.1, 0.15));
    let mut app = App::new(window);

    // Add a triangle
    app.add_shape(ShapeRenderable::from_shape(
        400.0, 300.0,
        ShapeKind::Triangle(Triangle::new([
            (-100.0, 50.0), (100.0, 50.0), (0.0, -100.0),
        ])),
        ShapeStyle::fill(Color::from_rgb(0.2, 0.6, 0.9)),
    ));

    // Initialize ImGui
    let imgui = ImGui::new(app.window.glfw_window_ptr(), true);

    // Shared state (modified by ImGui, read by pre_render)
    let pos_x = Rc::new(RefCell::new(400.0f32));
    let pos_y = Rc::new(RefCell::new(300.0f32));
    let scale = Rc::new(RefCell::new(1.0f32));

    let (pos_x_update, pos_y_update, scale_update) =
        (Rc::clone(&pos_x), Rc::clone(&pos_y), Rc::clone(&scale));

    // Update shape properties before rendering
    app.on_pre_render(move |shapes, _| {
        if let Some(shape) = shapes.first_mut() {
            shape.set_position(*pos_x_update.borrow(), *pos_y_update.borrow());
            shape.set_scale(*scale_update.borrow());
        }
    });

    // ImGui controls
    app.on_render(move |_| {
        imgui.new_frame();

        imgui.begin("Shape Controls", None, 0);
        imgui.text("Position");
        imgui.slider_float("X", &mut pos_x.borrow_mut(), 0.0, 800.0);
        imgui.slider_float("Y", &mut pos_y.borrow_mut(), 0.0, 600.0);
        imgui.separator();
        imgui.text("Transform");
        imgui.slider_float("Scale", &mut scale.borrow_mut(), 0.1, 3.0);
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
