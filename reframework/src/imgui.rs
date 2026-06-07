use std::ffi::{CStr, CString};
use imgui_sys::*;

// ── Macros ────────────────────────────────────────────────────────────────────

macro_rules! safe_cstring {
    ($s:expr => $body:expr) => {
        match CString::new($s) {
            Ok(s) => unsafe { $body(s.as_ptr()) },
            Err(_) => log::warn!("imgui: string contained null byte"),
        }
    };
    ($s:expr => $body:expr, $ret:expr) => {
        match CString::new($s) {
            Ok(s) => unsafe { $body(s.as_ptr()) },
            Err(_) => {
                log::warn!("imgui: string contained null byte");
                $ret
            },
        }
    };
}

macro_rules! wrap_void {
    ($($rust_name:ident => $ig_name:ident ($($arg:ident: $ty:ty),*);)*) => {
        $(
            pub fn $rust_name($($arg: $ty),*) {
                unsafe { $ig_name($($arg),*) }
            }
        )*
    };
}

macro_rules! wrap_bool {
    ($($rust_name:ident => $ig_name:ident ($($arg:ident: $ty:ty),*);)*) => {
        $(
            pub fn $rust_name($($arg: $ty),*) -> bool {
                unsafe { $ig_name($($arg),*) }
            }
        )*
    };
}

macro_rules! wrap_f32 {
    ($($rust_name:ident => $ig_name:ident ($($arg:ident: $ty:ty),*);)*) => {
        $(
            pub fn $rust_name($($arg: $ty),*) -> f32 {
                unsafe { $ig_name($($arg),*) }
            }
        )*
    };
}

macro_rules! wrap_f64 {
    ($($rust_name:ident => $ig_name:ident ($($arg:ident: $ty:ty),*);)*) => {
        $(
            pub fn $rust_name($($arg: $ty),*) -> f64 {
                unsafe { $ig_name($($arg),*) }
            }
        )*
    };
}

macro_rules! wrap_out_vec2 {
    ($($rust_name:ident => $ig_name:ident ($($arg:ident: $ty:ty),*);)*) => {
        $(
            pub fn $rust_name($($arg: $ty),*) -> ImVec2 {
                let mut out = ImVec2 { x: 0.0, y: 0.0 };
                unsafe { $ig_name(&mut out $(,$arg)*) }
                out
            }
        )*
    };
}

// ── Context ───────────────────────────────────────────────────────────────────

pub fn set_current_context(ctx: *mut ImGuiContext) {
    unsafe { igSetCurrentContext(ctx) }
}

pub fn get_current_context() -> *mut ImGuiContext {
    unsafe { igGetCurrentContext() }
}

pub fn get_io() -> *mut ImGuiIO {
    unsafe { igGetIO_Nil() }
}

pub fn get_style() -> *mut ImGuiStyle {
    unsafe { igGetStyle() }
}

pub fn set_allocator_functions(
    alloc_func: ImGuiMemAllocFunc,
    free_func: ImGuiMemFreeFunc,
    user_data: *mut ::std::os::raw::c_void,
) {
    unsafe { igSetAllocatorFunctions(alloc_func, free_func, user_data) }
}

pub fn get_version() -> Option<&'static str> {
    unsafe { CStr::from_ptr(igGetVersion()).to_str().ok() }
}

// ── Windows ───────────────────────────────────────────────────────────────────

pub fn begin(name: &CStr, p_open: Option<&mut bool>, flags: ImGuiWindowFlags) -> bool {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    unsafe { igBegin(name.as_ptr(), p, flags) }
}

pub fn begin_str(name: &str, p_open: Option<&mut bool>, flags: ImGuiWindowFlags) -> bool {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    safe_cstring!(name => |ptr| igBegin(ptr, p, flags), false)
}

pub fn end() {
    unsafe { igEnd() }
}

pub fn begin_child(str_id: &CStr, size: ImVec2, child_flags: ImGuiChildFlags, flags: ImGuiWindowFlags) -> bool {
    unsafe { igBeginChild_Str(str_id.as_ptr(), size, child_flags, flags) }
}

pub fn begin_child_str(str_id: &str, size: ImVec2, child_flags: ImGuiChildFlags, flags: ImGuiWindowFlags) -> bool {
    safe_cstring!(str_id => |ptr| igBeginChild_Str(ptr, size, child_flags, flags), false)
}

pub fn begin_child_id(id: ImGuiID, size: ImVec2, child_flags: ImGuiChildFlags, flags: ImGuiWindowFlags) -> bool {
    unsafe { igBeginChild_ID(id, size, child_flags, flags) }
}

wrap_void! {
    end_child              => igEndChild();
    set_next_window_pos    => igSetNextWindowPos(pos: ImVec2, cond: ImGuiCond, pivot: ImVec2);
    set_next_window_size   => igSetNextWindowSize(size: ImVec2, cond: ImGuiCond);
    set_next_window_content_size => igSetNextWindowContentSize(size: ImVec2);
    set_next_window_collapsed    => igSetNextWindowCollapsed(collapsed: bool, cond: ImGuiCond);
    set_next_window_focus        => igSetNextWindowFocus();
    set_next_window_scroll       => igSetNextWindowScroll(scroll: ImVec2);
    set_next_window_bg_alpha     => igSetNextWindowBgAlpha(alpha: f32);
    set_window_pos               => igSetWindowPos_Vec2(pos: ImVec2, cond: ImGuiCond);
    set_window_size              => igSetWindowSize_Vec2(size: ImVec2, cond: ImGuiCond);
    set_window_collapsed         => igSetWindowCollapsed_Bool(collapsed: bool, cond: ImGuiCond);
    set_window_focus             => igSetWindowFocus_Nil();
}

pub fn set_window_pos_named(name: &CStr, pos: ImVec2, cond: ImGuiCond) {
    unsafe { igSetWindowPos_Str(name.as_ptr(), pos, cond) }
}

pub fn set_window_pos_named_str(name: &str, pos: ImVec2, cond: ImGuiCond) {
    safe_cstring!(name => |ptr| igSetWindowPos_Str(ptr, pos, cond));
}

pub fn set_window_size_named(name: &CStr, size: ImVec2, cond: ImGuiCond) {
    unsafe { igSetWindowSize_Str(name.as_ptr(), size, cond) }
}

pub fn set_window_size_named_str(name: &str, size: ImVec2, cond: ImGuiCond) {
    safe_cstring!(name => |ptr| igSetWindowSize_Str(ptr, size, cond));
}

pub fn set_window_collapsed_named(name: &CStr, collapsed: bool, cond: ImGuiCond) {
    unsafe { igSetWindowCollapsed_Str(name.as_ptr(), collapsed, cond) }
}

pub fn set_window_collapsed_named_str(name: &str, collapsed: bool, cond: ImGuiCond) {
    safe_cstring!(name => |ptr| igSetWindowCollapsed_Str(ptr, collapsed, cond));
}

pub fn set_window_focus_named(name: &CStr) {
    unsafe { igSetWindowFocus_Str(name.as_ptr()) }
}

pub fn set_window_focus_named_str(name: &str) {
    safe_cstring!(name => |ptr| igSetWindowFocus_Str(ptr));
}

wrap_bool! {
    is_window_appearing  => igIsWindowAppearing();
    is_window_collapsed  => igIsWindowCollapsed();
    is_window_focused    => igIsWindowFocused(flags: ImGuiFocusedFlags);
    is_window_hovered    => igIsWindowHovered(flags: ImGuiHoveredFlags);
}

pub fn get_window_draw_list() -> *mut ImDrawList {
    unsafe { igGetWindowDrawList() }
}

wrap_f32! {
    get_window_width  => igGetWindowWidth();
    get_window_height => igGetWindowHeight();
    get_scroll_x      => igGetScrollX();
    get_scroll_y      => igGetScrollY();
    get_scroll_max_x  => igGetScrollMaxX();
    get_scroll_max_y  => igGetScrollMaxY();
}

wrap_void! {
    set_scroll_x          => igSetScrollX_Float(scroll_x: f32);
    set_scroll_y          => igSetScrollY_Float(scroll_y: f32);
    set_scroll_here_x     => igSetScrollHereX(center_x_ratio: f32);
    set_scroll_here_y     => igSetScrollHereY(center_y_ratio: f32);
    set_scroll_from_pos_x => igSetScrollFromPosX_Float(local_x: f32, center_x_ratio: f32);
    set_scroll_from_pos_y => igSetScrollFromPosY_Float(local_y: f32, center_y_ratio: f32);
}

