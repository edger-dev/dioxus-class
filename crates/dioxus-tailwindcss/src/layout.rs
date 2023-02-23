// https://tailwindcss.com/docs/aspect-ratio
pub const aspect_auto: &'static str = "aspect-auto";
pub const aspect_square: &'static str = "aspect-square";
pub const aspect_video: &'static str = "aspect-video";

pub fn aspect(w: u16, h: u16) -> String {
    format!("aspect-[{}/{}]", w, h)
}

// https://tailwindcss.com/docs/container
pub const container: &'static str = "container";

// https://tailwindcss.com/docs/columns
pub const columns_1: &'static str = "columns-1";
pub const columns_2: &'static str = "columns-2";
pub const columns_3: &'static str = "columns-3";
pub const columns_4: &'static str = "columns-4";
pub const columns_5: &'static str = "columns-5";
pub const columns_6: &'static str = "columns-6";
pub const columns_7: &'static str = "columns-7";
pub const columns_8: &'static str = "columns-8";
pub const columns_9: &'static str = "columns-9";
pub const columns_10: &'static str = "columns-10";
pub const columns_11: &'static str = "columns-11";
pub const columns_12: &'static str = "columns-12";
pub const columns_auto: &'static str = "columns-auto";
pub const columns_3xs: &'static str = "columns-3xs";
pub const columns_2xs: &'static str = "columns-2xs";
pub const columns_xs: &'static str = "columns-xs";
pub const columns_sm: &'static str = "columns-sm";
pub const columns_md: &'static str = "columns-md";
pub const columns_lg: &'static str = "columns-lg";
pub const columns_xl: &'static str = "columns-xl";
pub const columns_2xl: &'static str = "columns-2xl";
pub const columns_3xl: &'static str = "columns-3xl";
pub const columns_4xl: &'static str = "columns-4xl";
pub const columns_5xl: &'static str = "columns-5xl";
pub const columns_6xl: &'static str = "columns-6xl";
pub const columns_7xl: &'static str = "columns-7xl";

pub fn columns_any(p: &str) -> String {
    format!("columns-[{}]", p)
}

// https://tailwindcss.com/docs/break-after
pub const break_after_auto: &'static str = "break-after-auto";
pub const break_after_avoid: &'static str = "break-after-avoid";
pub const break_after_all: &'static str = "break-after-all";
pub const break_after_avoid_page: &'static str = "break-after-avoid-page";
pub const break_after_page: &'static str = "break-after-page";
pub const break_after_left: &'static str = "break-after-left";
pub const break_after_right: &'static str = "break-after-right";
pub const break_after_column: &'static str = "break-after-column";

// https://tailwindcss.com/docs/break-before
pub const break_before_auto: &'static str = "break-before-auto";
pub const break_before_avoid: &'static str = "break-before-avoid";
pub const break_before_all: &'static str = "break-before-all";
pub const break_before_avoid_page: &'static str = "break-before-avoid-page";
pub const break_before_page: &'static str = "break-before-page";
pub const break_before_left: &'static str = "break-before-left";
pub const break_before_right: &'static str = "break-before-right";
pub const break_before_column: &'static str = "break-before-column";

// https://tailwindcss.com/docs/break-inside
pub const break_inside_auto: &'static str = "break-inside-auto";
pub const break_inside_avoid: &'static str = "break-inside-avoid";
pub const break_inside_avoid_page: &'static str = "break-inside-avoid-page";
pub const break_inside_avoid_column: &'static str = "break-inside-avoid-column";

// https://tailwindcss.com/docs/box-decoration-break
pub const box_decoration_clone: &'static str = "box-decoration-clone";
pub const box_decoration_slice: &'static str = "box-decoration-slice";

// https://tailwindcss.com/docs/box-sizing
pub const box_border: &'static str = "box-border";
pub const box_content: &'static str = "box-content";

// https://tailwindcss.com/docs/display
pub const block: &'static str = "block";
pub const inline_block: &'static str = "inline-block";
pub const inline: &'static str = "inline";
pub const flex: &'static str = "flex";
pub const inline_flex: &'static str = "inline-flex";

