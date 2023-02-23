// https://tailwindcss.com/docs/flex-basis
pub const basis_auto: &'static str = "basis-auto";
pub const basis_0: &'static str = "basis-0";
pub const basis_px: &'static str = "basis-px";
pub const basis_half: &'static str = "basis-0.5";
pub const basis_1: &'static str = "basis-1";
pub const basis_1_half: &'static str = "basis-1.5";
pub const basis_2: &'static str = "basis-2";
pub const basis_2_half: &'static str = "basis-2.5";
pub const basis_3: &'static str = "basis-3";
pub const basis_3_half: &'static str = "basis-3.5";
pub const basis_4: &'static str = "basis-4";
pub const basis_5: &'static str = "basis-5";
pub const basis_6: &'static str = "basis-6";
pub const basis_7: &'static str = "basis-7";
pub const basis_8: &'static str = "basis-8";
pub const basis_9: &'static str = "basis-9";
pub const basis_10: &'static str = "basis-10";
pub const basis_11: &'static str = "basis-11";
pub const basis_12: &'static str = "basis-12";
pub const basis_16: &'static str = "basis-16";
pub const basis_20: &'static str = "basis-20";
pub const basis_24: &'static str = "basis-24";
pub const basis_28: &'static str = "basis-28";
pub const basis_32: &'static str = "basis-32";
pub const basis_36: &'static str = "basis-36";
pub const basis_40: &'static str = "basis-40";
pub const basis_44: &'static str = "basis-44";
pub const basis_48: &'static str = "basis-48";
pub const basis_52: &'static str = "basis-52";
pub const basis_56: &'static str = "basis-56";
pub const basis_60: &'static str = "basis-60";
pub const basis_64: &'static str = "basis-64";
pub const basis_72: &'static str = "basis-72";
pub const basis_80: &'static str = "basis-80";
pub const basis_96: &'static str = "basis-96";
pub const basis_1_2: &'static str = "basis-1/2";
pub const basis_1_3: &'static str = "basis-1/3";
pub const basis_2_3: &'static str = "basis-2/3";
pub const basis_1_4: &'static str = "basis-1/4";
pub const basis_2_4: &'static str = "basis-2/4";
pub const basis_3_4: &'static str = "basis-3/4";
pub const basis_1_5: &'static str = "basis-1/5";
pub const basis_2_5: &'static str = "basis-2/5";
pub const basis_3_5: &'static str = "basis-3/5";
pub const basis_4_5: &'static str = "basis-4/5";
pub const basis_1_6: &'static str = "basis-1/6";
pub const basis_2_6: &'static str = "basis-2/6";
pub const basis_3_6: &'static str = "basis-3/6";
pub const basis_4_6: &'static str = "basis-4/6";
pub const basis_5_6: &'static str = "basis-5/6";
pub const basis_1_12: &'static str = "basis-1/12";
pub const basis_2_12: &'static str = "basis-2/12";
pub const basis_3_12: &'static str = "basis-3/12";
pub const basis_4_12: &'static str = "basis-4/12";
pub const basis_5_12: &'static str = "basis-5/12";
pub const basis_6_12: &'static str = "basis-6/12";
pub const basis_7_12: &'static str = "basis-7/12";
pub const basis_8_12: &'static str = "basis-8/12";
pub const basis_9_12: &'static str = "basis-9/12";
pub const basis_10_12: &'static str = "basis-10/12";
pub const basis_11_12: &'static str = "basis-11/12";
pub const basis_full: &'static str = "basis-full";

pub fn basis_any(p: &str) -> String {
    format!("basis-[{}]", p)
}

// https://tailwindcss.com/docs/flex-direction
pub const flex_row: &'static str = "flex-row";
pub const flex_row_reverse: &'static str = "flex-row-reverse";
pub const flex_col: &'static str = "flex-col";
pub const flex_col_reverse: &'static str = "flex-col-reverse";

// https://tailwindcss.com/docs/flex-wrap
pub const flex_wrap: &'static str = "flex-wrap";
pub const flex_wrap_reverse: &'static str = "flex-wrap-reverse";
pub const flex_nowrap: &'static str = "flex-nowrap";

// https://tailwindcss.com/docs/flex
pub const flex_1: &'static str = "flex-1";
pub const flex_auto: &'static str = "flex-auto";
pub const flex_initial: &'static str = "flex-initial";
pub const flex_none: &'static str = "flex-none";

pub fn flex_any(p: &str) -> String {
    format!("flex-[{}]", p)
}

