//! Dear ImGui integration for wilhelm_renderer
//!
//! This crate provides Dear ImGui support on top of wilhelm_renderer.
//!
//! # Example
//!
//! ```ignore
//! use wilhelm_renderer::core::{App, Window, Color};
//! use wilhelm_renderer_imgui::ImGui;
//!
//! let window = Window::new("ImGui Example", 800, 600, Color::from_rgb(0.1, 0.1, 0.1));
//! let mut app = App::new(window);
//!
//! let imgui = ImGui::new(app.window.glfw_window_ptr());
//!
//! app.on_pre_render(move |_shapes, _renderer| {
//!     imgui.new_frame();
//!     imgui.begin("Debug", None, 0);
//!     imgui.text("Hello, ImGui!");
//!     imgui.end();
//! });
//!
//! app.on_render(move |_renderer| {
//!     imgui.render();
//! });
//!
//! app.run();
//! ```

pub use wilhelm_renderer;
pub use wilhelm_renderer::core::GLFWwindow;

use std::ffi::CString;
use std::ptr;

// FFI declarations for the C wrapper
mod ffi {
    use std::os::raw::{c_char, c_float, c_int, c_void};
    use wilhelm_renderer::core::GLFWwindow;

    unsafe extern "C" {
        // Context management
        pub fn imgui_create_context() -> *mut c_void;
        pub fn imgui_destroy_context(ctx: *mut c_void);

        // Backend initialization/shutdown
        pub fn imgui_init_for_glfw(window: *const GLFWwindow, install_callbacks: c_int) -> c_int;
        pub fn imgui_init_for_opengl3(glsl_version: *const c_char) -> c_int;
        pub fn imgui_shutdown_opengl3();
        pub fn imgui_shutdown_glfw();

        // Frame management
        pub fn imgui_new_frame();
        pub fn imgui_render();
        pub fn imgui_end_frame();

        // OpenGL3 backend rendering
        pub fn imgui_opengl3_render_draw_data();

        // IO access
        pub fn imgui_io_set_display_size(width: c_float, height: c_float);
        pub fn imgui_io_want_capture_mouse() -> c_int;
        pub fn imgui_io_want_capture_keyboard() -> c_int;

        // Basic widgets
        pub fn imgui_begin(name: *const c_char, p_open: *mut c_int, flags: c_int) -> c_int;
        pub fn imgui_end();
        pub fn imgui_text(text: *const c_char);
        pub fn imgui_button(label: *const c_char) -> c_int;
        pub fn imgui_checkbox(label: *const c_char, v: *mut c_int) -> c_int;
        pub fn imgui_slider_float(
            label: *const c_char,
            v: *mut c_float,
            v_min: c_float,
            v_max: c_float,
        ) -> c_int;
        pub fn imgui_slider_int(
            label: *const c_char,
            v: *mut c_int,
            v_min: c_int,
            v_max: c_int,
        ) -> c_int;
        pub fn imgui_input_float(label: *const c_char, v: *mut c_float) -> c_int;
        pub fn imgui_input_int(label: *const c_char, v: *mut c_int) -> c_int;
        pub fn imgui_color_edit3(label: *const c_char, col: *mut c_float) -> c_int;
        pub fn imgui_color_edit4(label: *const c_char, col: *mut c_float) -> c_int;
        pub fn imgui_same_line();
        pub fn imgui_separator();
        pub fn imgui_spacing();
        pub fn imgui_dummy(width: c_float, height: c_float);
        pub fn imgui_indent(indent_w: c_float);
        pub fn imgui_unindent(indent_w: c_float);

        // Tree nodes
        pub fn imgui_tree_node(label: *const c_char) -> c_int;
        pub fn imgui_tree_pop();

        // Combo box
        pub fn imgui_begin_combo(
            label: *const c_char,
            preview_value: *const c_char,
            flags: c_int,
        ) -> c_int;
        pub fn imgui_end_combo();
        pub fn imgui_selectable(label: *const c_char, selected: c_int, flags: c_int) -> c_int;

        // Menu
        pub fn imgui_begin_main_menu_bar() -> c_int;
        pub fn imgui_end_main_menu_bar();
        pub fn imgui_begin_menu(label: *const c_char, enabled: c_int) -> c_int;
        pub fn imgui_end_menu();
        pub fn imgui_menu_item(
            label: *const c_char,
            shortcut: *const c_char,
            selected: c_int,
            enabled: c_int,
        ) -> c_int;

        // Tooltips
        pub fn imgui_set_tooltip(text: *const c_char);
        pub fn imgui_begin_tooltip() -> c_int;
        pub fn imgui_end_tooltip();

        // Popups
        pub fn imgui_begin_popup(str_id: *const c_char, flags: c_int) -> c_int;
        pub fn imgui_begin_popup_modal(
            name: *const c_char,
            p_open: *mut c_int,
            flags: c_int,
        ) -> c_int;
        pub fn imgui_end_popup();
        pub fn imgui_open_popup(str_id: *const c_char);
        pub fn imgui_close_current_popup();

        // Tables
        pub fn imgui_begin_table(str_id: *const c_char, column: c_int, flags: c_int) -> c_int;
        pub fn imgui_end_table();
        pub fn imgui_table_next_row();
        pub fn imgui_table_next_column() -> c_int;
        pub fn imgui_table_set_column_index(column_n: c_int) -> c_int;
        pub fn imgui_table_setup_column(
            label: *const c_char,
            flags: c_int,
            init_width_or_weight: c_float,
        );
        pub fn imgui_table_headers_row();

        // Columns (legacy)
        pub fn imgui_columns(count: c_int, id: *const c_char, border: c_int);
        pub fn imgui_next_column();

        // Style
        pub fn imgui_push_style_color(idx: c_int, r: c_float, g: c_float, b: c_float, a: c_float);
        pub fn imgui_pop_style_color(count: c_int);
        pub fn imgui_push_style_var_float(idx: c_int, val: c_float);
        pub fn imgui_push_style_var_vec2(idx: c_int, x: c_float, y: c_float);
        pub fn imgui_pop_style_var(count: c_int);

        // ID stack
        pub fn imgui_push_id_int(int_id: c_int);
        pub fn imgui_push_id_str(str_id: *const c_char);
        pub fn imgui_pop_id();

        // Utilities
        pub fn imgui_is_item_hovered() -> c_int;
        pub fn imgui_is_item_clicked(mouse_button: c_int) -> c_int;
        pub fn imgui_is_item_active() -> c_int;
        pub fn imgui_set_next_window_pos(x: c_float, y: c_float, cond: c_int);
        pub fn imgui_set_next_window_size(width: c_float, height: c_float, cond: c_int);

        // Demo window
        pub fn imgui_show_demo_window(p_open: *mut c_int);
    }
}

