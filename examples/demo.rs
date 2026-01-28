//! Simple ImGui Demo
//!
//! Demonstrates Dear ImGui integration with wilhelm_renderer.

use wilhelm_renderer::core::{App, Color, Window};
use wilhelm_renderer::graphics2d::shapes::{ShapeKind, ShapeRenderable, ShapeStyle, Triangle};
use wilhelm_renderer_imgui::ImGui;

fn main() {
    let window = Window::new("ImGui Demo", 800, 600, Color::from_rgb(0.1, 0.1, 0.15));
    let mut app = App::new(window);

    // Add a triangle - use pixel coordinates!
    // Center of 800x600 window is (400, 300)
    // Triangle vertices are relative to the shape position
    app.add_shape(ShapeRenderable::from_shape(
        400.0,  // x position (center)
        300.0,  // y position (center)
        ShapeKind::Triangle(Triangle::new([
            (-100.0, 50.0),   // bottom left (relative to center)
            (100.0, 50.0),    // bottom right
            (0.0, -100.0),    // top
        ])),
        ShapeStyle::fill(Color::from_rgb(0.2, 0.6, 0.9)),
    ));

    // Initialize ImGui
    let imgui = ImGui::new(app.window.glfw_window_ptr(), true);

    let mut counter = 0;

    app.on_render(move |_renderer| {
        imgui.new_frame();

        imgui.begin("Debug", None, 0);
        if imgui.button("Click me!") {
            counter += 1;
            println!("Button clicked! Count: {}", counter);
        }
        imgui.text(&format!("Count: {}", counter));
        imgui.end();

        imgui.render();
    });

    app.run();
}