wrap_out_vec2! {
    get_window_pos       => igGetWindowPos();
    get_window_size      => igGetWindowSize();
    get_cursor_pos       => igGetCursorPos();
    get_cursor_screen_pos => igGetCursorScreenPos();
    get_cursor_start_pos => igGetCursorStartPos();
    get_content_region_avail => igGetContentRegionAvail();
    get_item_rect_min    => igGetItemRectMin();
    get_item_rect_max    => igGetItemRectMax();
    get_item_rect_size   => igGetItemRectSize();
    get_mouse_pos        => igGetMousePos();
    get_mouse_pos_on_opening_current_popup => igGetMousePosOnOpeningCurrentPopup();
}

wrap_f32! {
    get_cursor_pos_x => igGetCursorPosX();
    get_cursor_pos_y => igGetCursorPosY();
}

wrap_void! {
    set_cursor_pos   => igSetCursorPos(local_pos: ImVec2);
    set_cursor_pos_x => igSetCursorPosX(local_x: f32);
    set_cursor_pos_y => igSetCursorPosY(local_y: f32);
    set_cursor_screen_pos => igSetCursorScreenPos(pos: ImVec2);
}

// ── Font/Style ────────────────────────────────────────────────────────────────

wrap_void! {
    push_font        => igPushFont(font: *mut ImFont, font_size: f32);
    pop_font         => igPopFont();
    push_item_width  => igPushItemWidth(item_width: f32);
    pop_item_width   => igPopItemWidth();
    set_next_item_width => igSetNextItemWidth(item_width: f32);
    push_text_wrap_pos  => igPushTextWrapPos(wrap_local_pos_x: f32);
    pop_text_wrap_pos   => igPopTextWrapPos();
    push_item_flag   => igPushItemFlag(option: ImGuiItemFlags, enabled: bool);
    pop_item_flag    => igPopItemFlag();
    push_style_color_u32 => igPushStyleColor_U32(idx: ImGuiCol, col: ImU32);
    push_style_color     => igPushStyleColor_Vec4(idx: ImGuiCol, col: ImVec4);
    pop_style_color      => igPopStyleColor(count: i32);
    push_style_var_float => igPushStyleVar_Float(idx: ImGuiStyleVar, val: f32);
    push_style_var_vec2  => igPushStyleVar_Vec2(idx: ImGuiStyleVar, val: ImVec2);
    push_style_var_x     => igPushStyleVarX(idx: ImGuiStyleVar, val_x: f32);
    push_style_var_y     => igPushStyleVarY(idx: ImGuiStyleVar, val_y: f32);
    pop_style_var        => igPopStyleVar(count: i32);
    set_color_edit_options => igSetColorEditOptions(flags: ImGuiColorEditFlags);
}

wrap_f32! {
    calc_item_width               => igCalcItemWidth();
    get_text_line_height          => igGetTextLineHeight();
    get_text_line_height_spacing  => igGetTextLineHeightWithSpacing();
    get_frame_height              => igGetFrameHeight();
    get_frame_height_spacing      => igGetFrameHeightWithSpacing();
    get_tree_node_to_label_spacing => igGetTreeNodeToLabelSpacing();
    get_font_size                 => igGetFontSize();
}
wrap_f64! {
    get_time => igGetTime();
}

pub fn get_font() -> *mut ImFont {
    unsafe { igGetFont() }
}

pub fn get_color_u32_col(idx: ImGuiCol, alpha_mul: f32) -> ImU32 {
    unsafe { igGetColorU32_Col(idx, alpha_mul) }
}

pub fn get_color_u32_vec4(col: ImVec4) -> ImU32 {
    unsafe { igGetColorU32_Vec4(col) }
}

pub fn get_color_u32_u32(col: ImU32, alpha_mul: f32) -> ImU32 {
    unsafe { igGetColorU32_U32(col, alpha_mul) }
}

pub fn get_style_color_vec4(idx: ImGuiCol) -> *const ImVec4 {
    unsafe { igGetStyleColorVec4(idx) }
}

pub fn get_style_color_name(idx: ImGuiCol) -> Option<&'static str> {
    unsafe { CStr::from_ptr(igGetStyleColorName(idx)).to_str().ok() }
}

pub fn style_colors_dark() {
    unsafe { igStyleColorsDark(std::ptr::null_mut()) }
}

pub fn style_colors_light() {
    unsafe { igStyleColorsLight(std::ptr::null_mut()) }
}

pub fn style_colors_classic() {
    unsafe { igStyleColorsClassic(std::ptr::null_mut()) }
}

// ── ID Stack ──────────────────────────────────────────────────────────────────

pub fn push_id(str_id: &CStr) {
    unsafe { igPushID_Str(str_id.as_ptr()) }
}

pub fn push_id_str(str_id: &str) {
    safe_cstring!(str_id => |ptr| igPushID_Str(ptr));
}

pub fn push_id_int(int_id: i32) {
    unsafe { igPushID_Int(int_id) }
}

pub fn push_id_ptr(ptr_id: *const ::std::os::raw::c_void) {
    unsafe { igPushID_Ptr(ptr_id) };
}

wrap_void! { pop_id => igPopID(); }

pub fn get_id(str_id: &CStr) -> ImGuiID {
    unsafe { igGetID_Str(str_id.as_ptr()) }
}

pub fn get_id_str(str_id: &str) -> ImGuiID {
    safe_cstring!(str_id => |ptr| igGetID_Str(ptr), 0)
}

pub fn get_id_int(int_id: i32) -> ImGuiID {
    unsafe { igGetID_Int(int_id) }
}

pub fn get_id_ptr(ptr_id: *const ::std::os::raw::c_void) -> ImGuiID {
    unsafe { igGetID_Ptr(ptr_id) }
}

// ── Text ──────────────────────────────────────────────────────────────────────

pub fn text(s: &CStr) {
    unsafe { igText(s.as_ptr()) };
}

pub fn text_str(s: &str) {
    safe_cstring!(s => |ptr: *const i8| igText(c"%s".as_ptr(), ptr));
}

pub unsafe fn text_unformatted(s: &CStr) {
    let start = s.as_ptr();
    let end = unsafe { start.add(s.to_bytes().len()) };
    unsafe { igTextUnformatted(start, end) }
}

pub unsafe fn text_unformatted_str(s: &str) {
    let start = s.as_ptr() as *const i8;
    let end = unsafe { start.add(s.len()) };
    unsafe { igTextUnformatted(start, end) }
}

pub fn text_coloured(col: ImVec4, s: &CStr) {
    unsafe { igTextColored(col, c"%s".as_ptr(), s.as_ptr()) }
}

pub fn text_coloured_str(col: ImVec4, s: &str) {
    safe_cstring!(s => |ptr: *const i8| igTextColored(col, c"%s".as_ptr(), ptr));
}

pub fn text_disabled(s: &CStr) {
    unsafe { igTextDisabled(c"%s".as_ptr(), s.as_ptr()) }
}

pub fn text_disabled_str(s: &str) {
    safe_cstring!(s => |ptr: *const i8| igTextDisabled(c"%s".as_ptr(), ptr));
}

pub fn text_wrapped(s: &CStr) {
    unsafe { igTextWrapped(s.as_ptr()) }
}

pub fn text_wrapped_str(s: &str) {
    safe_cstring!(s => |ptr: *const i8| igTextWrapped(c"%s".as_ptr(), ptr));
}

pub fn label_text(label: &CStr, text: &CStr) {
    unsafe { igLabelText(label.as_ptr(), text.as_ptr()) }
}

pub fn label_text_str(label: &str, text: &str) {
    let label = CString::new(label).unwrap_or_default();
    let text = CString::new(text).unwrap_or_default();
    unsafe { igLabelText(label.as_ptr(), c"%s".as_ptr(), text.as_ptr()) }
}

pub fn bullet_text(s: &CStr) {
    unsafe { igBulletText(s.as_ptr()) }
}

pub fn bullet_text_str(s: &str) {
    safe_cstring!(s => |ptr: *const i8| igBulletText(c"%s".as_ptr(), ptr));
}

pub fn separator_text(label: &CStr) {
    unsafe { igSeparatorText(label.as_ptr()) }
}

pub fn separator_text_str(label: &str) {
    safe_cstring!(label => |ptr| igSeparatorText(ptr));
}

pub fn text_link(label: &CStr) -> bool {
    unsafe { igTextLink(label.as_ptr()) }
}

pub fn text_link_str(label: &str) -> bool {
    safe_cstring!(label => |ptr| igTextLink(ptr), false)
}

pub fn text_link_open_url(label: &CStr, url: &CStr) -> bool {
    unsafe { igTextLinkOpenURL(label.as_ptr(), url.as_ptr()) }
}

