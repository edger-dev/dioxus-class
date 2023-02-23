// https://tailwindcss.com/docs/width
pub const w_0: &'static str = "w-0";
pub const w_px: &'static str = "w-px";
pub const w_half: &'static str = "w-0.5";
pub const w_1: &'static str = "w-1";
pub const w_1_half: &'static str = "w-1.5";
pub const w_2: &'static str = "w-2";
pub const w_2_half: &'static str = "w-2.5";
pub const w_3: &'static str = "w-3";
pub const w_3_half: &'static str = "w-3.5";
pub const w_4: &'static str = "w-4";
pub const w_5: &'static str = "w-5";
pub const w_6: &'static str = "w-6";
pub const w_7: &'static str = "w-7";
pub const w_8: &'static str = "w-8";
pub const w_9: &'static str = "w-9";
pub const w_10: &'static str = "w-10";
pub const w_11: &'static str = "w-11";
pub const w_12: &'static str = "w-12";
pub const w_16: &'static str = "w-16";
pub const w_20: &'static str = "w-20";
pub const w_24: &'static str = "w-24";
pub const w_28: &'static str = "w-28";
pub const w_32: &'static str = "w-32";
pub const w_36: &'static str = "w-36";
pub const w_40: &'static str = "w-40";
pub const w_44: &'static str = "w-44";
pub const w_48: &'static str = "w-48";
pub const w_52: &'static str = "w-52";
pub const w_56: &'static str = "w-56";
pub const w_60: &'static str = "w-60";
pub const w_64: &'static str = "w-64";
pub const w_72: &'static str = "w-72";
pub const w_80: &'static str = "w-80";
pub const w_96: &'static str = "w-96";
pub const w_1_2: &'static str = "w-1/2";
pub const w_1_3: &'static str = "w-1/3";
pub const w_2_3: &'static str = "w-2/3";
pub const w_1_4: &'static str = "w-1/4";
pub const w_2_4: &'static str = "w-2/4";
pub const w_3_4: &'static str = "w-3/4";
pub const w_1_5: &'static str = "w-1/5";
pub const w_2_5: &'static str = "w-2/5";
pub const w_3_5: &'static str = "w-3/5";
pub const w_4_5: &'static str = "w-4/5";
pub const w_1_6: &'static str = "w-1/6";
pub const w_2_6: &'static str = "w-2/6";
pub const w_3_6: &'static str = "w-3/6";
pub const w_4_6: &'static str = "w-4/6";
pub const w_5_6: &'static str = "w-5/6";
pub const w_1_12: &'static str = "w-1/12";
pub const w_2_12: &'static str = "w-2/12";
pub const w_3_12: &'static str = "w-3/12";
pub const w_4_12: &'static str = "w-4/12";
pub const w_5_12: &'static str = "w-5/12";
pub const w_6_12: &'static str = "w-6/12";
pub const w_7_12: &'static str = "w-7/12";
pub const w_8_12: &'static str = "w-8/12";
pub const w_9_12: &'static str = "w-9/12";
pub const w_10_12: &'static str = "w-10/12";
pub const w_11_12: &'static str = "w-11/12";
pub const w_full: &'static str = "w-full";
pub const w_screen: &'static str = "w-screen";
pub const w_min: &'static str = "w-min";
pub const w_max: &'static str = "w-max";
pub const w_fit: &'static str = "w-fit";

pub fn w_any(p: &str) -> String {
    format!("w-[{}]", p)
}

// https://tailwindcss.com/docs/min-width
pub const min_w_full: &'static str = "min-w-full";
pub const min_w_min: &'static str = "min-w-min";
pub const min_w_max: &'static str = "min-w-max";
pub const min_w_fit: &'static str = "min-w-fit";

pub fn min_w_any(p: &str) -> String {
    format!("min-w-[{}]", p)
}

pub const max_w_0: &'static str = "max-w-0";
pub const max_w_none: &'static str = "max-w-none";
pub const max_w_xs: &'static str = "max-w-xs";
pub const max_w_sm: &'static str = "max-w-sm";
pub const max_w_md: &'static str = "max-w-md";
pub const max_w_lg: &'static str = "max-w-lg";
pub const max_w_xl: &'static str = "max-w-xl";
pub const max_w_2xl: &'static str = "max-w-2xl";
pub const max_w_3xl: &'static str = "max-w-3xl";
pub const max_w_4xl: &'static str = "max-w-4xl";
pub const max_w_5xl: &'static str = "max-w-5xl";
pub const max_w_6xl: &'static str = "max-w-6xl";
pub const max_w_7xl: &'static str = "max-w-7xl";
pub const max_w_full: &'static str = "max-w-full";
pub const max_w_min: &'static str = "max-w-min";
pub const max_w_max: &'static str = "max-w-max";
pub const max_w_fix: &'static str = "max-w-fix";
pub const max_w_prose: &'static str = "max-w-prose";
pub const max_w_screen_sm: &'static str = "max-w-screen-sm";
pub const max_w_screen_md: &'static str = "max-w-screen-md";
pub const max_w_screen_lg: &'static str = "max-w-screen-lg";
pub const max_w_screen_xl: &'static str = "max-w-screen-xl";
pub const max_w_screen_2xl: &'static str = "max-w-screen-2xl";

pub fn max_w_any(p: &str) -> String {
    format!("max-w-[{}]", p)
}