pub const table: &'static str = "table";
pub const inline_table: &'static str = "inline-table";
pub const table_caption: &'static str = "table-caption";
pub const table_: &'static str = "table-cell";
pub const table_column: &'static str = "table-column";
pub const table_column_group: &'static str = "table-column-group";
pub const table_footer_group: &'static str = "table-footer-group";
pub const table_header_group: &'static str = "table-header-group";
pub const table_row_group: &'static str = "table-row-group";
pub const table_row: &'static str = "table-row";
pub const flow_root: &'static str = "flow-root";
pub const grid: &'static str = "grid";
pub const inline_grid: &'static str = "inline-grid";
pub const contents: &'static str = "contents";
pub const list_item: &'static str = "list-item";
pub const hidden: &'static str = "hidden";

// https://tailwindcss.com/docs/float
pub const float_right: &'static str = "float-right";
pub const float_left: &'static str = "float-left";
pub const float_none: &'static str = "float-none";

// https://tailwindcss.com/docs/clear
pub const clear_left: &'static str = "clear-left";
pub const clear_right: &'static str = "clear-right";
pub const clear_both: &'static str = "clear-both";
pub const clear_none: &'static str = "clear-none";

// https://tailwindcss.com/docs/isolation
pub const isolate: &'static str = "isolate";
pub const isolation_auto: &'static str = "isolation-auto";

// https://tailwindcss.com/docs/object-fit
pub const object_contain: &'static str = "object-contain";
pub const object_cover: &'static str = "object-cover";
pub const object_fill: &'static str = "object-fill";
pub const object_none: &'static str = "object-none";
pub const object_scale_down: &'static str = "object-scale-down";

// https://tailwindcss.com/docs/object-position
pub const object_bottom: &'static str = "object-bottom";
pub const object_center: &'static str = "object-center";
pub const object_left: &'static str = "object-left";
pub const object_left_bottom: &'static str = "object-left-bottom";
pub const object_left_top: &'static str = "object-left-top";
pub const object_right: &'static str = "object-right";
pub const object_right_bottom: &'static str = "object-right-bottom";
pub const object_right_top: &'static str = "object-right-top";
pub const object_top: &'static str = "object-top";

// https://tailwindcss.com/docs/overflow
pub const overflow_auto: &'static str = "overflow-auto";
pub const overflow_hidden: &'static str = "overflow-hidden";
pub const overflow_clip: &'static str = "overflow-clip";
pub const overflow_visible: &'static str = "overflow-visible";
pub const overflow_scroll: &'static str = "overflow-scroll";
pub const overflow_x_auto: &'static str = "overflow-x-auto";
pub const overflow_y_auto: &'static str = "overflow-y-auto";
pub const overflow_x_hidden: &'static str = "overflow-x-hidden";
pub const overflow_y_hidden: &'static str = "overflow-y-hidden";
pub const overflow_x_clip: &'static str = "overflow-x-clip";
pub const overflow_y_clip: &'static str = "overflow-y-clip";
pub const overflow_x_visible: &'static str = "overflow-x-visible";
pub const overflow_y_visible: &'static str = "overflow-y-visible";
pub const overflow_x_scroll: &'static str = "overflow-x-scroll";
pub const overflow_y_scroll: &'static str = "overflow-y-scroll";

// https://tailwindcss.com/docs/overscroll-behavior
pub const overscroll_auto: &'static str = "overscroll-auto";
pub const overscroll_contain: &'static str = "overscroll-contain";
pub const overscroll_none: &'static str = "overscroll-none";
pub const overscroll_y_auto: &'static str = "overscroll-y-auto";
pub const overscroll_y_contain: &'static str = "overscroll-y-contain";
pub const overscroll_y_none: &'static str = "overscroll-y-none";
pub const overscroll_x_auto: &'static str = "overscroll-x-auto";
pub const overscroll_x_contain: &'static str = "overscroll-x-contain";
pub const overscroll_x_none: &'static str = "overscroll-x-none";

// https://tailwindcss.com/docs/position
pub const static_: &'static str = "static";
pub const fixed: &'static str = "fixed";
pub const absolute: &'static str = "absolute";
pub const relative: &'static str = "relative";
pub const sticky: &'static str = "sticky";