pub fn text_link_open_url_str(label: &str, url: &str) -> bool {
    let label = CString::new(label).unwrap_or_default();
    let url = CString::new(url).unwrap_or_default();
    unsafe { igTextLinkOpenURL(label.as_ptr(), url.as_ptr()) }
}

pub fn calc_text_size(text: &CStr, hide_after_double_hash: bool, wrap_width: f32) -> ImVec2 {
    let mut out = ImVec2 { x: 0.0, y: 0.0 };
    let start = text.as_ptr();
    let end = unsafe { start.add(text.to_bytes().len()) };
    unsafe { igCalcTextSize(&mut out, start, end, hide_after_double_hash, wrap_width) }
    out
}

pub fn calc_text_size_str(text: &str, hide_after_double_hash: bool, wrap_width: f32) -> ImVec2 {
    let mut out = ImVec2 { x: 0.0, y: 0.0 };
    let start = text.as_ptr() as *const i8;
    let end = unsafe { start.add(text.len()) };
    unsafe { igCalcTextSize(&mut out, start, end, hide_after_double_hash, wrap_width) }
    out
}

// ── Buttons ───────────────────────────────────────────────────────────────────

pub fn button(label: &CStr) -> bool {
    unsafe { igButton(label.as_ptr(), ImVec2 { x: 0.0, y: 0.0 }) }
}

pub fn button_str(label: &str) -> bool {
    safe_cstring!(label => |ptr| igButton(ptr, ImVec2 { x: 0.0, y: 0.0 }), false)
}

pub fn button_sized(label: &CStr, size: ImVec2) -> bool {
    unsafe { igButton(label.as_ptr(), size) }
}

pub fn button_sized_str(label: &str, size: ImVec2) -> bool {
    safe_cstring!(label => |ptr| igButton(ptr, size), false)
}

pub fn small_button(label: &CStr) -> bool {
    unsafe { igSmallButton(label.as_ptr()) }
}

pub fn small_button_str(label: &str) -> bool {
    safe_cstring!(label => |ptr| igSmallButton(ptr), false)
}

pub fn invisible_button(str_id: &CStr, size: ImVec2, flags: ImGuiButtonFlags) -> bool {
    unsafe { igInvisibleButton(str_id.as_ptr(), size, flags) }
}

pub fn invisible_button_str(str_id: &str, size: ImVec2, flags: ImGuiButtonFlags) -> bool {
    safe_cstring!(str_id => |ptr| igInvisibleButton(ptr, size, flags), false)
}

pub fn arrow_button(str_id: &CStr, dir: ImGuiDir) -> bool {
    unsafe { igArrowButton(str_id.as_ptr(), dir) }
}

pub fn arrow_button_str(str_id: &str, dir: ImGuiDir) -> bool {
    safe_cstring!(str_id => |ptr| igArrowButton(ptr, dir), false)
}

pub fn checkbox(label: &CStr, v: &mut bool) -> bool {
    unsafe { igCheckbox(label.as_ptr(), v as *mut bool) }
}

pub fn checkbox_str(label: &str, v: &mut bool) -> bool {
    safe_cstring!(label => |ptr| igCheckbox(ptr, v as *mut bool), false)
}

pub fn checkbox_flags_int(label: &CStr, flags: &mut i32, flags_value: i32) -> bool {
    unsafe { igCheckboxFlags_IntPtr(label.as_ptr(), flags as *mut i32, flags_value) }
}

pub fn checkbox_flags_int_str(label: &str, flags: &mut i32, flags_value: i32) -> bool {
    safe_cstring!(label => |ptr| igCheckboxFlags_IntPtr(ptr, flags as *mut i32, flags_value), false)
}

pub fn checkbox_flags_uint(label: &CStr, flags: &mut u32, flags_value: u32) -> bool {
    unsafe { igCheckboxFlags_UintPtr(label.as_ptr(), flags as *mut u32, flags_value) }
}

pub fn checkbox_flags_uint_str(label: &str, flags: &mut u32, flags_value: u32) -> bool {
    safe_cstring!(label => |ptr| igCheckboxFlags_UintPtr(ptr, flags as *mut u32, flags_value), false)
}

pub fn radio_button(label: &CStr, active: bool) -> bool {
    unsafe { igRadioButton_Bool(label.as_ptr(), active) }
}

pub fn radio_button_str(label: &str, active: bool) -> bool {
    safe_cstring!(label => |ptr| igRadioButton_Bool(ptr, active), false)
}

pub fn radio_button_int(label: &CStr, v: &mut i32, v_button: i32) -> bool {
    unsafe { igRadioButton_IntPtr(label.as_ptr(), v as *mut i32, v_button) }
}

pub fn radio_button_int_str(label: &str, v: &mut i32, v_button: i32) -> bool {
    safe_cstring!(label => |ptr| igRadioButton_IntPtr(ptr, v as *mut i32, v_button), false)
}

pub fn progress_bar(fraction: f32, size: ImVec2, overlay: Option<&CStr>) {
    let overlay_ptr = overlay.map_or(std::ptr::null(), |s| s.as_ptr());
    unsafe { igProgressBar(fraction, size, overlay_ptr) }
}

pub fn progress_bar_str(fraction: f32, size: ImVec2, overlay: Option<&str>) {
    let overlay_cstr = overlay.and_then(|s| CString::new(s).ok());
    let overlay_ptr = overlay_cstr.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());
    unsafe { igProgressBar(fraction, size, overlay_ptr) }
}

wrap_void! { bullet => igBullet(); }

pub fn selectable_bool(label: &CStr, selected: bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
    unsafe { igSelectable_Bool(label.as_ptr(), selected, flags, size) }
}

pub fn selectable_bool_str(label: &str, selected: bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
    safe_cstring!(label => |ptr| igSelectable_Bool(ptr, selected, flags, size), false)
}

pub fn selectable_bool_ptr(label: &CStr, p_selected: &mut bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
    unsafe { igSelectable_BoolPtr(label.as_ptr(), p_selected as *mut bool, flags, size) }
}

pub fn selectable_bool_ptr_str(label: &str, p_selected: &mut bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
    safe_cstring!(label => |ptr| igSelectable_BoolPtr(ptr, p_selected as *mut bool, flags, size), false)
}

// ── Input ─────────────────────────────────────────────────────────────────────

// Safety: Make sure to check that buf ends with a null byte
pub unsafe fn input_text(label: &CStr, buf: &mut [u8], flags: ImGuiInputTextFlags) -> bool {
    unsafe {
        igInputText(label.as_ptr(), buf.as_mut_ptr() as *mut i8, buf.len(), flags, None, std::ptr::null_mut())
    }
}

pub unsafe fn input_text_str(label: &str, buf: &mut [u8], flags: ImGuiInputTextFlags) -> bool {
    let label = CString::new(label).unwrap_or_default();
    unsafe {
        igInputText(label.as_ptr(), buf.as_mut_ptr() as *mut i8, buf.len(), flags, None, std::ptr::null_mut())
    }
}

pub fn input_text_with_hint(label: &CStr, hint: &CStr, buf: &mut [u8], flags: ImGuiInputTextFlags) -> bool {
    unsafe {
        igInputTextWithHint(label.as_ptr(), hint.as_ptr(), buf.as_mut_ptr() as *mut i8, buf.len(), flags, None, std::ptr::null_mut())
    }
}

pub fn input_text_with_hint_str(label: &str, hint: &str, buf: &mut [u8], flags: ImGuiInputTextFlags) -> bool {
    let label = CString::new(label).unwrap_or_default();
    let hint = CString::new(hint).unwrap_or_default();
    unsafe {
        igInputTextWithHint(label.as_ptr(), hint.as_ptr(), buf.as_mut_ptr() as *mut i8, buf.len(), flags, None, std::ptr::null_mut())
    }
}

pub fn input_text_multiline(label: &CStr, buf: &mut [u8], size: ImVec2, flags: ImGuiInputTextFlags) -> bool {
    unsafe {
        igInputTextMultiline(label.as_ptr(), buf.as_mut_ptr() as *mut i8, buf.len(), size, flags, None, std::ptr::null_mut())
    }
}

pub fn input_text_multiline_str(label: &str, buf: &mut [u8], size: ImVec2, flags: ImGuiInputTextFlags) -> bool {
    let label = CString::new(label).unwrap_or_default();
    unsafe {
        igInputTextMultiline(label.as_ptr(), buf.as_mut_ptr() as *mut i8, buf.len(), size, flags, None, std::ptr::null_mut())
    }
}

pub fn input_float(label: &CStr, v: &mut f32, step: f32, step_fast: f32, flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputFloat(label.as_ptr(), v as *mut f32, step, step_fast, c"%.3f".as_ptr(), flags) }
}