// https://tailwindcss.com/docs/flex-grow
pub const grow: &'static str = "grow";
pub const grow_0: &'static str = "grow-0";

pub fn grow_any(p: &str) -> String {
    format!("grow-[{}]", p)
}

// https://tailwindcss.com/docs/flex-shrink
pub const shrink: &'static str = "shrink";
pub const shrink_0: &'static str = "shrink-0";

pub fn shrink_any(p: &str) -> String {
    format!("shrink-[{}]", p)
}

// https://tailwindcss.com/docs/order
pub const order_1: &'static str = "order-1";
pub const order_2: &'static str = "order-2";
pub const order_3: &'static str = "order-3";
pub const order_4: &'static str = "order-4";
pub const order_5: &'static str = "order-5";
pub const order_6: &'static str = "order-6";
pub const order_7: &'static str = "order-7";
pub const order_8: &'static str = "order-8";
pub const order_9: &'static str = "order-9";
pub const order_10: &'static str = "order-10";
pub const order_11: &'static str = "order-11";
pub const order_12: &'static str = "order-12";
pub const order_first: &'static str = "order-first";
pub const order_last: &'static str = "order-last";
pub const order_none: &'static str = "order-none";

pub fn order_any(p: &str) -> String {
    format!("order-[{}]", p)
}

// https://tailwindcss.com/docs/grid-template-columns
pub const grid_cols_1: &'static str = "grid-cols-1";
pub const grid_cols_2: &'static str = "grid-cols-2";
pub const grid_cols_3: &'static str = "grid-cols-3";
pub const grid_cols_4: &'static str = "grid-cols-4";
pub const grid_cols_5: &'static str = "grid-cols-5";
pub const grid_cols_6: &'static str = "grid-cols-6";
pub const grid_cols_7: &'static str = "grid-cols-7";
pub const grid_cols_8: &'static str = "grid-cols-8";
pub const grid_cols_9: &'static str = "grid-cols-9";
pub const grid_cols_10: &'static str = "grid-cols-10";
pub const grid_cols_11: &'static str = "grid-cols-11";
pub const grid_cols_12: &'static str = "grid-cols-12";
pub const grid_cols_none: &'static str = "grid-cols-none";

pub fn grid_cols_any(p: &str) -> String {
    format!("grid-cols-[{}]", p)
}

// https://tailwindcss.com/docs/grid-column
pub const col_auto: &'static str = "col-auto";

pub const col_span_1: &'static str = "col-span-1";
pub const col_span_2: &'static str = "col-span-2";
pub const col_span_3: &'static str = "col-span-3";
pub const col_span_4: &'static str = "col-span-4";
pub const col_span_5: &'static str = "col-span-5";
pub const col_span_6: &'static str = "col-span-6";
pub const col_span_7: &'static str = "col-span-7";
pub const col_span_8: &'static str = "col-span-8";
pub const col_span_9: &'static str = "col-span-9";
pub const col_span_10: &'static str = "col-span-10";
pub const col_span_11: &'static str = "col-span-11";
pub const col_span_12: &'static str = "col-span-12";
pub const col_span_full: &'static str = "col-span-full";

pub const col_start_1: &'static str = "col-start-1";
pub const col_start_2: &'static str = "col-start-2";
pub const col_start_3: &'static str = "col-start-3";
pub const col_start_4: &'static str = "col-start-4";
pub const col_start_5: &'static str = "col-start-5";
pub const col_start_6: &'static str = "col-start-6";
pub const col_start_7: &'static str = "col-start-7";
pub const col_start_8: &'static str = "col-start-8";
pub const col_start_9: &'static str = "col-start-9";
pub const col_start_10: &'static str = "col-start-10";
pub const col_start_11: &'static str = "col-start-11";
pub const col_start_12: &'static str = "col-start-12";
pub const col_start_auto: &'static str = "col-start-auto";

pub const col_end_1: &'static str = "col-end-1";
pub const col_end_2: &'static str = "col-end-2";
pub const col_end_3: &'static str = "col-end-3";
pub const col_end_4: &'static str = "col-end-4";
pub const col_end_5: &'static str = "col-end-5";
pub const col_end_6: &'static str = "col-end-6";
pub const col_end_7: &'static str = "col-end-7";
pub const col_end_8: &'static str = "col-end-8";
pub const col_end_9: &'static str = "col-end-9";
pub const col_end_10: &'static str = "col-end-10";
pub const col_end_11: &'static str = "col-end-11";
pub const col_end_12: &'static str = "col-end-12";
pub const col_end_auto: &'static str = "col-end-auto";