/// Window flags for `begin()`
pub mod window_flags {
    pub const NONE: i32 = 0;
    pub const NO_TITLE_BAR: i32 = 1 << 0;
    pub const NO_RESIZE: i32 = 1 << 1;
    pub const NO_MOVE: i32 = 1 << 2;
    pub const NO_SCROLLBAR: i32 = 1 << 3;
    pub const NO_SCROLL_WITH_MOUSE: i32 = 1 << 4;
    pub const NO_COLLAPSE: i32 = 1 << 5;
    pub const ALWAYS_AUTO_RESIZE: i32 = 1 << 6;
    pub const NO_BACKGROUND: i32 = 1 << 7;
    pub const NO_SAVED_SETTINGS: i32 = 1 << 8;
    pub const NO_MOUSE_INPUTS: i32 = 1 << 9;
    pub const MENU_BAR: i32 = 1 << 10;
    pub const HORIZONTAL_SCROLLBAR: i32 = 1 << 11;
    pub const NO_FOCUS_ON_APPEARING: i32 = 1 << 12;
    pub const NO_BRING_TO_FRONT_ON_FOCUS: i32 = 1 << 13;
    pub const ALWAYS_VERTICAL_SCROLLBAR: i32 = 1 << 14;
    pub const ALWAYS_HORIZONTAL_SCROLLBAR: i32 = 1 << 15;
    pub const NO_NAV_INPUTS: i32 = 1 << 16;
    pub const NO_NAV_FOCUS: i32 = 1 << 17;
    pub const UNSAVED_DOCUMENT: i32 = 1 << 18;
    pub const NO_NAV: i32 = NO_NAV_INPUTS | NO_NAV_FOCUS;
    pub const NO_DECORATION: i32 = NO_TITLE_BAR | NO_RESIZE | NO_SCROLLBAR | NO_COLLAPSE;
    pub const NO_INPUTS: i32 = NO_MOUSE_INPUTS | NO_NAV_INPUTS | NO_NAV_FOCUS;
}