pub fn input_float_str(label: &str, v: &mut f32, step: f32, step_fast: f32, flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputFloat(ptr, v as *mut f32, step, step_fast, c"%.3f".as_ptr(), flags), false)
}

pub fn input_float2(label: &CStr, v: &mut [f32; 2], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputFloat2(label.as_ptr(), v.as_mut_ptr(), c"%.3f".as_ptr(), flags) }
}

pub fn input_float2_str(label: &str, v: &mut [f32; 2], flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputFloat2(ptr, v.as_mut_ptr(), c"%.3f".as_ptr(), flags), false)
}

pub fn input_float3(label: &CStr, v: &mut [f32; 3], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputFloat3(label.as_ptr(), v.as_mut_ptr(), c"%.3f".as_ptr(), flags) }
}

pub fn input_float3_str(label: &str, v: &mut [f32; 3], flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputFloat3(ptr, v.as_mut_ptr(), c"%.3f".as_ptr(), flags), false)
}

pub fn input_float4(label: &CStr, v: &mut [f32; 4], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputFloat4(label.as_ptr(), v.as_mut_ptr(), c"%.3f".as_ptr(), flags) }
}

pub fn input_float4_str(label: &str, v: &mut [f32; 4], flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputFloat4(ptr, v.as_mut_ptr(), c"%.3f".as_ptr(), flags), false)
}

pub fn input_int(label: &CStr, v: &mut i32, step: i32, step_fast: i32, flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputInt(label.as_ptr(), v as *mut i32, step, step_fast, flags) }
}

pub fn input_int_str(label: &str, v: &mut i32, step: i32, step_fast: i32, flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputInt(ptr, v as *mut i32, step, step_fast, flags), false)
}

pub fn input_int2(label: &CStr, v: &mut [i32; 2], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputInt2(label.as_ptr(), v.as_mut_ptr(), flags) }
}

pub fn input_int2_str(label: &str, v: &mut [i32; 2], flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputInt2(ptr, v.as_mut_ptr(), flags), false)
}

pub fn input_int3(label: &CStr, v: &mut [i32; 3], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputInt3(label.as_ptr(), v.as_mut_ptr(), flags) }
}

pub fn input_int3_str(label: &str, v: &mut [i32; 3], flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputInt3(ptr, v.as_mut_ptr(), flags), false)
}

pub fn input_int4(label: &CStr, v: &mut [i32; 4], flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputInt4(label.as_ptr(), v.as_mut_ptr(), flags) }
}

pub fn input_int4_str(label: &str, v: &mut [i32; 4], flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputInt4(ptr, v.as_mut_ptr(), flags), false)
}

pub fn input_double(label: &CStr, v: &mut f64, step: f64, step_fast: f64, flags: ImGuiInputTextFlags) -> bool {
    unsafe { igInputDouble(label.as_ptr(), v as *mut f64, step, step_fast, c"%.6f".as_ptr(), flags) }
}

pub fn input_double_str(label: &str, v: &mut f64, step: f64, step_fast: f64, flags: ImGuiInputTextFlags) -> bool {
    safe_cstring!(label => |ptr| igInputDouble(ptr, v as *mut f64, step, step_fast, c"%.6f".as_ptr(), flags), false)
}

// ── Sliders ───────────────────────────────────────────────────────────────────

pub fn slider_float(label: &CStr, v: &mut f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderFloat(label.as_ptr(), v as *mut f32, min, max, c"%.3f".as_ptr(), flags) }
}

pub fn slider_float_str(label: &str, v: &mut f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderFloat(ptr, v as *mut f32, min, max, c"%.3f".as_ptr(), flags), false)
}

pub fn slider_float2(label: &CStr, v: &mut [f32; 2], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderFloat2(label.as_ptr(), v.as_mut_ptr(), min, max, c"%.3f".as_ptr(), flags) }
}

pub fn slider_float2_str(label: &str, v: &mut [f32; 2], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderFloat2(ptr, v.as_mut_ptr(), min, max, c"%.3f".as_ptr(), flags), false)
}

pub fn slider_float3(label: &CStr, v: &mut [f32; 3], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderFloat3(label.as_ptr(), v.as_mut_ptr(), min, max, c"%.3f".as_ptr(), flags) }
}

pub fn slider_float3_str(label: &str, v: &mut [f32; 3], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderFloat3(ptr, v.as_mut_ptr(), min, max, c"%.3f".as_ptr(), flags), false)
}

pub fn slider_float4(label: &CStr, v: &mut [f32; 4], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderFloat4(label.as_ptr(), v.as_mut_ptr(), min, max, c"%.3f".as_ptr(), flags) }
}

pub fn slider_float4_str(label: &str, v: &mut [f32; 4], min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderFloat4(ptr, v.as_mut_ptr(), min, max, c"%.3f".as_ptr(), flags), false)
}

pub fn slider_angle(label: &CStr, v_rad: &mut f32, min_deg: f32, max_deg: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderAngle(label.as_ptr(), v_rad as *mut f32, min_deg, max_deg, c"%.0f deg".as_ptr(), flags) }
}

pub fn slider_angle_str(label: &str, v_rad: &mut f32, min_deg: f32, max_deg: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderAngle(ptr, v_rad as *mut f32, min_deg, max_deg, c"%.0f deg".as_ptr(), flags), false)
}

pub fn slider_int(label: &CStr, v: &mut i32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderInt(label.as_ptr(), v as *mut i32, min, max, c"%d".as_ptr(), flags) }
}

pub fn slider_int_str(label: &str, v: &mut i32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderInt(ptr, v as *mut i32, min, max, c"%d".as_ptr(), flags), false)
}

pub fn slider_int2(label: &CStr, v: &mut [i32; 2], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderInt2(label.as_ptr(), v.as_mut_ptr(), min, max, c"%d".as_ptr(), flags) }
}

pub fn slider_int2_str(label: &str, v: &mut [i32; 2], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderInt2(ptr, v.as_mut_ptr(), min, max, c"%d".as_ptr(), flags), false)
}

pub fn slider_int3(label: &CStr, v: &mut [i32; 3], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderInt3(label.as_ptr(), v.as_mut_ptr(), min, max, c"%d".as_ptr(), flags) }
}

pub fn slider_int3_str(label: &str, v: &mut [i32; 3], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderInt3(ptr, v.as_mut_ptr(), min, max, c"%d".as_ptr(), flags), false)
}

pub fn slider_int4(label: &CStr, v: &mut [i32; 4], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igSliderInt4(label.as_ptr(), v.as_mut_ptr(), min, max, c"%d".as_ptr(), flags) }
}

pub fn slider_int4_str(label: &str, v: &mut [i32; 4], min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igSliderInt4(ptr, v.as_mut_ptr(), min, max, c"%d".as_ptr(), flags), false)
}

// ── Drag ─────────────────────────────────────────────────────────────────────

pub fn drag_float(label: &CStr, v: &mut f32, speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloat(label.as_ptr(), v as *mut f32, speed, min, max, c"%.3f".as_ptr(), flags) }
}

pub fn drag_float_str(label: &str, v: &mut f32, speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragFloat(ptr, v as *mut f32, speed, min, max, c"%.3f".as_ptr(), flags), false)
}

pub fn drag_float2(label: &CStr, v: &mut [f32; 2], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloat2(label.as_ptr(), v.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), flags) }
}

pub fn drag_float2_str(label: &str, v: &mut [f32; 2], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragFloat2(ptr, v.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), flags), false)
}

pub fn drag_float3(label: &CStr, v: &mut [f32; 3], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloat3(label.as_ptr(), v.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), flags) }
}

pub fn drag_float3_str(label: &str, v: &mut [f32; 3], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragFloat3(ptr, v.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), flags), false)
}

pub fn drag_float4(label: &CStr, v: &mut [f32; 4], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloat4(label.as_ptr(), v.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), flags) }
}

pub fn drag_float4_str(label: &str, v: &mut [f32; 4], speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragFloat4(ptr, v.as_mut_ptr(), speed, min, max, c"%.3f".as_ptr(), flags), false)
}

pub fn drag_float_range2(label: &CStr, v_min: &mut f32, v_max: &mut f32, speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragFloatRange2(label.as_ptr(), v_min as *mut f32, v_max as *mut f32, speed, min, max, c"%.3f".as_ptr(), std::ptr::null(), flags) }
}

pub fn drag_float_range2_str(label: &str, v_min: &mut f32, v_max: &mut f32, speed: f32, min: f32, max: f32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragFloatRange2(ptr, v_min as *mut f32, v_max as *mut f32, speed, min, max, c"%.3f".as_ptr(), std::ptr::null(), flags), false)
}