pub fn col_any(p: &str) -> String {
    format!("col-[{}]", p)
}

pub fn col_start_any(p: &str) -> String {
    format!("col-start-[{}]", p)
}

pub fn col_end_any(p: &str) -> String {
    format!("col-end-[{}]", p)
}

// https://tailwindcss.com/docs/grid-template-rows
pub const grid_rows_1: &'static str = "grid-rows-1";
pub const grid_rows_2: &'static str = "grid-rows-2";
pub const grid_rows_3: &'static str = "grid-rows-3";
pub const grid_rows_4: &'static str = "grid-rows-4";
pub const grid_rows_5: &'static str = "grid-rows-5";
pub const grid_rows_6: &'static str = "grid-rows-6";
pub const grid_rows_none: &'static str = "grid-rows-none";

pub fn grid_rows_any(p: &str) -> String {
    format!("grid-rows-[{}]", p)
}

// https://tailwindcss.com/docs/grid-row
pub const row_auto: &'static str = "row-auto";
pub const row_span_1: &'static str = "row-span-1";
pub const row_span_2: &'static str = "row-span-2";
pub const row_span_3: &'static str = "row-span-3";
pub const row_span_4: &'static str = "row-span-4";
pub const row_span_5: &'static str = "row-span-5";
pub const row_span_6: &'static str = "row-span-6";
pub const row_span_full: &'static str = "row-span-full";

pub const row_start_1: &'static str = "row-start-1";
pub const row_start_2: &'static str = "row-start-2";
pub const row_start_3: &'static str = "row-start-3";
pub const row_start_4: &'static str = "row-start-4";
pub const row_start_5: &'static str = "row-start-5";
pub const row_start_6: &'static str = "row-start-6";
pub const row_start_7: &'static str = "row-start-7";
pub const row_start_auto: &'static str = "row-start-auto";


pub const row_end_1: &'static str = "row-end-1";
pub const row_end_2: &'static str = "row-end-2";
pub const row_end_3: &'static str = "row-end-3";
pub const row_end_4: &'static str = "row-end-4";
pub const row_end_5: &'static str = "row-end-5";
pub const row_end_6: &'static str = "row-end-6";
pub const row_end_7: &'static str = "row-end-7";
pub const row_end_auto: &'static str = "row-end-auto";

pub fn row_any(p: &str) -> String {
    format!("row-[{}]", p)
}

pub fn row_start_any(p: &str) -> String {
    format!("row-start-[{}]", p)
}

pub fn row_end_any(p: &str) -> String {
    format!("row-end-[{}]", p)
}

// https://tailwindcss.com/docs/grid-auto-flow
pub const grid_flow_row: &'static str = "grid-flow-row";
pub const grid_flow_col: &'static str = "grid-flow-col";
pub const grid_flow_dense: &'static str = "grid-flow-dense";
pub const grid_flow_row_dense: &'static str = "grid-flow-row-dense";
pub const grid_flow_col_dense: &'static str = "grid-flow-col-dense";

// https://tailwindcss.com/docs/grid-auto-columns
pub const auto_cols_auto: &'static str = "auto-cols-auto";
pub const auto_cols_min: &'static str = "auto-cols-min";
pub const auto_cols_max: &'static str = "auto-cols-max";
pub const auto_cols_fr: &'static str = "auto-cols-fr";

pub fn auto_cols_any(p: &str) -> String {
    format!("auto-cols-[{}]", p)
}

// https://tailwindcss.com/docs/grid-auto-rows
pub const auto_rows_auto: &'static str = "auto-rows-auto";
pub const auto_rows_min: &'static str = "auto-rows-min";
pub const auto_rows_max: &'static str = "auto-rows-max";
pub const auto_rows_fr: &'static str = "auto-rows-fr";

pub fn auto_rows_any(p: &str) -> String {
    format!("auto-rows-[{}]", p)
}