// https://tailwindcss.com/docs/height
pub const h_0: &'static str = "h-0";
pub const h_px: &'static str = "h-px";
pub const h_half: &'static str = "h-0.5";
pub const h_1: &'static str = "h-1";
pub const h_1_half: &'static str = "h-1.5";
pub const h_2: &'static str = "h-2";
pub const h_2_half: &'static str = "h-2.5";
pub const h_3: &'static str = "h-3";
pub const h_3_half: &'static str = "h-3.5";
pub const h_4: &'static str = "h-4";
pub const h_5: &'static str = "h-5";
pub const h_6: &'static str = "h-6";
pub const h_7: &'static str = "h-7";
pub const h_8: &'static str = "h-8";
pub const h_9: &'static str = "h-9";
pub const h_10: &'static str = "h-10";
pub const h_11: &'static str = "h-11";
pub const h_12: &'static str = "h-12";
pub const h_16: &'static str = "h-16";
pub const h_20: &'static str = "h-20";
pub const h_24: &'static str = "h-24";
pub const h_28: &'static str = "h-28";
pub const h_32: &'static str = "h-32";
pub const h_36: &'static str = "h-36";
pub const h_40: &'static str = "h-40";
pub const h_44: &'static str = "h-44";
pub const h_48: &'static str = "h-48";
pub const h_52: &'static str = "h-52";
pub const h_56: &'static str = "h-56";
pub const h_60: &'static str = "h-60";
pub const h_64: &'static str = "h-64";
pub const h_72: &'static str = "h-72";
pub const h_80: &'static str = "h-80";
pub const h_96: &'static str = "h-96";
pub const h_1_2: &'static str = "h-1/2";
pub const h_1_3: &'static str = "h-1/3";
pub const h_2_3: &'static str = "h-2/3";
pub const h_1_4: &'static str = "h-1/4";
pub const h_2_4: &'static str = "h-2/4";
pub const h_3_4: &'static str = "h-3/4";
pub const h_1_5: &'static str = "h-1/5";
pub const h_2_5: &'static str = "h-2/5";
pub const h_3_5: &'static str = "h-3/5";
pub const h_4_5: &'static str = "h-4/5";
pub const h_1_6: &'static str = "h-1/6";
pub const h_2_6: &'static str = "h-2/6";
pub const h_3_6: &'static str = "h-3/6";
pub const h_4_6: &'static str = "h-4/6";
pub const h_5_6: &'static str = "h-5/6";
pub const h_auto: &'static str = "h-auto";
pub const h_full: &'static str = "h-full";
pub const h_screen: &'static str = "h-screen";
pub const h_min: &'static str = "h-min";
pub const h_max: &'static str = "h-max";
pub const h_fit: &'static str = "h-fit";

pub fn h_any(p: &str) -> String {
    format!("h-[{}]", p)
}

// https://tailwindcss.com/docs/min-height
pub const min_h_0: &'static str = "min-h-0";
pub const min_h_full: &'static str = "min-h-full";
pub const min_h_screen: &'static str = "min-h-screen";
pub const min_h_min: &'static str = "min-h-min";
pub const min_h_max: &'static str = "min-h-max";
pub const min_h_fit: &'static str = "min-h-fit";

pub fn min_h_any(p: &str) -> String {
    format!("min-h-[{}]", p)
}

pub const max_h_0: &'static str = "max-h-0";
pub const max_h_px: &'static str = "max-h-px";
pub const max_h_half: &'static str = "max-h-0.5";
pub const max_h_1: &'static str = "max-h-1";
pub const max_h_1_half: &'static str = "max-h-1.5";
pub const max_h_2: &'static str = "max-h-2";
pub const max_h_2_half: &'static str = "max-h-2.5";
pub const max_h_3: &'static str = "max-h-3";
pub const max_h_3_half: &'static str = "max-h-3.5";
pub const max_h_4: &'static str = "max-h-4";
pub const max_h_5: &'static str = "max-h-5";
pub const max_h_6: &'static str = "max-h-6";
pub const max_h_7: &'static str = "max-h-7";
pub const max_h_8: &'static str = "max-h-8";
pub const max_h_9: &'static str = "max-h-9";
pub const max_h_10: &'static str = "max-h-10";
pub const max_h_11: &'static str = "max-h-11";
pub const max_h_12: &'static str = "max-h-12";
pub const max_h_16: &'static str = "max-h-16";
pub const max_h_20: &'static str = "max-h-20";
pub const max_h_24: &'static str = "max-h-24";
pub const max_h_28: &'static str = "max-h-28";
pub const max_h_32: &'static str = "max-h-32";
pub const max_h_36: &'static str = "max-h-36";
pub const max_h_40: &'static str = "max-h-40";
pub const max_h_44: &'static str = "max-h-44";
pub const max_h_48: &'static str = "max-h-48";
pub const max_h_52: &'static str = "max-h-52";
pub const max_h_56: &'static str = "max-h-56";
pub const max_h_60: &'static str = "max-h-60";
pub const max_h_64: &'static str = "max-h-64";
pub const max_h_72: &'static str = "max-h-72";
pub const max_h_80: &'static str = "max-h-80";
pub const max_h_96: &'static str = "max-h-96";
pub const max_h_none: &'static str = "max-h-none";
pub const max_h_full: &'static str = "max-h-full";
pub const max_h_screen: &'static str = "max-h-screen";
pub const max_h_min: &'static str = "max-h-min";
pub const max_h_max: &'static str = "max-h-max";
pub const max_h_fit: &'static str = "max-h-fit";

pub fn max_h_any(p: &str) -> String {
    format!("max-h-[{}]", p)
}