pub fn drag_int(label: &CStr, v: &mut i32, speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragInt(label.as_ptr(), v as *mut i32, speed, min, max, c"%d".as_ptr(), flags) }
}

pub fn drag_int_str(label: &str, v: &mut i32, speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragInt(ptr, v as *mut i32, speed, min, max, c"%d".as_ptr(), flags), false)
}

pub fn drag_int2(label: &CStr, v: &mut [i32; 2], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragInt2(label.as_ptr(), v.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), flags) }
}

pub fn drag_int2_str(label: &str, v: &mut [i32; 2], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragInt2(ptr, v.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), flags), false)
}

pub fn drag_int3(label: &CStr, v: &mut [i32; 3], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragInt3(label.as_ptr(), v.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), flags) }
}

pub fn drag_int3_str(label: &str, v: &mut [i32; 3], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragInt3(ptr, v.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), flags), false)
}

pub fn drag_int4(label: &CStr, v: &mut [i32; 4], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragInt4(label.as_ptr(), v.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), flags) }
}

pub fn drag_int4_str(label: &str, v: &mut [i32; 4], speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragInt4(ptr, v.as_mut_ptr(), speed, min, max, c"%d".as_ptr(), flags), false)
}

pub fn drag_int_range2(label: &CStr, v_min: &mut i32, v_max: &mut i32, speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    unsafe { igDragIntRange2(label.as_ptr(), v_min as *mut i32, v_max as *mut i32, speed, min, max, c"%d".as_ptr(), std::ptr::null(), flags) }
}

pub fn drag_int_range2_str(label: &str, v_min: &mut i32, v_max: &mut i32, speed: f32, min: i32, max: i32, flags: ImGuiSliderFlags) -> bool {
    safe_cstring!(label => |ptr| igDragIntRange2(ptr, v_min as *mut i32, v_max as *mut i32, speed, min, max, c"%d".as_ptr(), std::ptr::null(), flags), false)
}

// ── Colors ────────────────────────────────────────────────────────────────────

pub fn color_edit3(label: &CStr, col: &mut [f32; 3], flags: ImGuiColorEditFlags) -> bool {
    unsafe { igColorEdit3(label.as_ptr(), col.as_mut_ptr(), flags) }
}

pub fn color_edit3_str(label: &str, col: &mut [f32; 3], flags: ImGuiColorEditFlags) -> bool {
    safe_cstring!(label => |ptr| igColorEdit3(ptr, col.as_mut_ptr(), flags), false)
}

pub fn color_edit4(label: &CStr, col: &mut [f32; 4], flags: ImGuiColorEditFlags) -> bool {
    unsafe { igColorEdit4(label.as_ptr(), col.as_mut_ptr(), flags) }
}

pub fn color_edit4_str(label: &str, col: &mut [f32; 4], flags: ImGuiColorEditFlags) -> bool {
    safe_cstring!(label => |ptr| igColorEdit4(ptr, col.as_mut_ptr(), flags), false)
}

pub fn color_picker3(label: &CStr, col: &mut [f32; 3], flags: ImGuiColorEditFlags) -> bool {
    unsafe { igColorPicker3(label.as_ptr(), col.as_mut_ptr(), flags) }
}

pub fn color_picker3_str(label: &str, col: &mut [f32; 3], flags: ImGuiColorEditFlags) -> bool {
    safe_cstring!(label => |ptr| igColorPicker3(ptr, col.as_mut_ptr(), flags), false)
}

pub fn color_picker4(label: &CStr, col: &mut [f32; 4], flags: ImGuiColorEditFlags) -> bool {
    unsafe { igColorPicker4(label.as_ptr(), col.as_mut_ptr(), flags, std::ptr::null()) }
}

pub fn color_picker4_str(label: &str, col: &mut [f32; 4], flags: ImGuiColorEditFlags) -> bool {
    safe_cstring!(label => |ptr| igColorPicker4(ptr, col.as_mut_ptr(), flags, std::ptr::null()), false)
}

pub fn color_button(desc_id: &CStr, col: ImVec4, flags: ImGuiColorEditFlags, size: ImVec2) -> bool {
    unsafe { igColorButton(desc_id.as_ptr(), col, flags, size) }
}

pub fn color_button_str(desc_id: &str, col: ImVec4, flags: ImGuiColorEditFlags, size: ImVec2) -> bool {
    safe_cstring!(desc_id => |ptr| igColorButton(ptr, col, flags, size), false)
}

pub fn color_convert_u32_to_float4(col: ImU32) -> ImVec4 {
    let mut out = ImVec4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    unsafe { igColorConvertU32ToFloat4(&mut out, col) }
    out
}

pub fn color_convert_float4_to_u32(col: ImVec4) -> ImU32 {
    unsafe { igColorConvertFloat4ToU32(col) }
}

// ── Layout ────────────────────────────────────────────────────────────────────

wrap_void! {
    separator          => igSeparator();
    same_line          => igSameLine(offset: f32, spacing: f32);
    new_line           => igNewLine();
    spacing            => igSpacing();
    dummy              => igDummy(size: ImVec2);
    indent             => igIndent(indent_w: f32);
    unindent           => igUnindent(indent_w: f32);
    begin_group        => igBeginGroup();
    end_group          => igEndGroup();
    align_text_to_frame_padding => igAlignTextToFramePadding();
    next_column        => igNextColumn();
    set_column_width   => igSetColumnWidth(column_index: i32, width: f32);
    set_column_offset  => igSetColumnOffset(column_index: i32, offset_x: f32);
}

pub fn columns(count: i32, id: Option<&CStr>, borders: bool) {
    match id {
        Some(s) => unsafe { igColumns(count, s.as_ptr(), borders) },
        None => unsafe { igColumns(count, std::ptr::null(), borders) },
    }
}

pub fn columns_str(count: i32, id: Option<&str>, borders: bool) {
    match id {
        Some(s) => safe_cstring!(s => |ptr| igColumns(count, ptr, borders)),
        None => unsafe { igColumns(count, std::ptr::null(), borders) },
    }
}

wrap_bool! {
    table_next_column    => igTableNextColumn();
    table_set_column_index => igTableSetColumnIndex(column_n: i32);
    table_next_row_bool  => igTableNextColumn();
    is_any_item_hovered  => igIsAnyItemHovered();
    is_any_item_active   => igIsAnyItemActive();
    is_any_item_focused  => igIsAnyItemFocused();
}

wrap_f32! {
    get_column_width  => igGetColumnWidth(column_index: i32);
    get_column_offset => igGetColumnOffset(column_index: i32);
}

pub fn get_column_index() -> i32 { unsafe { igGetColumnIndex() } }
pub fn get_columns_count() -> i32 { unsafe { igGetColumnsCount() } }
pub fn get_frame_count() -> i32 { unsafe { igGetFrameCount() } }
pub fn get_item_id() -> ImGuiID { unsafe { igGetItemID() } }

// ── Tables ────────────────────────────────────────────────────────────────────

pub fn begin_table(str_id: &CStr, columns: i32, flags: ImGuiTableFlags, outer_size: ImVec2, inner_width: f32) -> bool {
    unsafe { igBeginTable(str_id.as_ptr(), columns, flags, outer_size, inner_width) }
}

pub fn begin_table_str(str_id: &str, columns: i32, flags: ImGuiTableFlags, outer_size: ImVec2, inner_width: f32) -> bool {
    safe_cstring!(str_id => |ptr| igBeginTable(ptr, columns, flags, outer_size, inner_width), false)
}

wrap_void! {
    end_table              => igEndTable();
    table_next_row         => igTableNextRow(row_flags: ImGuiTableRowFlags, min_row_height: f32);
    table_setup_scroll_freeze => igTableSetupScrollFreeze(cols: i32, rows: i32);
    table_headers_row      => igTableHeadersRow();
    table_angled_headers_row => igTableAngledHeadersRow();
    table_set_bg_color     => igTableSetBgColor(target: ImGuiTableBgTarget, color: ImU32, column_n: i32);
    table_set_column_enabled => igTableSetColumnEnabled(column_n: i32, v: bool);
}

pub fn table_setup_column(label: &CStr, flags: ImGuiTableColumnFlags, init_width: f32, user_id: ImGuiID) {
    unsafe { igTableSetupColumn(label.as_ptr(), flags, init_width, user_id) }
}

pub fn table_setup_column_str(label: &str, flags: ImGuiTableColumnFlags, init_width: f32, user_id: ImGuiID) {
    safe_cstring!(label => |ptr| igTableSetupColumn(ptr, flags, init_width, user_id));
}

pub fn table_header(label: &CStr) {
    unsafe { igTableHeader(label.as_ptr()) }
}

pub fn table_header_str(label: &str) {
    safe_cstring!(label => |ptr| igTableHeader(ptr));
}