// https://tailwindcss.com/docs/gap
pub const gap_0: &'static str = "gap-0";
pub const gap_px: &'static str = "gap-px";
pub const gap_half: &'static str = "gap-0.5";
pub const gap_1: &'static str = "gap-1";
pub const gap_1_half: &'static str = "gap-1.5";
pub const gap_2: &'static str = "gap-2";
pub const gap_2_half: &'static str = "gap-2.5";
pub const gap_3: &'static str = "gap-3";
pub const gap_3_half: &'static str = "gap-3.5";
pub const gap_4: &'static str = "gap-4";
pub const gap_5: &'static str = "gap-5";
pub const gap_6: &'static str = "gap-6";
pub const gap_7: &'static str = "gap-7";
pub const gap_8: &'static str = "gap-8";
pub const gap_9: &'static str = "gap-9";
pub const gap_10: &'static str = "gap-10";
pub const gap_11: &'static str = "gap-11";
pub const gap_12: &'static str = "gap-12";
pub const gap_16: &'static str = "gap-16";
pub const gap_20: &'static str = "gap-20";
pub const gap_24: &'static str = "gap-24";
pub const gap_28: &'static str = "gap-28";
pub const gap_32: &'static str = "gap-32";
pub const gap_36: &'static str = "gap-36";
pub const gap_40: &'static str = "gap-40";
pub const gap_44: &'static str = "gap-44";
pub const gap_48: &'static str = "gap-48";
pub const gap_52: &'static str = "gap-52";
pub const gap_56: &'static str = "gap-56";
pub const gap_60: &'static str = "gap-60";
pub const gap_64: &'static str = "gap-64";
pub const gap_72: &'static str = "gap-72";
pub const gap_80: &'static str = "gap-80";
pub const gap_96: &'static str = "gap-96";

pub fn gap_any(p: &str) -> String {
    format!("gap-[{}]", p)
}

pub const gap_x_0: &'static str = "gap-x-0";
pub const gap_x_px: &'static str = "gap-x-px";
pub const gap_x_half: &'static str = "gap-x-0.5";
pub const gap_x_1: &'static str = "gap-x-1";
pub const gap_x_1_half: &'static str = "gap-x-1.5";
pub const gap_x_2: &'static str = "gap-x-2";
pub const gap_x_2_half: &'static str = "gap-x-2.5";
pub const gap_x_3: &'static str = "gap-x-3";
pub const gap_x_3_half: &'static str = "gap-x-3.5";
pub const gap_x_4: &'static str = "gap-x-4";
pub const gap_x_5: &'static str = "gap-x-5";
pub const gap_x_6: &'static str = "gap-x-6";
pub const gap_x_7: &'static str = "gap-x-7";
pub const gap_x_8: &'static str = "gap-x-8";
pub const gap_x_9: &'static str = "gap-x-9";
pub const gap_x_10: &'static str = "gap-x-10";
pub const gap_x_11: &'static str = "gap-x-11";
pub const gap_x_12: &'static str = "gap-x-12";
pub const gap_x_16: &'static str = "gap-x-16";
pub const gap_x_20: &'static str = "gap-x-20";
pub const gap_x_24: &'static str = "gap-x-24";
pub const gap_x_28: &'static str = "gap-x-28";
pub const gap_x_32: &'static str = "gap-x-32";
pub const gap_x_36: &'static str = "gap-x-36";
pub const gap_x_40: &'static str = "gap-x-40";
pub const gap_x_44: &'static str = "gap-x-44";
pub const gap_x_48: &'static str = "gap-x-48";
pub const gap_x_52: &'static str = "gap-x-52";
pub const gap_x_56: &'static str = "gap-x-56";
pub const gap_x_60: &'static str = "gap-x-60";
pub const gap_x_64: &'static str = "gap-x-64";
pub const gap_x_72: &'static str = "gap-x-72";
pub const gap_x_80: &'static str = "gap-x-80";
pub const gap_x_96: &'static str = "gap-x-96";

pub fn gap_x_any(p: &str) -> String {
    format!("gap-x-[{}]", p)
}