// https://tailwindcss.com/docs/top-right-bottom-left
pub const top_auto: &'static str = "top-auto";
pub const top_0: &'static str = "top-0";
pub const top_px: &'static str = "top-px";
pub const top_half: &'static str = "top-0.5";
pub const top_1: &'static str = "top-1";
pub const top_1_half: &'static str = "top-1.5";
pub const top_2: &'static str = "top-2";
pub const top_2_half: &'static str = "top-2.5";
pub const top_3: &'static str = "top-3";
pub const top_3_half: &'static str = "top-3.5";
pub const top_4: &'static str = "top-4";
pub const top_5: &'static str = "top-5";
pub const top_6: &'static str = "top-6";
pub const top_7: &'static str = "top-7";
pub const top_8: &'static str = "top-8";
pub const top_9: &'static str = "top-9";
pub const top_10: &'static str = "top-10";
pub const top_11: &'static str = "top-11";
pub const top_12: &'static str = "top-12";
pub const top_16: &'static str = "top-16";
pub const top_20: &'static str = "top-20";
pub const top_24: &'static str = "top-24";
pub const top_28: &'static str = "top-28";
pub const top_32: &'static str = "top-32";
pub const top_36: &'static str = "top-36";
pub const top_40: &'static str = "top-40";
pub const top_44: &'static str = "top-44";
pub const top_48: &'static str = "top-48";
pub const top_52: &'static str = "top-52";
pub const top_56: &'static str = "top-56";
pub const top_60: &'static str = "top-60";
pub const top_64: &'static str = "top-64";
pub const top_72: &'static str = "top-72";
pub const top_80: &'static str = "top-80";
pub const top_96: &'static str = "top-96";
pub const top_1_2: &'static str = "top-1/2";
pub const top_1_3: &'static str = "top-1/3";
pub const top_2_3: &'static str = "top-2/3";
pub const top_1_4: &'static str = "top-1/4";
pub const top_2_4: &'static str = "top-2/4";
pub const top_3_4: &'static str = "top-3/4";
pub const top_full: &'static str = "top-full";

pub fn top_any(p: &str) -> String {
    format!("top-[{}]", p)
}

pub const bottom_auto: &'static str = "bottom-auto";
pub const bottom_0: &'static str = "bottom-0";
pub const bottom_px: &'static str = "bottom-px";
pub const bottom_half: &'static str = "bottom-0.5";
pub const bottom_1: &'static str = "bottom-1";
pub const bottom_1_half: &'static str = "bottom-1.5";
pub const bottom_2: &'static str = "bottom-2";
pub const bottom_2_half: &'static str = "bottom-2.5";
pub const bottom_3: &'static str = "bottom-3";
pub const bottom_3_half: &'static str = "bottom-3.5";
pub const bottom_4: &'static str = "bottom-4";
pub const bottom_5: &'static str = "bottom-5";
pub const bottom_6: &'static str = "bottom-6";
pub const bottom_7: &'static str = "bottom-7";
pub const bottom_8: &'static str = "bottom-8";
pub const bottom_9: &'static str = "bottom-9";
pub const bottom_10: &'static str = "bottom-10";
pub const bottom_11: &'static str = "bottom-11";
pub const bottom_12: &'static str = "bottom-12";
pub const bottom_16: &'static str = "bottom-16";
pub const bottom_20: &'static str = "bottom-20";
pub const bottom_24: &'static str = "bottom-24";
pub const bottom_28: &'static str = "bottom-28";
pub const bottom_32: &'static str = "bottom-32";
pub const bottom_36: &'static str = "bottom-36";
pub const bottom_40: &'static str = "bottom-40";
pub const bottom_44: &'static str = "bottom-44";
pub const bottom_48: &'static str = "bottom-48";
pub const bottom_52: &'static str = "bottom-52";
pub const bottom_56: &'static str = "bottom-56";
pub const bottom_60: &'static str = "bottom-60";
pub const bottom_64: &'static str = "bottom-64";
pub const bottom_72: &'static str = "bottom-72";
pub const bottom_80: &'static str = "bottom-80";
pub const bottom_96: &'static str = "bottom-96";
pub const bottom_1_2: &'static str = "bottom-1/2";
pub const bottom_1_3: &'static str = "bottom-1/3";
pub const bottom_2_3: &'static str = "bottom-2/3";
pub const bottom_1_4: &'static str = "bottom-1/4";
pub const bottom_2_4: &'static str = "bottom-2/4";
pub const bottom_3_4: &'static str = "bottom-3/4";
pub const bottom_full: &'static str = "bottom-full";