pub fn table_get_sort_specs() -> *mut ImGuiTableSortSpecs {
    unsafe { igTableGetSortSpecs() }
}

pub fn table_get_column_count() -> i32 { unsafe { igTableGetColumnCount() } }
pub fn table_get_column_index() -> i32 { unsafe { igTableGetColumnIndex() } }
pub fn table_get_row_index() -> i32 { unsafe { igTableGetRowIndex() } }
pub fn table_get_hovered_column() -> i32 { unsafe { igTableGetHoveredColumn() } }

pub fn table_get_column_flags(column_n: i32) -> ImGuiTableColumnFlags {
    unsafe { igTableGetColumnFlags(column_n) }
}

pub fn table_get_column_name(column_n: i32) -> Option<String> {
    unsafe { 
        let ptr = igTableGetColumnName_Int(column_n);
        if ptr.is_null() { return None; }
        CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_string())
    }
}

// ── Trees ─────────────────────────────────────────────────────────────────────

pub fn tree_node(label: &CStr) -> bool {
    unsafe { igTreeNode_Str(label.as_ptr()) }
}

pub fn tree_node_str(label: &str) -> bool {
    safe_cstring!(label => |ptr| igTreeNode_Str(ptr), false)
}

pub fn tree_node_ex(label: &CStr, flags: ImGuiTreeNodeFlags) -> bool {
    unsafe { igTreeNodeEx_Str(label.as_ptr(), flags) }
}

pub fn tree_node_ex_str(label: &str, flags: ImGuiTreeNodeFlags) -> bool {
    safe_cstring!(label => |ptr| igTreeNodeEx_Str(ptr, flags), false)
}

pub fn tree_push(str_id: &CStr) {
    unsafe { igTreePush_Str(str_id.as_ptr()) }
}

pub fn tree_push_str(str_id: &str) {
    safe_cstring!(str_id => |ptr| igTreePush_Str(ptr));
}

pub fn tree_push_ptr(ptr_id: *const ::std::os::raw::c_void) {
    unsafe { igTreePush_Ptr(ptr_id) }
}

wrap_void! { tree_pop => igTreePop(); }

pub fn collapsing_header(label: &CStr, flags: ImGuiTreeNodeFlags) -> bool {
    unsafe { igCollapsingHeader_TreeNodeFlags(label.as_ptr(), flags) }
}

pub fn collapsing_header_str(label: &str, flags: ImGuiTreeNodeFlags) -> bool {
    safe_cstring!(label => |ptr| igCollapsingHeader_TreeNodeFlags(ptr, flags), false)
}

pub fn collapsing_header_open(label: &CStr, p_visible: &mut bool, flags: ImGuiTreeNodeFlags) -> bool {
    unsafe { igCollapsingHeader_BoolPtr(label.as_ptr(), p_visible as *mut bool, flags) }
}

pub fn collapsing_header_open_str(label: &str, p_visible: &mut bool, flags: ImGuiTreeNodeFlags) -> bool {
    safe_cstring!(label => |ptr| igCollapsingHeader_BoolPtr(ptr, p_visible as *mut bool, flags), false)
}

wrap_void! {
    set_next_item_open       => igSetNextItemOpen(is_open: bool, cond: ImGuiCond);
    set_next_item_storage_id => igSetNextItemStorageID(storage_id: ImGuiID);
}

// ── Tabs ──────────────────────────────────────────────────────────────────────

pub fn begin_tab_bar(str_id: &CStr, flags: ImGuiTabBarFlags) -> bool {
    unsafe { igBeginTabBar(str_id.as_ptr(), flags) }
}

pub fn begin_tab_bar_str(str_id: &str, flags: ImGuiTabBarFlags) -> bool {
    safe_cstring!(str_id => |ptr| igBeginTabBar(ptr, flags), false)
}

wrap_void! { end_tab_bar => igEndTabBar(); }

pub fn begin_tab_item(label: &CStr, p_open: Option<&mut bool>, flags: ImGuiTabItemFlags) -> bool {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    unsafe { igBeginTabItem(label.as_ptr(), p, flags) }
}

pub fn begin_tab_item_str(label: &str, p_open: Option<&mut bool>, flags: ImGuiTabItemFlags) -> bool {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    safe_cstring!(label => |ptr| igBeginTabItem(ptr, p, flags), false)
}

wrap_void! { end_tab_item => igEndTabItem(); }

pub fn tab_item_button(label: &CStr, flags: ImGuiTabItemFlags) -> bool {
    unsafe { igTabItemButton(label.as_ptr(), flags) }
}

pub fn tab_item_button_str(label: &str, flags: ImGuiTabItemFlags) -> bool {
    safe_cstring!(label => |ptr| igTabItemButton(ptr, flags), false)
}

pub fn set_tab_item_closed(label: &CStr) {
    unsafe { igSetTabItemClosed(label.as_ptr()) }
}

pub fn set_tab_item_closed_str(label: &str) {
    safe_cstring!(label => |ptr| igSetTabItemClosed(ptr));
}

// ── Menus ─────────────────────────────────────────────────────────────────────

wrap_bool! {
    begin_menu_bar      => igBeginMenuBar();
    begin_main_menu_bar => igBeginMainMenuBar();
}

wrap_void! {
    end_menu_bar      => igEndMenuBar();
    end_main_menu_bar => igEndMainMenuBar();
}

pub fn begin_menu(label: &CStr, enabled: bool) -> bool {
    unsafe { igBeginMenu(label.as_ptr(), enabled) }
}

pub fn begin_menu_str(label: &str, enabled: bool) -> bool {
    safe_cstring!(label => |ptr| igBeginMenu(ptr, enabled), false)
}

wrap_void! { end_menu => igEndMenu(); }

pub fn menu_item(label: &CStr, shortcut: Option<&CStr>, selected: bool, enabled: bool) -> bool {
    let shortcut_ptr = shortcut.map_or(std::ptr::null(), |s| s.as_ptr());
    unsafe { igMenuItem_Bool(label.as_ptr(), shortcut_ptr, selected, enabled) }
}

pub fn menu_item_str(label: &str, shortcut: Option<&str>, selected: bool, enabled: bool) -> bool {
    let label = CString::new(label).unwrap_or_default();
    let shortcut = shortcut.and_then(|s| CString::new(s).ok());
    let shortcut_ptr = shortcut.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());
    unsafe { igMenuItem_Bool(label.as_ptr(), shortcut_ptr, selected, enabled) }
}

pub fn menu_item_toggle(label: &CStr, shortcut: Option<&CStr>, p_selected: &mut bool, enabled: bool) -> bool {
    let shortcut_ptr = shortcut.map_or(std::ptr::null(), |s| s.as_ptr());
    unsafe { igMenuItem_BoolPtr(label.as_ptr(), shortcut_ptr, p_selected as *mut bool, enabled) }
}

pub fn menu_item_toggle_str(label: &str, shortcut: Option<&str>, p_selected: &mut bool, enabled: bool) -> bool {
    let label = CString::new(label).unwrap_or_default();
    let shortcut = shortcut.and_then(|s| CString::new(s).ok());
    let shortcut_ptr = shortcut.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());
    unsafe { igMenuItem_BoolPtr(label.as_ptr(), shortcut_ptr, p_selected as *mut bool, enabled) }
}

// ── Tooltips ──────────────────────────────────────────────────────────────────

wrap_bool! {
    begin_tooltip      => igBeginTooltip();
    begin_item_tooltip => igBeginItemTooltip();
}

wrap_void! { end_tooltip => igEndTooltip(); }

pub fn set_tooltip(s: &CStr) {
    unsafe { igSetTooltip(s.as_ptr()) }
}

pub fn set_tooltip_str(s: &str) {
    safe_cstring!(s => |ptr: *const i8| igSetTooltip(c"%s".as_ptr(), ptr));
}

pub fn set_item_tooltip(s: &CStr) {
    unsafe { igSetItemTooltip(s.as_ptr()) }
}

pub fn set_item_tooltip_str(s: &str) {
    safe_cstring!(s => |ptr: *const i8| igSetItemTooltip(c"%s".as_ptr(), ptr));
}

// ── Popups ────────────────────────────────────────────────────────────────────

pub fn begin_popup(str_id: &CStr, flags: ImGuiWindowFlags) -> bool {
    unsafe { igBeginPopup(str_id.as_ptr(), flags) }
}

pub fn begin_popup_str(str_id: &str, flags: ImGuiWindowFlags) -> bool {
    safe_cstring!(str_id => |ptr| igBeginPopup(ptr, flags), false)
}

