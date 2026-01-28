#include "imgui_wrapper.h"
#include "imgui/imgui.h"
#include "imgui/backends/imgui_impl_glfw.h"
#include "imgui/backends/imgui_impl_opengl3.h"

extern "C" {

// Context management
void* imgui_create_context(void) {
    return ImGui::CreateContext();
}

void imgui_destroy_context(void* ctx) {
    ImGui::DestroyContext(static_cast<ImGuiContext*>(ctx));
}

// Backend initialization/shutdown
int imgui_init_for_glfw(GLFWwindow* window, int install_callbacks) {
    return ImGui_ImplGlfw_InitForOpenGL(window, install_callbacks != 0) ? 1 : 0;
}

int imgui_init_for_opengl3(const char* glsl_version) {
    return ImGui_ImplOpenGL3_Init(glsl_version) ? 1 : 0;
}

void imgui_shutdown_opengl3(void) {
    ImGui_ImplOpenGL3_Shutdown();
}

void imgui_shutdown_glfw(void) {
    ImGui_ImplGlfw_Shutdown();
}

// Frame management
void imgui_new_frame(void) {
    ImGui_ImplOpenGL3_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();
}

void imgui_render(void) {
    ImGui::Render();
}

void imgui_end_frame(void) {
    ImGui::EndFrame();
}

// OpenGL3 backend rendering
void imgui_opengl3_render_draw_data(void) {
    ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
}

// IO access
void imgui_io_set_display_size(float width, float height) {
    ImGuiIO& io = ImGui::GetIO();
    io.DisplaySize = ImVec2(width, height);
}

int imgui_io_want_capture_mouse(void) {
    return ImGui::GetIO().WantCaptureMouse ? 1 : 0;
}

int imgui_io_want_capture_keyboard(void) {
    return ImGui::GetIO().WantCaptureKeyboard ? 1 : 0;
}

// Basic widgets
int imgui_begin(const char* name, int* p_open, int flags) {
    bool* open_ptr = nullptr;
    bool open_val;
    if (p_open) {
        open_val = (*p_open != 0);
        open_ptr = &open_val;
    }
    bool result = ImGui::Begin(name, open_ptr, static_cast<ImGuiWindowFlags>(flags));
    if (p_open && open_ptr) {
        *p_open = open_val ? 1 : 0;
    }
    return result ? 1 : 0;
}

void imgui_end(void) {
    ImGui::End();
}

void imgui_text(const char* text) {
    ImGui::TextUnformatted(text);
}

int imgui_button(const char* label) {
    return ImGui::Button(label) ? 1 : 0;
}

int imgui_checkbox(const char* label, int* v) {
    bool val = (*v != 0);
    bool result = ImGui::Checkbox(label, &val);
    *v = val ? 1 : 0;
    return result ? 1 : 0;
}

int imgui_slider_float(const char* label, float* v, float v_min, float v_max) {
    return ImGui::SliderFloat(label, v, v_min, v_max) ? 1 : 0;
}

int imgui_slider_int(const char* label, int* v, int v_min, int v_max) {
    return ImGui::SliderInt(label, v, v_min, v_max) ? 1 : 0;
}

int imgui_input_float(const char* label, float* v) {
    return ImGui::InputFloat(label, v) ? 1 : 0;
}

int imgui_input_int(const char* label, int* v) {
    return ImGui::InputInt(label, v) ? 1 : 0;
}

int imgui_color_edit3(const char* label, float col[3]) {
    return ImGui::ColorEdit3(label, col) ? 1 : 0;
}

int imgui_color_edit4(const char* label, float col[4]) {
    return ImGui::ColorEdit4(label, col) ? 1 : 0;
}

void imgui_same_line(void) {
    ImGui::SameLine();
}

void imgui_separator(void) {
    ImGui::Separator();
}

void imgui_spacing(void) {
    ImGui::Spacing();
}

void imgui_dummy(float width, float height) {
    ImGui::Dummy(ImVec2(width, height));
}

void imgui_indent(float indent_w) {
    ImGui::Indent(indent_w);
}

void imgui_unindent(float indent_w) {
    ImGui::Unindent(indent_w);
}

// Tree nodes
int imgui_tree_node(const char* label) {
    return ImGui::TreeNode(label) ? 1 : 0;
}

void imgui_tree_pop(void) {
    ImGui::TreePop();
}

// Combo box
int imgui_begin_combo(const char* label, const char* preview_value, int flags) {
    return ImGui::BeginCombo(label, preview_value, static_cast<ImGuiComboFlags>(flags)) ? 1 : 0;
}

void imgui_end_combo(void) {
    ImGui::EndCombo();
}

int imgui_selectable(const char* label, int selected, int flags) {
    return ImGui::Selectable(label, selected != 0, static_cast<ImGuiSelectableFlags>(flags)) ? 1 : 0;
}

// Menu
int imgui_begin_main_menu_bar(void) {
    return ImGui::BeginMainMenuBar() ? 1 : 0;
}

void imgui_end_main_menu_bar(void) {
    ImGui::EndMainMenuBar();
}

int imgui_begin_menu(const char* label, int enabled) {
    return ImGui::BeginMenu(label, enabled != 0) ? 1 : 0;
}

void imgui_end_menu(void) {
    ImGui::EndMenu();
}

int imgui_menu_item(const char* label, const char* shortcut, int selected, int enabled) {
    return ImGui::MenuItem(label, shortcut, selected != 0, enabled != 0) ? 1 : 0;
}

// Tooltips
void imgui_set_tooltip(const char* text) {
    ImGui::SetTooltip("%s", text);
}

int imgui_begin_tooltip(void) {
    return ImGui::BeginTooltip() ? 1 : 0;
}

void imgui_end_tooltip(void) {
    ImGui::EndTooltip();
}

// Popups
int imgui_begin_popup(const char* str_id, int flags) {
    return ImGui::BeginPopup(str_id, static_cast<ImGuiWindowFlags>(flags)) ? 1 : 0;
}

int imgui_begin_popup_modal(const char* name, int* p_open, int flags) {
    bool* open_ptr = nullptr;
    bool open_val;
    if (p_open) {
        open_val = (*p_open != 0);
        open_ptr = &open_val;
    }
    bool result = ImGui::BeginPopupModal(name, open_ptr, static_cast<ImGuiWindowFlags>(flags));
    if (p_open && open_ptr) {
        *p_open = open_val ? 1 : 0;
    }
    return result ? 1 : 0;
}

void imgui_end_popup(void) {
    ImGui::EndPopup();
}

void imgui_open_popup(const char* str_id) {
    ImGui::OpenPopup(str_id);
}

void imgui_close_current_popup(void) {
    ImGui::CloseCurrentPopup();
}

// Tables
int imgui_begin_table(const char* str_id, int column, int flags) {
    return ImGui::BeginTable(str_id, column, static_cast<ImGuiTableFlags>(flags)) ? 1 : 0;
}

void imgui_end_table(void) {
    ImGui::EndTable();
}

void imgui_table_next_row(void) {
    ImGui::TableNextRow();
}

int imgui_table_next_column(void) {
    return ImGui::TableNextColumn() ? 1 : 0;
}

int imgui_table_set_column_index(int column_n) {
    return ImGui::TableSetColumnIndex(column_n) ? 1 : 0;
}

void imgui_table_setup_column(const char* label, int flags, float init_width_or_weight) {
    ImGui::TableSetupColumn(label, static_cast<ImGuiTableColumnFlags>(flags), init_width_or_weight);
}

void imgui_table_headers_row(void) {
    ImGui::TableHeadersRow();
}

// Columns (legacy)
void imgui_columns(int count, const char* id, int border) {
    ImGui::Columns(count, id, border != 0);
}

void imgui_next_column(void) {
    ImGui::NextColumn();
}

// Style
void imgui_push_style_color(int idx, float r, float g, float b, float a) {
    ImGui::PushStyleColor(static_cast<ImGuiCol>(idx), ImVec4(r, g, b, a));
}

void imgui_pop_style_color(int count) {
    ImGui::PopStyleColor(count);
}

void imgui_push_style_var_float(int idx, float val) {
    ImGui::PushStyleVar(static_cast<ImGuiStyleVar>(idx), val);
}

void imgui_push_style_var_vec2(int idx, float x, float y) {
    ImGui::PushStyleVar(static_cast<ImGuiStyleVar>(idx), ImVec2(x, y));
}

void imgui_pop_style_var(int count) {
    ImGui::PopStyleVar(count);
}

// ID stack
void imgui_push_id_int(int int_id) {
    ImGui::PushID(int_id);
}

void imgui_push_id_str(const char* str_id) {
    ImGui::PushID(str_id);
}

void imgui_pop_id(void) {
    ImGui::PopID();
}

// Utilities
int imgui_is_item_hovered(void) {
    return ImGui::IsItemHovered() ? 1 : 0;
}

int imgui_is_item_clicked(int mouse_button) {
    return ImGui::IsItemClicked(static_cast<ImGuiMouseButton>(mouse_button)) ? 1 : 0;
}

int imgui_is_item_active(void) {
    return ImGui::IsItemActive() ? 1 : 0;
}

void imgui_set_next_window_pos(float x, float y, int cond) {
    ImGui::SetNextWindowPos(ImVec2(x, y), static_cast<ImGuiCond>(cond));
}

void imgui_set_next_window_size(float width, float height, int cond) {
    ImGui::SetNextWindowSize(ImVec2(width, height), static_cast<ImGuiCond>(cond));
}

// Demo window
void imgui_show_demo_window(int* p_open) {
    bool* open_ptr = nullptr;
    bool open_val;
    if (p_open) {
        open_val = (*p_open != 0);
        open_ptr = &open_val;
    }
    ImGui::ShowDemoWindow(open_ptr);
    if (p_open && open_ptr) {
        *p_open = open_val ? 1 : 0;
    }
}

} // extern "C"
