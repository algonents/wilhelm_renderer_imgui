//! Simple ImGui Demo
//!
//! Demonstrates Dear ImGui integration with wilhelm_renderer.
//! Shows how to use ImGui controls to dynamically modify shape properties.

use std::cell::RefCell;
use std::rc::Rc;

use wilhelm_renderer::core::{App, Color, Window};
use wilhelm_renderer::graphics2d::shapes::{ShapeKind, ShapeRenderable, ShapeStyle, Triangle};
use wilhelm_renderer_imgui::ImGui;

fn main() {
    let window = Window::new("Wilhelm Renderer Imgui", 800, 600, Color::from_rgb(0.1, 0.1, 0.15));
    let mut app = App::new(window);

    // Add a triangle - use pixel coordinates!
    // Center of 800x600 window is (400, 300)
    // Triangle vertices are relative to the shape position
    app.add_shape(ShapeRenderable::from_shape(
        400.0,
        300.0,
        ShapeKind::Triangle(Triangle::new([
            (-100.0, 50.0),  // bottom left (relative to center)
            (100.0, 50.0),   // bottom right
            (0.0, -100.0),   // top
        ])),
        ShapeStyle::fill(Color::from_rgb(0.2, 0.6, 0.9)),
    ));

    // Initialize ImGui
    let imgui = ImGui::new(app.window.glfw_window_ptr(), true);

    // Shared state (modified by ImGui, read by pre_render)
    let pos_x = Rc::new(RefCell::new(400.0f32));
    let pos_y = Rc::new(RefCell::new(300.0f32));
    let scale = Rc::new(RefCell::new(1.0f32));

    // Clone Rc handles for pre_render callback
    let pos_x_update = Rc::clone(&pos_x);
    let pos_y_update = Rc::clone(&pos_y);
    let scale_update = Rc::clone(&scale);

    // Update shape properties before rendering
    app.on_pre_render(move |shapes, _renderer| {
        if let Some(shape) = shapes.first_mut() {
            shape.set_position(*pos_x_update.borrow(), *pos_y_update.borrow());
            shape.set_scale(*scale_update.borrow());
        }
    });

    // ImGui controls
    app.on_render(move |_renderer| {
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