pub fn begin_popup_modal(name: &CStr, p_open: Option<&mut bool>, flags: ImGuiWindowFlags) -> bool {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    unsafe { igBeginPopupModal(name.as_ptr(), p, flags) }
}

pub fn begin_popup_modal_str(name: &str, p_open: Option<&mut bool>, flags: ImGuiWindowFlags) -> bool {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    safe_cstring!(name => |ptr| igBeginPopupModal(ptr, p, flags), false)
}

wrap_void! { end_popup => igEndPopup(); }

pub fn open_popup(str_id: &CStr, flags: ImGuiPopupFlags) {
    unsafe { igOpenPopup_Str(str_id.as_ptr(), flags) }
}

pub fn open_popup_str(str_id: &str, flags: ImGuiPopupFlags) {
    safe_cstring!(str_id => |ptr| igOpenPopup_Str(ptr, flags));
}

pub fn open_popup_id(id: ImGuiID, flags: ImGuiPopupFlags) {
    unsafe { igOpenPopup_ID(id, flags) }
}

pub fn open_popup_on_item_click(str_id: Option<&CStr>, flags: ImGuiPopupFlags) {
    match str_id {
        Some(s) => unsafe { igOpenPopupOnItemClick(s.as_ptr(), flags) },
        None => unsafe { igOpenPopupOnItemClick(std::ptr::null(), flags) },
    }
}

pub fn open_popup_on_item_click_str(str_id: Option<&str>, flags: ImGuiPopupFlags) {
    match str_id {
        Some(s) => safe_cstring!(s => |ptr| igOpenPopupOnItemClick(ptr, flags)),
        None => unsafe { igOpenPopupOnItemClick(std::ptr::null(), flags) },
    }
}

wrap_void! { close_current_popup => igCloseCurrentPopup(); }

pub fn begin_popup_context_item(str_id: Option<&CStr>, flags: ImGuiPopupFlags) -> bool {
    match str_id {
        Some(s) => unsafe { igBeginPopupContextItem(s.as_ptr(), flags) },
        None => unsafe { igBeginPopupContextItem(std::ptr::null(), flags) },
    }
}

pub fn begin_popup_context_item_str(str_id: Option<&str>, flags: ImGuiPopupFlags) -> bool {
    match str_id {
        Some(s) => safe_cstring!(s => |ptr| igBeginPopupContextItem(ptr, flags), false),
        None => unsafe { igBeginPopupContextItem(std::ptr::null(), flags) },
    }
}

pub fn begin_popup_context_window(str_id: Option<&CStr>, flags: ImGuiPopupFlags) -> bool {
    match str_id {
        Some(s) => unsafe { igBeginPopupContextWindow(s.as_ptr(), flags) },
        None => unsafe { igBeginPopupContextWindow(std::ptr::null(), flags) },
    }
}

pub fn begin_popup_context_window_str(str_id: Option<&str>, flags: ImGuiPopupFlags) -> bool {
    match str_id {
        Some(s) => safe_cstring!(s => |ptr| igBeginPopupContextWindow(ptr, flags), false),
        None => unsafe { igBeginPopupContextWindow(std::ptr::null(), flags) },
    }
}

pub fn is_popup_open(str_id: &CStr, flags: ImGuiPopupFlags) -> bool {
    unsafe { igIsPopupOpen_Str(str_id.as_ptr(), flags) }
}

pub fn is_popup_open_str(str_id: &str, flags: ImGuiPopupFlags) -> bool {
    safe_cstring!(str_id => |ptr| igIsPopupOpen_Str(ptr, flags), false)
}

// ── Combo ─────────────────────────────────────────────────────────────────────

pub fn begin_combo(label: &CStr, preview: &CStr, flags: ImGuiComboFlags) -> bool {
    unsafe { igBeginCombo(label.as_ptr(), preview.as_ptr(), flags) }
}

pub fn begin_combo_str(label: &str, preview: &str, flags: ImGuiComboFlags) -> bool {
    let label = CString::new(label).unwrap_or_default();
    let preview = CString::new(preview).unwrap_or_default();
    unsafe { igBeginCombo(label.as_ptr(), preview.as_ptr(), flags) }
}

wrap_void! { end_combo => igEndCombo(); }

pub fn combo(label: &CStr, current_item: &mut i32, items: &[&CStr], height_in_items: i32) -> bool {
    let items_ptr: Vec<*const i8> = items.iter().map(|s| s.as_ptr()).collect();
    unsafe {
        igCombo_Str_arr(label.as_ptr(), current_item as *mut i32, items_ptr.as_ptr(), items.len() as i32, height_in_items)
    }
}

pub fn combo_str(label: &str, current_item: &mut i32, items: &[&str], height_in_items: i32) -> bool {
    let label = CString::new(label).unwrap_or_default();
    let items_c: Vec<CString> = items.iter().map(|s| CString::new(*s).unwrap_or_default()).collect();
    let items_ptr: Vec<*const i8> = items_c.iter().map(|s| s.as_ptr()).collect();
    unsafe {
        igCombo_Str_arr(label.as_ptr(), current_item as *mut i32, items_ptr.as_ptr(), items.len() as i32, height_in_items)
    }
}

// ── Multi-select ──────────────────────────────────────────────────────────────

pub fn begin_multi_select(flags: ImGuiMultiSelectFlags, selection_size: i32, items_count: i32) -> *mut ImGuiMultiSelectIO {
    unsafe { igBeginMultiSelect(flags, selection_size, items_count) }
}

pub fn end_multi_select() -> *mut ImGuiMultiSelectIO {
    unsafe { igEndMultiSelect() }
}

wrap_void! {
    set_next_item_selection_user_data => igSetNextItemSelectionUserData(selection_user_data: ImGuiSelectionUserData);
}

wrap_bool! {
    is_item_toggled_selection => igIsItemToggledSelection();
}

// ── List box ──────────────────────────────────────────────────────────────────

pub fn begin_list_box(label: &CStr, size: ImVec2) -> bool {
    unsafe { igBeginListBox(label.as_ptr(), size) }
}

pub fn begin_list_box_str(label: &str, size: ImVec2) -> bool {
    safe_cstring!(label => |ptr| igBeginListBox(ptr, size), false)
}

wrap_void! { end_list_box => igEndListBox(); }

pub fn list_box(label: &CStr, current_item: &mut i32, items: &[&CStr], height_in_items: i32) -> bool {
    let items_ptr: Vec<*const i8> = items.iter().map(|s| s.as_ptr()).collect();
    unsafe {
        igListBox_Str_arr(label.as_ptr(), current_item as *mut i32, items_ptr.as_ptr(), items.len() as i32, height_in_items)
    }
}

pub fn list_box_str(label: &str, current_item: &mut i32, items: &[&str], height_in_items: i32) -> bool {
    let label = CString::new(label).unwrap_or_default();
    let items_c: Vec<CString> = items.iter().map(|s| CString::new(*s).unwrap_or_default()).collect();
    let items_ptr: Vec<*const i8> = items_c.iter().map(|s| s.as_ptr()).collect();
    unsafe {
        igListBox_Str_arr(label.as_ptr(), current_item as *mut i32, items_ptr.as_ptr(), items.len() as i32, height_in_items)
    }
}

// ── Item queries ──────────────────────────────────────────────────────────────

wrap_bool! {
    is_item_hovered             => igIsItemHovered(flags: ImGuiHoveredFlags);
    is_item_active              => igIsItemActive();
    is_item_focused             => igIsItemFocused();
    is_item_clicked             => igIsItemClicked(mouse_button: ImGuiMouseButton);
    is_item_visible             => igIsItemVisible();
    is_item_edited              => igIsItemEdited();
    is_item_activated           => igIsItemActivated();
    is_item_deactivated         => igIsItemDeactivated();
    is_item_deactivated_after_edit => igIsItemDeactivatedAfterEdit();
    is_item_toggled_open        => igIsItemToggledOpen();
    is_rect_visible_size        => igIsRectVisible_Nil(size: ImVec2);
    is_rect_visible             => igIsRectVisible_Vec2(rect_min: ImVec2, rect_max: ImVec2);
    is_mouse_hovering_rect      => igIsMouseHoveringRect(r_min: ImVec2, r_max: ImVec2, clip: bool);
    is_any_mouse_down           => igIsAnyMouseDown();
    is_mouse_dragging           => igIsMouseDragging(button: ImGuiMouseButton, lock_threshold: f32);
}

// ── Keyboard/Mouse ────────────────────────────────────────────────────────────

