#ifndef IMGUI_WRAPPER_H
#define IMGUI_WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

// Forward declaration for GLFW window
struct GLFWwindow;

// Context management
void* imgui_create_context(void);
void imgui_destroy_context(void* ctx);

// Backend initialization/shutdown
int imgui_init_for_glfw(GLFWwindow* window, int install_callbacks);
int imgui_init_for_opengl3(const char* glsl_version);
void imgui_shutdown_opengl3(void);
void imgui_shutdown_glfw(void);

// Frame management
void imgui_new_frame(void);
void imgui_render(void);
void imgui_end_frame(void);

// OpenGL3 backend rendering
void imgui_opengl3_render_draw_data(void);

// IO access
void imgui_io_set_display_size(float width, float height);
int imgui_io_want_capture_mouse(void);
int imgui_io_want_capture_keyboard(void);

// Basic widgets
int imgui_begin(const char* name, int* p_open, int flags);
void imgui_end(void);
void imgui_text(const char* text);
int imgui_button(const char* label);
int imgui_checkbox(const char* label, int* v);
int imgui_slider_float(const char* label, float* v, float v_min, float v_max);
int imgui_slider_int(const char* label, int* v, int v_min, int v_max);
int imgui_input_float(const char* label, float* v);
int imgui_input_int(const char* label, int* v);
int imgui_color_edit3(const char* label, float col[3]);
int imgui_color_edit4(const char* label, float col[4]);
void imgui_same_line(void);
void imgui_separator(void);
void imgui_spacing(void);
void imgui_dummy(float width, float height);
void imgui_indent(float indent_w);
void imgui_unindent(float indent_w);

// Tree nodes
int imgui_tree_node(const char* label);
void imgui_tree_pop(void);

// Combo box
int imgui_begin_combo(const char* label, const char* preview_value, int flags);
void imgui_end_combo(void);
int imgui_selectable(const char* label, int selected, int flags);

// Menu
int imgui_begin_main_menu_bar(void);
void imgui_end_main_menu_bar(void);
int imgui_begin_menu(const char* label, int enabled);
void imgui_end_menu(void);
int imgui_menu_item(const char* label, const char* shortcut, int selected, int enabled);

// Tooltips
void imgui_set_tooltip(const char* text);
int imgui_begin_tooltip(void);
void imgui_end_tooltip(void);

// Popups
int imgui_begin_popup(const char* str_id, int flags);
int imgui_begin_popup_modal(const char* name, int* p_open, int flags);
void imgui_end_popup(void);
void imgui_open_popup(const char* str_id);
void imgui_close_current_popup(void);

// Tables
int imgui_begin_table(const char* str_id, int column, int flags);
void imgui_end_table(void);
void imgui_table_next_row(void);
int imgui_table_next_column(void);
int imgui_table_set_column_index(int column_n);
void imgui_table_setup_column(const char* label, int flags, float init_width_or_weight);
void imgui_table_headers_row(void);

// Columns (legacy)
void imgui_columns(int count, const char* id, int border);
void imgui_next_column(void);

// Style
void imgui_push_style_color(int idx, float r, float g, float b, float a);
void imgui_pop_style_color(int count);
void imgui_push_style_var_float(int idx, float val);
void imgui_push_style_var_vec2(int idx, float x, float y);
void imgui_pop_style_var(int count);

// ID stack
void imgui_push_id_int(int int_id);
void imgui_push_id_str(const char* str_id);
void imgui_pop_id(void);

// Utilities
int imgui_is_item_hovered(void);
int imgui_is_item_clicked(int mouse_button);
int imgui_is_item_active(void);
void imgui_set_next_window_pos(float x, float y, int cond);
void imgui_set_next_window_size(float width, float height, int cond);

// Demo window (useful for testing)
void imgui_show_demo_window(int* p_open);

// DPI scaling (for Windows high-DPI displays)
float imgui_get_dpi_scale(GLFWwindow* window);
void imgui_apply_dpi_scale(GLFWwindow* window);

#ifdef __cplusplus
}
#endif

#endif // IMGUI_WRAPPER_H