pub fn bottom_any(p: &str) -> String {
    format!("bottom-[{}]", p)
}

pub const left_auto: &'static str = "left-auto";
pub const left_0: &'static str = "left-0";
pub const left_px: &'static str = "left-px";
pub const left_half: &'static str = "left-0.5";
pub const left_1: &'static str = "left-1";
pub const left_1_half: &'static str = "left-1.5";
pub const left_2: &'static str = "left-2";
pub const left_2_half: &'static str = "left-2.5";
pub const left_3: &'static str = "left-3";
pub const left_3_half: &'static str = "left-3.5";
pub const left_4: &'static str = "left-4";
pub const left_5: &'static str = "left-5";
pub const left_6: &'static str = "left-6";
pub const left_7: &'static str = "left-7";
pub const left_8: &'static str = "left-8";
pub const left_9: &'static str = "left-9";
pub const left_10: &'static str = "left-10";
pub const left_11: &'static str = "left-11";
pub const left_12: &'static str = "left-12";
pub const left_16: &'static str = "left-16";
pub const left_20: &'static str = "left-20";
pub const left_24: &'static str = "left-24";
pub const left_28: &'static str = "left-28";
pub const left_32: &'static str = "left-32";
pub const left_36: &'static str = "left-36";
pub const left_40: &'static str = "left-40";
pub const left_44: &'static str = "left-44";
pub const left_48: &'static str = "left-48";
pub const left_52: &'static str = "left-52";
pub const left_56: &'static str = "left-56";
pub const left_60: &'static str = "left-60";
pub const left_64: &'static str = "left-64";
pub const left_72: &'static str = "left-72";
pub const left_80: &'static str = "left-80";
pub const left_96: &'static str = "left-96";
pub const left_1_2: &'static str = "left-1/2";
pub const left_1_3: &'static str = "left-1/3";
pub const left_2_3: &'static str = "left-2/3";
pub const left_1_4: &'static str = "left-1/4";
pub const left_2_4: &'static str = "left-2/4";
pub const left_3_4: &'static str = "left-3/4";
pub const left_full: &'static str = "left-full";

pub fn left_any(p: &str) -> String {
    format!("left-[{}]", p)
}

pub const right_auto: &'static str = "right-auto";
pub const right_0: &'static str = "right-0";
pub const right_px: &'static str = "right-px";
pub const right_half: &'static str = "right-0.5";
pub const right_1: &'static str = "right-1";
pub const right_1_half: &'static str = "right-1.5";
pub const right_2: &'static str = "right-2";
pub const right_2_half: &'static str = "right-2.5";
pub const right_3: &'static str = "right-3";
pub const right_3_half: &'static str = "right-3.5";
pub const right_4: &'static str = "right-4";
pub const right_5: &'static str = "right-5";
pub const right_6: &'static str = "right-6";
pub const right_7: &'static str = "right-7";
pub const right_8: &'static str = "right-8";
pub const right_9: &'static str = "right-9";
pub const right_10: &'static str = "right-10";
pub const right_11: &'static str = "right-11";
pub const right_12: &'static str = "right-12";
pub const right_16: &'static str = "right-16";
pub const right_20: &'static str = "right-20";
pub const right_24: &'static str = "right-24";
pub const right_28: &'static str = "right-28";
pub const right_32: &'static str = "right-32";
pub const right_36: &'static str = "right-36";
pub const right_40: &'static str = "right-40";
pub const right_44: &'static str = "right-44";
pub const right_48: &'static str = "right-48";
pub const right_52: &'static str = "right-52";
pub const right_56: &'static str = "right-56";
pub const right_60: &'static str = "right-60";
pub const right_64: &'static str = "right-64";
pub const right_72: &'static str = "right-72";
pub const right_80: &'static str = "right-80";
pub const right_96: &'static str = "right-96";
pub const right_1_2: &'static str = "right-1/2";
pub const right_1_3: &'static str = "right-1/3";
pub const right_2_3: &'static str = "right-2/3";
pub const right_1_4: &'static str = "right-1/4";
pub const right_2_4: &'static str = "right-2/4";
pub const right_3_4: &'static str = "right-3/4";
pub const right_full: &'static str = "right-full";

pub fn right_any(p: &str) -> String {
    format!("right-[{}]", p)
}