pub const gap_y_0: &'static str = "gap-y-0";
pub const gap_y_px: &'static str = "gap-y-px";
pub const gap_y_half: &'static str = "gap-y-0.5";
pub const gap_y_1: &'static str = "gap-y-1";
pub const gap_y_1_half: &'static str = "gap-y-1.5";
pub const gap_y_2: &'static str = "gap-y-2";
pub const gap_y_2_half: &'static str = "gap-y-2.5";
pub const gap_y_3: &'static str = "gap-y-3";
pub const gap_y_3_half: &'static str = "gap-y-3.5";
pub const gap_y_4: &'static str = "gap-y-4";
pub const gap_y_5: &'static str = "gap-y-5";
pub const gap_y_6: &'static str = "gap-y-6";
pub const gap_y_7: &'static str = "gap-y-7";
pub const gap_y_8: &'static str = "gap-y-8";
pub const gap_y_9: &'static str = "gap-y-9";
pub const gap_y_10: &'static str = "gap-y-10";
pub const gap_y_11: &'static str = "gap-y-11";
pub const gap_y_12: &'static str = "gap-y-12";
pub const gap_y_16: &'static str = "gap-y-16";
pub const gap_y_20: &'static str = "gap-y-20";
pub const gap_y_24: &'static str = "gap-y-24";
pub const gap_y_28: &'static str = "gap-y-28";
pub const gap_y_32: &'static str = "gap-y-32";
pub const gap_y_36: &'static str = "gap-y-36";
pub const gap_y_40: &'static str = "gap-y-40";
pub const gap_y_44: &'static str = "gap-y-44";
pub const gap_y_48: &'static str = "gap-y-48";
pub const gap_y_52: &'static str = "gap-y-52";
pub const gap_y_56: &'static str = "gap-y-56";
pub const gap_y_60: &'static str = "gap-y-60";
pub const gap_y_64: &'static str = "gap-y-64";
pub const gap_y_72: &'static str = "gap-y-72";
pub const gap_y_80: &'static str = "gap-y-80";
pub const gap_y_96: &'static str = "gap-y-96";

pub fn gap_y_any(p: &str) -> String {
    format!("gap-y-[{}]", p)
}

// https://tailwindcss.com/docs/justify-content
pub const justify_start: &'static str = "justify-start";
pub const justify_end: &'static str = "justify-end";
pub const justify_center: &'static str = "justify-center";
pub const justify_between: &'static str = "justify-between";
pub const justify_around: &'static str = "justify-around";
pub const justify_evenly: &'static str = "justify-evenly";

// https://tailwindcss.com/docs/justify-items
pub const justify_items_start: &'static str = "justify-items-start";
pub const justify_items_end: &'static str = "justify-items-end";
pub const justify_items_center: &'static str = "justify-items-center";
pub const justify_items_stretch: &'static str = "justify-items-stretch";

// https://tailwindcss.com/docs/justify-self
pub const justify_self_auto: &'static str = "justify-self-auto";
pub const justify_self_start: &'static str = "justify-self-start";
pub const justify_self_end: &'static str = "justify-self-end";
pub const justify_self_center: &'static str = "justify-self-center";
pub const justify_self_stretch: &'static str = "justify-self-stretch";

// https://tailwindcss.com/docs/align-content
pub const content_start: &'static str = "content-start";
pub const content_end: &'static str = "content-end";
pub const content_center: &'static str = "content-center";
pub const content_between: &'static str = "content-between";
pub const content_around: &'static str = "content-around";
pub const content_evenly: &'static str = "content-evenly";
pub const content_baseline: &'static str = "content-baseline";

// https://tailwindcss.com/docs/align-items
pub const items_start: &'static str = "items-start";
pub const items_end: &'static str = "items-end";
pub const items_center: &'static str = "items-center";
pub const items_baseline: &'static str = "items-baseline";
pub const items_stretch: &'static str = "items-stretch";

// https://tailwindcss.com/docs/align-self
pub const self_auto: &'static str = "self-auto";
pub const self_start: &'static str = "self-start";
pub const self_end: &'static str = "self-end";
pub const self_center: &'static str = "self-center";
pub const self_baseline: &'static str = "self-baseline";
pub const self_stretch: &'static str = "self-stretch";

// https://tailwindcss.com/docs/place-content
pub const place_content_start: &'static str = "place-content-start";
pub const place_content_end: &'static str = "place-content-end";
pub const place_content_center: &'static str = "place-content-center";
pub const place_content_between: &'static str = "place-content-between";
pub const place_content_around: &'static str = "place-content-around";
pub const place_content_evenly: &'static str = "place-content-evenly";
pub const place_content_baseline: &'static str = "place-content-baseline";
pub const place_content_stretch: &'static str = "place-content-stretch";

// https://tailwindcss.com/docs/place-items
pub const place_items_start: &'static str = "place-items-start";
pub const place_items_end: &'static str = "place-items-end";
pub const place_items_center: &'static str = "place-items-center";
pub const place_items_baseline: &'static str = "place-items-baseline";
pub const place_items_stretch: &'static str = "place-items-stretch";

// https://tailwindcss.com/docs/place-self
pub const place_self_auto: &'static str = "place-self-auto";
pub const place_self_start: &'static str = "place-self-start";
pub const place_self_end: &'static str = "place-self-end";
pub const place_self_center: &'static str = "place-self-center";
pub const place_self_stretch: &'static str = "place-self-stretch";