/// Condition flags for `set_next_window_pos()` and `set_next_window_size()`
pub mod cond {
    pub const NONE: i32 = 0;
    pub const ALWAYS: i32 = 1 << 0;
    pub const ONCE: i32 = 1 << 1;
    pub const FIRST_USE_EVER: i32 = 1 << 2;
    pub const APPEARING: i32 = 1 << 3;
}

/// Table flags for `begin_table()`
pub mod table_flags {
    pub const NONE: i32 = 0;
    pub const RESIZABLE: i32 = 1 << 0;
    pub const REORDERABLE: i32 = 1 << 1;
    pub const HIDEABLE: i32 = 1 << 2;
    pub const SORTABLE: i32 = 1 << 3;
    pub const NO_SAVED_SETTINGS: i32 = 1 << 4;
    pub const CONTEXT_MENU_IN_BODY: i32 = 1 << 5;
    pub const ROW_BG: i32 = 1 << 6;
    pub const BORDERS_INNER_H: i32 = 1 << 7;
    pub const BORDERS_OUTER_H: i32 = 1 << 8;
    pub const BORDERS_INNER_V: i32 = 1 << 9;
    pub const BORDERS_OUTER_V: i32 = 1 << 10;
    pub const BORDERS_H: i32 = BORDERS_INNER_H | BORDERS_OUTER_H;
    pub const BORDERS_V: i32 = BORDERS_INNER_V | BORDERS_OUTER_V;
    pub const BORDERS_INNER: i32 = BORDERS_INNER_V | BORDERS_INNER_H;
    pub const BORDERS_OUTER: i32 = BORDERS_OUTER_V | BORDERS_OUTER_H;
    pub const BORDERS: i32 = BORDERS_INNER | BORDERS_OUTER;
}

/// Style color indices for `push_style_color()`
pub mod col {
    pub const TEXT: i32 = 0;
    pub const TEXT_DISABLED: i32 = 1;
    pub const WINDOW_BG: i32 = 2;
    pub const CHILD_BG: i32 = 3;
    pub const POPUP_BG: i32 = 4;
    pub const BORDER: i32 = 5;
    pub const BORDER_SHADOW: i32 = 6;
    pub const FRAME_BG: i32 = 7;
    pub const FRAME_BG_HOVERED: i32 = 8;
    pub const FRAME_BG_ACTIVE: i32 = 9;
    pub const TITLE_BG: i32 = 10;
    pub const TITLE_BG_ACTIVE: i32 = 11;
    pub const TITLE_BG_COLLAPSED: i32 = 12;
    pub const MENU_BAR_BG: i32 = 13;
    pub const SCROLLBAR_BG: i32 = 14;
    pub const SCROLLBAR_GRAB: i32 = 15;
    pub const SCROLLBAR_GRAB_HOVERED: i32 = 16;
    pub const SCROLLBAR_GRAB_ACTIVE: i32 = 17;
    pub const CHECK_MARK: i32 = 18;
    pub const SLIDER_GRAB: i32 = 19;
    pub const SLIDER_GRAB_ACTIVE: i32 = 20;
    pub const BUTTON: i32 = 21;
    pub const BUTTON_HOVERED: i32 = 22;
    pub const BUTTON_ACTIVE: i32 = 23;
    pub const HEADER: i32 = 24;
    pub const HEADER_HOVERED: i32 = 25;
    pub const HEADER_ACTIVE: i32 = 26;
}

/// Dear ImGui context and safe wrapper
pub struct ImGui {
    ctx: *mut std::ffi::c_void,
}

impl ImGui {
    /// Create a new ImGui context and initialize backends for the given GLFW window.
    ///
    /// # Arguments
    /// * `window` - Raw GLFW window pointer from `Window::glfw_window_ptr()`
    /// * `install_callbacks` - If true, ImGui will install its own GLFW callbacks
    ///
    /// # Safety
    /// The window pointer must be valid for the lifetime of the ImGui context.
    pub fn new(window: *const GLFWwindow, install_callbacks: bool) -> Self {
        let ctx = unsafe { ffi::imgui_create_context() };

        let glsl_version = CString::new("#version 330").unwrap();
        unsafe {
            ffi::imgui_init_for_glfw(window, if install_callbacks { 1 } else { 0 });
            ffi::imgui_init_for_opengl3(glsl_version.as_ptr());
        }

        Self { ctx }
    }

    /// Start a new ImGui frame. Call this at the beginning of your render loop.
    pub fn new_frame(&self) {
        unsafe { ffi::imgui_new_frame() };
    }