pub const inset_auto: &'static str = "inset-auto";
pub const inset_0: &'static str = "inset-0";
pub const inset_px: &'static str = "inset-px";
pub const inset_half: &'static str = "inset-0.5";
pub const inset_1: &'static str = "inset-1";
pub const inset_1_half: &'static str = "inset-1.5";
pub const inset_2: &'static str = "inset-2";
pub const inset_2_half: &'static str = "inset-2.5";
pub const inset_3: &'static str = "inset-3";
pub const inset_3_half: &'static str = "inset-3.5";
pub const inset_4: &'static str = "inset-4";
pub const inset_5: &'static str = "inset-5";
pub const inset_6: &'static str = "inset-6";
pub const inset_7: &'static str = "inset-7";
pub const inset_8: &'static str = "inset-8";
pub const inset_9: &'static str = "inset-9";
pub const inset_10: &'static str = "inset-10";
pub const inset_11: &'static str = "inset-11";
pub const inset_12: &'static str = "inset-12";
pub const inset_16: &'static str = "inset-16";
pub const inset_20: &'static str = "inset-20";
pub const inset_24: &'static str = "inset-24";
pub const inset_28: &'static str = "inset-28";
pub const inset_32: &'static str = "inset-32";
pub const inset_36: &'static str = "inset-36";
pub const inset_40: &'static str = "inset-40";
pub const inset_44: &'static str = "inset-44";
pub const inset_48: &'static str = "inset-48";
pub const inset_52: &'static str = "inset-52";
pub const inset_56: &'static str = "inset-56";
pub const inset_60: &'static str = "inset-60";
pub const inset_64: &'static str = "inset-64";
pub const inset_72: &'static str = "inset-72";
pub const inset_80: &'static str = "inset-80";
pub const inset_96: &'static str = "inset-96";
pub const inset_1_2: &'static str = "inset-1/2";
pub const inset_1_3: &'static str = "inset-1/3";
pub const inset_2_3: &'static str = "inset-2/3";
pub const inset_1_4: &'static str = "inset-1/4";
pub const inset_2_4: &'static str = "inset-2/4";
pub const inset_3_4: &'static str = "inset-3/4";
pub const inset_full: &'static str = "inset-full";

pub fn inset_any(p: &str) -> String {
    format!("inset-[{}]", p)
}

pub const inset_x_auto: &'static str = "inset-x-auto";
pub const inset_x_0: &'static str = "inset-x-0";
pub const inset_x_px: &'static str = "inset-x-px";
pub const inset_x_half: &'static str = "inset-x-0.5";
pub const inset_x_1: &'static str = "inset-x-1";
pub const inset_x_1_half: &'static str = "inset-x-1.5";
pub const inset_x_2: &'static str = "inset-x-2";
pub const inset_x_2_half: &'static str = "inset-x-2.5";
pub const inset_x_3: &'static str = "inset-x-3";
pub const inset_x_3_half: &'static str = "inset-x-3.5";
pub const inset_x_4: &'static str = "inset-x-4";
pub const inset_x_5: &'static str = "inset-x-5";
pub const inset_x_6: &'static str = "inset-x-6";
pub const inset_x_7: &'static str = "inset-x-7";
pub const inset_x_8: &'static str = "inset-x-8";
pub const inset_x_9: &'static str = "inset-x-9";
pub const inset_x_10: &'static str = "inset-x-10";
pub const inset_x_11: &'static str = "inset-x-11";
pub const inset_x_12: &'static str = "inset-x-12";
pub const inset_x_16: &'static str = "inset-x-16";
pub const inset_x_20: &'static str = "inset-x-20";
pub const inset_x_24: &'static str = "inset-x-24";
pub const inset_x_28: &'static str = "inset-x-28";
pub const inset_x_32: &'static str = "inset-x-32";
pub const inset_x_36: &'static str = "inset-x-36";
pub const inset_x_40: &'static str = "inset-x-40";
pub const inset_x_44: &'static str = "inset-x-44";
pub const inset_x_48: &'static str = "inset-x-48";
pub const inset_x_52: &'static str = "inset-x-52";
pub const inset_x_56: &'static str = "inset-x-56";
pub const inset_x_60: &'static str = "inset-x-60";
pub const inset_x_64: &'static str = "inset-x-64";
pub const inset_x_72: &'static str = "inset-x-72";
pub const inset_x_80: &'static str = "inset-x-80";
pub const inset_x_96: &'static str = "inset-x-96";
pub const inset_x_1_2: &'static str = "inset-x-1/2";
pub const inset_x_1_3: &'static str = "inset-x-1/3";
pub const inset_x_2_3: &'static str = "inset-x-2/3";
pub const inset_x_1_4: &'static str = "inset-x-1/4";
pub const inset_x_2_4: &'static str = "inset-x-2/4";
pub const inset_x_3_4: &'static str = "inset-x-3/4";
pub const inset_x_full: &'static str = "inset-x-full";