wrap_bool! {
    is_key_down     => igIsKeyDown_Nil(key: ImGuiKey);
    is_key_pressed  => igIsKeyPressed_Bool(key: ImGuiKey, repeat: bool);
    is_key_released => igIsKeyReleased_Nil(key: ImGuiKey);
    is_key_chord_pressed => igIsKeyChordPressed_Nil(key_chord: ImGuiKeyChord);
    is_mouse_down   => igIsMouseDown_Nil(button: ImGuiMouseButton);
    is_mouse_clicked => igIsMouseClicked_Bool(button: ImGuiMouseButton, repeat: bool);
    is_mouse_released => igIsMouseReleased_Nil(button: ImGuiMouseButton);
    is_mouse_double_clicked => igIsMouseDoubleClicked_Nil(button: ImGuiMouseButton);
    is_mouse_released_with_delay => igIsMouseReleasedWithDelay(button: ImGuiMouseButton, delay: f32);
    shortcut        => igShortcut_Nil(key_chord: ImGuiKeyChord, flags: ImGuiInputFlags);
}

wrap_void! {
    set_next_frame_want_capture_keyboard => igSetNextFrameWantCaptureKeyboard(want: bool);
    set_next_frame_want_capture_mouse    => igSetNextFrameWantCaptureMouse(want: bool);
    set_next_item_shortcut               => igSetNextItemShortcut(key_chord: ImGuiKeyChord, flags: ImGuiInputFlags);
    set_item_key_owner                   => igSetItemKeyOwner_Nil(key: ImGuiKey);
    set_nav_cursor_visible               => igSetNavCursorVisible(visible: bool);
    set_next_item_allow_overlap          => igSetNextItemAllowOverlap();
    reset_mouse_drag_delta               => igResetMouseDragDelta(button: ImGuiMouseButton);
    set_mouse_cursor                     => igSetMouseCursor(cursor_type: ImGuiMouseCursor);
    set_keyboard_focus_here              => igSetKeyboardFocusHere(offset: i32);
    set_item_default_focus               => igSetItemDefaultFocus();
}

pub fn get_mouse_drag_delta(button: ImGuiMouseButton, lock_threshold: f32) -> ImVec2 {
    let mut out = ImVec2 { x: 0.0, y: 0.0 };
    unsafe { igGetMouseDragDelta(&mut out, button, lock_threshold) }
    out
}

pub fn get_mouse_cursor() -> ImGuiMouseCursor {
    unsafe { igGetMouseCursor() }
}

pub fn get_mouse_clicked_count(button: ImGuiMouseButton) -> i32 {
    unsafe { igGetMouseClickedCount(button) }
}

pub fn get_key_pressed_amount(key: ImGuiKey, repeat_delay: f32, rate: f32) -> i32 {
    unsafe { igGetKeyPressedAmount(key, repeat_delay, rate) }
}

pub fn get_key_name(key: ImGuiKey) -> Option<&'static str> {
    unsafe { CStr::from_ptr(igGetKeyName(key)).to_str().ok() }
}

// ── Drag & Drop ───────────────────────────────────────────────────────────────

wrap_bool! {
    begin_drag_drop_source => igBeginDragDropSource(flags: ImGuiDragDropFlags);
    begin_drag_drop_target => igBeginDragDropTarget();
}

wrap_void! {
    end_drag_drop_source => igEndDragDropSource();
    end_drag_drop_target => igEndDragDropTarget();
}

pub fn set_drag_drop_payload(type_: &CStr, data: *const ::std::os::raw::c_void, sz: usize, cond: ImGuiCond) -> bool {
    unsafe { igSetDragDropPayload(type_.as_ptr(), data, sz, cond) }
}

pub fn set_drag_drop_payload_str(type_: &str, data: *const ::std::os::raw::c_void, sz: usize, cond: ImGuiCond) -> bool {
    safe_cstring!(type_ => |ptr| igSetDragDropPayload(ptr, data, sz, cond), false)
}

pub fn accept_drag_drop_payload(type_: &CStr, flags: ImGuiDragDropFlags) -> *const ImGuiPayload {
    unsafe { igAcceptDragDropPayload(type_.as_ptr(), flags) }
}

pub fn accept_drag_drop_payload_str(type_: &str, flags: ImGuiDragDropFlags) -> *const ImGuiPayload {
    match CString::new(type_) {
        Ok(s) => unsafe { igAcceptDragDropPayload(s.as_ptr(), flags) },
        Err(_) => std::ptr::null(),
    }
}

pub fn get_drag_drop_payload() -> *const ImGuiPayload {
    unsafe { igGetDragDropPayload() }
}

// ── Disabled ──────────────────────────────────────────────────────────────────

wrap_void! {
    begin_disabled => igBeginDisabled(disabled: bool);
    end_disabled   => igEndDisabled();
}

// ── Clip rect ─────────────────────────────────────────────────────────────────

wrap_void! {
    push_clip_rect => igPushClipRect(clip_rect_min: ImVec2, clip_rect_max: ImVec2, intersect_with_current: bool);
    pop_clip_rect  => igPopClipRect();
}

// ── Clipboard ─────────────────────────────────────────────────────────────────

pub fn get_clipboard_text() -> Option<String> {
    unsafe {
        let ptr = igGetClipboardText();
        if ptr.is_null() { return None; }
        CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_string())
    }
}

pub fn set_clipboard_text(text: &CStr) {
    unsafe { igSetClipboardText(text.as_ptr()) }
}

pub fn set_clipboard_text_str(text: &str) {
    safe_cstring!(text => |ptr| igSetClipboardText(ptr));
}

// ── Debug/Demo ────────────────────────────────────────────────────────────────

pub fn show_demo_window(p_open: Option<&mut bool>) {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    unsafe { igShowDemoWindow(p) }
}

pub fn show_metrics_window(p_open: Option<&mut bool>) {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    unsafe { igShowMetricsWindow(p) }
}

pub fn show_debug_log_window(p_open: Option<&mut bool>) {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    unsafe { igShowDebugLogWindow(p) }
}

pub fn show_about_window(p_open: Option<&mut bool>) {
    let p = p_open.map_or(std::ptr::null_mut(), |b| b as *mut bool);
    unsafe { igShowAboutWindow(p) }
}

wrap_void! {
    show_user_guide       => igShowUserGuide();
    debug_flash_style_color => igDebugFlashStyleColor(idx: ImGuiCol);
    debug_start_item_picker => igDebugStartItemPicker();
    log_to_tty            => igLogToTTY(auto_open_depth: i32);
    log_to_clipboard      => igLogToClipboard(auto_open_depth: i32);
    log_finish            => igLogFinish();
    log_buttons           => igLogButtons();
}

pub fn log_to_file(auto_open_depth: i32, filename: Option<&CStr>) {
    match filename {
        Some(s) => unsafe { igLogToFile(auto_open_depth, s.as_ptr()) },
        None => unsafe { igLogToFile(auto_open_depth, std::ptr::null()) },
    }
}

pub fn log_to_file_str(auto_open_depth: i32, filename: Option<&str>) {
    match filename {
        Some(s) => safe_cstring!(s => |ptr| igLogToFile(auto_open_depth, ptr)),
        None => unsafe { igLogToFile(auto_open_depth, std::ptr::null()) },
    }
}

// ── Value helpers ─────────────────────────────────────────────────────────────

pub fn value_bool(prefix: &CStr, b: bool) {
    unsafe { igValue_Bool(prefix.as_ptr(), b) }
}

pub fn value_bool_str(prefix: &str, b: bool) {
    safe_cstring!(prefix => |ptr| igValue_Bool(ptr, b));
}

pub fn value_int(prefix: &CStr, v: i32) {
    unsafe { igValue_Int(prefix.as_ptr(), v) }
}

pub fn value_int_str(prefix: &str, v: i32) {
    safe_cstring!(prefix => |ptr| igValue_Int(ptr, v));
}

pub fn value_uint(prefix: &CStr, v: u32) {
    unsafe { igValue_Uint(prefix.as_ptr(), v) }
}

pub fn value_uint_str(prefix: &str, v: u32) {
    safe_cstring!(prefix => |ptr| igValue_Uint(ptr, v));
}

pub fn value_float(prefix: &CStr, v: f32, format: &CStr) {
    unsafe { igValue_Float(prefix.as_ptr(), v, format.as_ptr()) }
}

pub fn value_float_str(prefix: &str, v: f32, format: &CStr) {
    safe_cstring!(prefix => |ptr| igValue_Float(ptr, v, format.as_ptr()));
}

// ── Viewport ──────────────────────────────────────────────────────────────────

pub fn get_main_viewport() -> *mut ImGuiViewport {
    unsafe { igGetMainViewport() }
}

pub fn get_background_draw_list() -> *mut ImDrawList {
    unsafe { igGetBackgroundDrawList_Nil() }
}

pub fn get_foreground_draw_list() -> *mut ImDrawList {
    unsafe { igGetForegroundDrawList_Nil() }
}