    /// Finalize and render the ImGui frame. Call this at the end of your render loop.
    pub fn render(&self) {
        unsafe {
            ffi::imgui_render();
            ffi::imgui_opengl3_render_draw_data();
        }
    }

    /// Returns true if ImGui wants to capture mouse input (e.g., mouse is over an ImGui window).
    pub fn want_capture_mouse(&self) -> bool {
        unsafe { ffi::imgui_io_want_capture_mouse() != 0 }
    }

    /// Returns true if ImGui wants to capture keyboard input.
    pub fn want_capture_keyboard(&self) -> bool {
        unsafe { ffi::imgui_io_want_capture_keyboard() != 0 }
    }

    // ---- Windows ----

    /// Begin a new window. Returns true if the window is not collapsed.
    ///
    /// # Arguments
    /// * `name` - Window title/ID
    /// * `open` - Optional mutable bool; if Some, shows a close button
    /// * `flags` - Window flags from `window_flags` module
    pub fn begin(&self, name: &str, open: Option<&mut bool>, flags: i32) -> bool {
        let name_c = CString::new(name).unwrap();
        unsafe {
            match open {
                Some(open_ref) => {
                    let mut open_int = if *open_ref { 1 } else { 0 };
                    let result = ffi::imgui_begin(name_c.as_ptr(), &mut open_int, flags);
                    *open_ref = open_int != 0;
                    result != 0
                }
                None => ffi::imgui_begin(name_c.as_ptr(), ptr::null_mut(), flags) != 0,
            }
        }
    }

    /// End the current window. Must be called after `begin()`.
    pub fn end(&self) {
        unsafe { ffi::imgui_end() };
    }

    /// Set position of the next window.
    pub fn set_next_window_pos(&self, x: f32, y: f32, cond: i32) {
        unsafe { ffi::imgui_set_next_window_pos(x, y, cond) };
    }

    /// Set size of the next window.
    pub fn set_next_window_size(&self, width: f32, height: f32, cond: i32) {
        unsafe { ffi::imgui_set_next_window_size(width, height, cond) };
    }

    // ---- Widgets: Text ----

    /// Display text.
    pub fn text(&self, text: &str) {
        let text_c = CString::new(text).unwrap();
        unsafe { ffi::imgui_text(text_c.as_ptr()) };
    }

    // ---- Widgets: Buttons ----