pub fn inset_x_any(p: &str) -> String {
    format!("inset-x-[{}]", p)
}

pub const inset_y_auto: &'static str = "inset-y-auto";
pub const inset_y_0: &'static str = "inset-y-0";
pub const inset_y_px: &'static str = "inset-y-px";
pub const inset_y_half: &'static str = "inset-y-0.5";
pub const inset_y_1: &'static str = "inset-y-1";
pub const inset_y_1_half: &'static str = "inset-y-1.5";
pub const inset_y_2: &'static str = "inset-y-2";
pub const inset_y_2_half: &'static str = "inset-y-2.5";
pub const inset_y_3: &'static str = "inset-y-3";
pub const inset_y_3_half: &'static str = "inset-y-3.5";
pub const inset_y_4: &'static str = "inset-y-4";
pub const inset_y_5: &'static str = "inset-y-5";
pub const inset_y_6: &'static str = "inset-y-6";
pub const inset_y_7: &'static str = "inset-y-7";
pub const inset_y_8: &'static str = "inset-y-8";
pub const inset_y_9: &'static str = "inset-y-9";
pub const inset_y_10: &'static str = "inset-y-10";
pub const inset_y_11: &'static str = "inset-y-11";
pub const inset_y_12: &'static str = "inset-y-12";
pub const inset_y_16: &'static str = "inset-y-16";
pub const inset_y_20: &'static str = "inset-y-20";
pub const inset_y_24: &'static str = "inset-y-24";
pub const inset_y_28: &'static str = "inset-y-28";
pub const inset_y_32: &'static str = "inset-y-32";
pub const inset_y_36: &'static str = "inset-y-36";
pub const inset_y_40: &'static str = "inset-y-40";
pub const inset_y_44: &'static str = "inset-y-44";
pub const inset_y_48: &'static str = "inset-y-48";
pub const inset_y_52: &'static str = "inset-y-52";
pub const inset_y_56: &'static str = "inset-y-56";
pub const inset_y_60: &'static str = "inset-y-60";
pub const inset_y_64: &'static str = "inset-y-64";
pub const inset_y_72: &'static str = "inset-y-72";
pub const inset_y_80: &'static str = "inset-y-80";
pub const inset_y_96: &'static str = "inset-y-96";
pub const inset_y_1_2: &'static str = "inset-y-1/2";
pub const inset_y_1_3: &'static str = "inset-y-1/3";
pub const inset_y_2_3: &'static str = "inset-y-2/3";
pub const inset_y_1_4: &'static str = "inset-y-1/4";
pub const inset_y_2_4: &'static str = "inset-y-2/4";
pub const inset_y_3_4: &'static str = "inset-y-3/4";
pub const inset_y_full: &'static str = "inset-y-full";

pub fn inset_y_any(p: &str) -> String {
    format!("inset-y-[{}]", p)
}

// https://tailwindcss.com/docs/visibility
pub const visible: &'static str = "visible";
pub const invisible: &'static str = "invisible";
pub const collapse: &'static str = "collapse";

// https://tailwindcss.com/docs/z-index
pub const z_0: &'static str = "z-0";
pub const z_10: &'static str = "z-10";
pub const z_20: &'static str = "z-20";
pub const z_30: &'static str = "z-30";
pub const z_40: &'static str = "z-40";
pub const z_50: &'static str = "z-50";
pub const z_auto: &'static str = "z-auto";

pub fn z_any(p: &str) -> String {
    format!("z-[{}]", p)
}

pub const minus_z_10: &'static str = "-z-10";
pub const minus_z_20: &'static str = "-z-20";
pub const minus_z_30: &'static str = "-z-30";
pub const minus_z_40: &'static str = "-z-40";
pub const minus_z_50: &'static str = "-z-50";