    /// Button widget. Returns true if clicked.
    pub fn button(&self, label: &str) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_button(label_c.as_ptr()) != 0 }
    }

    /// Checkbox widget. Returns true if value changed.
    pub fn checkbox(&self, label: &str, v: &mut bool) -> bool {
        let label_c = CString::new(label).unwrap();
        let mut v_int = if *v { 1 } else { 0 };
        let changed = unsafe { ffi::imgui_checkbox(label_c.as_ptr(), &mut v_int) != 0 };
        *v = v_int != 0;
        changed
    }

    // ---- Widgets: Sliders ----

    /// Float slider. Returns true if value changed.
    pub fn slider_float(&self, label: &str, v: &mut f32, min: f32, max: f32) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_slider_float(label_c.as_ptr(), v, min, max) != 0 }
    }

    /// Int slider. Returns true if value changed.
    pub fn slider_int(&self, label: &str, v: &mut i32, min: i32, max: i32) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_slider_int(label_c.as_ptr(), v, min, max) != 0 }
    }

    // ---- Widgets: Input ----

    /// Float input. Returns true if value changed.
    pub fn input_float(&self, label: &str, v: &mut f32) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_input_float(label_c.as_ptr(), v) != 0 }
    }

    /// Int input. Returns true if value changed.
    pub fn input_int(&self, label: &str, v: &mut i32) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_input_int(label_c.as_ptr(), v) != 0 }
    }

    // ---- Widgets: Color ----

    /// RGB color editor. Returns true if value changed.
    pub fn color_edit3(&self, label: &str, col: &mut [f32; 3]) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_color_edit3(label_c.as_ptr(), col.as_mut_ptr()) != 0 }
    }

    /// RGBA color editor. Returns true if value changed.
    pub fn color_edit4(&self, label: &str, col: &mut [f32; 4]) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_color_edit4(label_c.as_ptr(), col.as_mut_ptr()) != 0 }
    }

    // ---- Layout ----

    /// Place next widget on the same line as the previous one.
    pub fn same_line(&self) {
        unsafe { ffi::imgui_same_line() };
    }

    /// Add a horizontal separator.
    pub fn separator(&self) {
        unsafe { ffi::imgui_separator() };
    }

    /// Add vertical spacing.
    pub fn spacing(&self) {
        unsafe { ffi::imgui_spacing() };
    }

    /// Add a dummy item of given size.
    pub fn dummy(&self, width: f32, height: f32) {
        unsafe { ffi::imgui_dummy(width, height) };
    }

    /// Indent content.
    pub fn indent(&self, indent_w: f32) {
        unsafe { ffi::imgui_indent(indent_w) };
    }

    /// Unindent content.
    pub fn unindent(&self, indent_w: f32) {
        unsafe { ffi::imgui_unindent(indent_w) };
    }

    // ---- Tree ----

    /// Begin a tree node. Returns true if the node is open.
    pub fn tree_node(&self, label: &str) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_tree_node(label_c.as_ptr()) != 0 }
    }

    /// End a tree node. Call only if `tree_node()` returned true.
    pub fn tree_pop(&self) {
        unsafe { ffi::imgui_tree_pop() };
    }

    // ---- Combo ----

    /// Begin a combo box. Returns true if the combo is open.
    pub fn begin_combo(&self, label: &str, preview: &str, flags: i32) -> bool {
        let label_c = CString::new(label).unwrap();
        let preview_c = CString::new(preview).unwrap();
        unsafe { ffi::imgui_begin_combo(label_c.as_ptr(), preview_c.as_ptr(), flags) != 0 }
    }

    /// End a combo box. Call only if `begin_combo()` returned true.
    pub fn end_combo(&self) {
        unsafe { ffi::imgui_end_combo() };
    }

    /// Selectable item in a combo/list. Returns true if clicked.
    pub fn selectable(&self, label: &str, selected: bool, flags: i32) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_selectable(label_c.as_ptr(), if selected { 1 } else { 0 }, flags) != 0 }
    }

    // ---- Menu ----

    /// Begin the main menu bar.
    pub fn begin_main_menu_bar(&self) -> bool {
        unsafe { ffi::imgui_begin_main_menu_bar() != 0 }
    }

    /// End the main menu bar.
    pub fn end_main_menu_bar(&self) {
        unsafe { ffi::imgui_end_main_menu_bar() };
    }

    /// Begin a menu.
    pub fn begin_menu(&self, label: &str, enabled: bool) -> bool {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_begin_menu(label_c.as_ptr(), if enabled { 1 } else { 0 }) != 0 }
    }

    /// End a menu.
    pub fn end_menu(&self) {
        unsafe { ffi::imgui_end_menu() };
    }

    /// Menu item. Returns true if activated.
    pub fn menu_item(&self, label: &str, shortcut: Option<&str>, selected: bool, enabled: bool) -> bool {
        let label_c = CString::new(label).unwrap();
        let shortcut_c = shortcut.map(|s| CString::new(s).unwrap());
        let shortcut_ptr = shortcut_c.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        unsafe {
            ffi::imgui_menu_item(
                label_c.as_ptr(),
                shortcut_ptr,
                if selected { 1 } else { 0 },
                if enabled { 1 } else { 0 },
            ) != 0
        }
    }

    // ---- Tooltips ----

    /// Set a tooltip for the previous item.
    pub fn set_tooltip(&self, text: &str) {
        let text_c = CString::new(text).unwrap();
        unsafe { ffi::imgui_set_tooltip(text_c.as_ptr()) };
    }

    /// Begin a tooltip.
    pub fn begin_tooltip(&self) -> bool {
        unsafe { ffi::imgui_begin_tooltip() != 0 }
    }

    /// End a tooltip.
    pub fn end_tooltip(&self) {
        unsafe { ffi::imgui_end_tooltip() };
    }

    // ---- Popups ----

    /// Begin a popup.
    pub fn begin_popup(&self, str_id: &str, flags: i32) -> bool {
        let str_id_c = CString::new(str_id).unwrap();
        unsafe { ffi::imgui_begin_popup(str_id_c.as_ptr(), flags) != 0 }
    }

    /// Begin a modal popup.
    pub fn begin_popup_modal(&self, name: &str, open: Option<&mut bool>, flags: i32) -> bool {
        let name_c = CString::new(name).unwrap();
        unsafe {
            match open {
                Some(open_ref) => {
                    let mut open_int = if *open_ref { 1 } else { 0 };
                    let result = ffi::imgui_begin_popup_modal(name_c.as_ptr(), &mut open_int, flags);
                    *open_ref = open_int != 0;
                    result != 0
                }
                None => ffi::imgui_begin_popup_modal(name_c.as_ptr(), ptr::null_mut(), flags) != 0,
            }
        }
    }

    /// End a popup.
    pub fn end_popup(&self) {
        unsafe { ffi::imgui_end_popup() };
    }

    /// Open a popup.
    pub fn open_popup(&self, str_id: &str) {
        let str_id_c = CString::new(str_id).unwrap();
        unsafe { ffi::imgui_open_popup(str_id_c.as_ptr()) };
    }

    /// Close the current popup.
    pub fn close_current_popup(&self) {
        unsafe { ffi::imgui_close_current_popup() };
    }

    // ---- Tables ----

    /// Begin a table.
    pub fn begin_table(&self, str_id: &str, columns: i32, flags: i32) -> bool {
        let str_id_c = CString::new(str_id).unwrap();
        unsafe { ffi::imgui_begin_table(str_id_c.as_ptr(), columns, flags) != 0 }
    }

    /// End a table.
    pub fn end_table(&self) {
        unsafe { ffi::imgui_end_table() };
    }

    /// Move to the next row in a table.
    pub fn table_next_row(&self) {
        unsafe { ffi::imgui_table_next_row() };
    }

    /// Move to the next column in a table.
    pub fn table_next_column(&self) -> bool {
        unsafe { ffi::imgui_table_next_column() != 0 }
    }

    /// Set the current column index.
    pub fn table_set_column_index(&self, column: i32) -> bool {
        unsafe { ffi::imgui_table_set_column_index(column) != 0 }
    }

    /// Setup a column (call before first row).
    pub fn table_setup_column(&self, label: &str, flags: i32, init_width: f32) {
        let label_c = CString::new(label).unwrap();
        unsafe { ffi::imgui_table_setup_column(label_c.as_ptr(), flags, init_width) };
    }

    /// Display column headers row.
    pub fn table_headers_row(&self) {
        unsafe { ffi::imgui_table_headers_row() };
    }

    // ---- Style ----

    /// Push a style color.
    pub fn push_style_color(&self, idx: i32, r: f32, g: f32, b: f32, a: f32) {
        unsafe { ffi::imgui_push_style_color(idx, r, g, b, a) };
    }

    /// Pop style colors.
    pub fn pop_style_color(&self, count: i32) {
        unsafe { ffi::imgui_pop_style_color(count) };
    }

    // ---- ID Stack ----

    /// Push an integer ID.
    pub fn push_id_int(&self, id: i32) {
        unsafe { ffi::imgui_push_id_int(id) };
    }

    /// Push a string ID.
    pub fn push_id(&self, id: &str) {
        let id_c = CString::new(id).unwrap();
        unsafe { ffi::imgui_push_id_str(id_c.as_ptr()) };
    }

    /// Pop an ID.
    pub fn pop_id(&self) {
        unsafe { ffi::imgui_pop_id() };
    }

    // ---- Item Queries ----

    /// Returns true if the last item is hovered.
    pub fn is_item_hovered(&self) -> bool {
        unsafe { ffi::imgui_is_item_hovered() != 0 }
    }

    /// Returns true if the last item was clicked.
    pub fn is_item_clicked(&self, mouse_button: i32) -> bool {
        unsafe { ffi::imgui_is_item_clicked(mouse_button) != 0 }
    }

    /// Returns true if the last item is active.
    pub fn is_item_active(&self) -> bool {
        unsafe { ffi::imgui_is_item_active() != 0 }
    }

    // ---- Demo ----

    /// Show the ImGui demo window.
    pub fn show_demo_window(&self, open: Option<&mut bool>) {
        unsafe {
            match open {
                Some(open_ref) => {
                    let mut open_int = if *open_ref { 1 } else { 0 };
                    ffi::imgui_show_demo_window(&mut open_int);
                    *open_ref = open_int != 0;
                }
                None => ffi::imgui_show_demo_window(ptr::null_mut()),
            }
        }
    }
}

impl Drop for ImGui {
    fn drop(&mut self) {
        unsafe {
            ffi::imgui_shutdown_opengl3();
            ffi::imgui_shutdown_glfw();
            ffi::imgui_destroy_context(self.ctx);
        }
    }
}

// Note: ImGui is !Send and !Sync because it contains a raw pointer (*mut c_void)
