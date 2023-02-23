// https://tailwindcss.com/docs/font-family
pub const font_sans: &'static str = "font-sans";
pub const font_serif: &'static str = "font-serif";
pub const font_mono: &'static str = "font-mono";

pub fn font_any(p: &str) -> String {
    format!("font-[{}]", p)
}

// https://tailwindcss.com/docs/font-size
pub const text_xs: &'static str = "text-xs";
pub const text_sm: &'static str = "text-sm";
pub const text_base: &'static str = "text-base";
pub const text_lg: &'static str = "text-lg";
pub const text_xl: &'static str = "text-xl";
pub const text_2xl: &'static str = "text-2xl";
pub const text_3xl: &'static str = "text-3xl";
pub const text_4xl: &'static str = "text-4xl";
pub const text_5xl: &'static str = "text-5xl";
pub const text_6xl: &'static str = "text-6xl";
pub const text_7xl: &'static str = "text-7xl";
pub const text_8xl: &'static str = "text-8xl";
pub const text_9xl: &'static str = "text-9xl";

// https://tailwindcss.com/docs/font-smoothing
pub const antialiased: &'static str = "antialiased";
pub const subpixel_antialiased: &'static str = "subpixel-antialiased";

// https://tailwindcss.com/docs/font-style
pub const italic: &'static str = "italic";
pub const not_italic: &'static str = "not-italic";

// https://tailwindcss.com/docs/font-weight
pub const font_thin: &'static str = "font-thin";
pub const font_extralight: &'static str = "font-extralight";
pub const font_light: &'static str = "font-light";
pub const font_normal: &'static str = "font-normal";
pub const font_medium: &'static str = "font-medium";
pub const font_semibold: &'static str = "font-semibold";
pub const font_bold: &'static str = "font-bold";
pub const font_extrabold: &'static str = "font-extrabold";
pub const font_black: &'static str = "font-black";

// https://tailwindcss.com/docs/font-variant-numeric
pub const normal_nums: &'static str = "normal-nums";
pub const ordinal: &'static str = "ordinal";
pub const slashed_zero: &'static str = "slashed-zero";
pub const lining_nums: &'static str = "lining-nums";
pub const oldstyle_nums: &'static str = "oldstyle-nums";
pub const proportional_nums: &'static str = "proportional-nums";
pub const tabular_nums: &'static str = "tabular-nums";
pub const diagonal_fractions: &'static str = "diagonal-fractions";
pub const stacked_fractions: &'static str = "stacked-fractions";

// https://tailwindcss.com/docs/letter-spacing
pub const tracking_tighter: &'static str = "tracking-tighter";
pub const tracking_tight: &'static str = "tracking-tight";
pub const tracking_normal: &'static str = "tracking-normal";
pub const tracking_wide: &'static str = "tracking-wide";
pub const tracking_wider: &'static str = "tracking-wider";
pub const tracking_widest: &'static str = "tracking-widest";

pub fn tracking_any(p: &str) -> String {
    format!("tracking-[{}]", p)
}

// https://tailwindcss.com/docs/line-height
pub const leading_3: &'static str = "leading-3";
pub const leading_4: &'static str = "leading-4";
pub const leading_5: &'static str = "leading-5";
pub const leading_6: &'static str = "leading-6";
pub const leading_7: &'static str = "leading-7";
pub const leading_8: &'static str = "leading-8";
pub const leading_9: &'static str = "leading-9";
pub const leading_10: &'static str = "leading-10";
pub const leading_none: &'static str = "leading-none";
pub const leading_tight: &'static str = "leading-tight";
pub const leading_snug: &'static str = "leading-snug";
pub const leading_normal: &'static str = "leading-normal";
pub const leading_relaxed: &'static str = "leading-relaxed";
pub const leading_loose: &'static str = "leading-loose";

pub fn leading_any(p: &str) -> String {
    format!("leading-[{}]", p)
}

// https://tailwindcss.com/docs/list-style-type
pub const list_none: &'static str = "list-none";
pub const list_disc: &'static str = "list-disc";
pub const list_decimal: &'static str = "list-decimal";

pub fn list_any(p: &str) -> String {
    format!("list-[{}]", p)
}

// https://tailwindcss.com/docs/list-style-position
pub const list_inside: &'static str = "list-inside";
pub const list_outside: &'static str = "list-outside";

// https://tailwindcss.com/docs/text-align
pub const text_left: &'static str = "text-left";
pub const text_center: &'static str = "text-center";
pub const text_right: &'static str = "text-right";
pub const text_justify: &'static str = "text-justify";
pub const text_start: &'static str = "text-start";
pub const text_end: &'static str = "text-end";

// https://tailwindcss.com/docs/text-color
pub const text_inherit: &'static str = "text-inherit";
pub const text_current: &'static str = "text-current";
pub const text_transparent: &'static str = "text-transparent";
pub const text_black: &'static str = "text-black";
pub const text_white: &'static str = "text-white";

pub const text_slate_50: &'static str = "text-slate-50";
pub const text_slate_100: &'static str = "text-slate-100";
pub const text_slate_200: &'static str = "text-slate-200";
pub const text_slate_300: &'static str = "text-slate-300";
pub const text_slate_400: &'static str = "text-slate-400";
pub const text_slate_500: &'static str = "text-slate-500";
pub const text_slate_600: &'static str = "text-slate-600";
pub const text_slate_700: &'static str = "text-slate-700";
pub const text_slate_800: &'static str = "text-slate-800";
pub const text_slate_900: &'static str = "text-slate-900";

pub const text_gray_50: &'static str = "text-gray-50";
pub const text_gray_100: &'static str = "text-gray-100";
pub const text_gray_200: &'static str = "text-gray-200";
pub const text_gray_300: &'static str = "text-gray-300";
pub const text_gray_400: &'static str = "text-gray-400";
pub const text_gray_500: &'static str = "text-gray-500";
pub const text_gray_600: &'static str = "text-gray-600";
pub const text_gray_700: &'static str = "text-gray-700";
pub const text_gray_800: &'static str = "text-gray-800";
pub const text_gray_900: &'static str = "text-gray-900";

pub const text_zinc_50: &'static str = "text-zinc-50";
pub const text_zinc_100: &'static str = "text-zinc-100";
pub const text_zinc_200: &'static str = "text-zinc-200";
pub const text_zinc_300: &'static str = "text-zinc-300";
pub const text_zinc_400: &'static str = "text-zinc-400";
pub const text_zinc_500: &'static str = "text-zinc-500";
pub const text_zinc_600: &'static str = "text-zinc-600";
pub const text_zinc_700: &'static str = "text-zinc-700";
pub const text_zinc_800: &'static str = "text-zinc-800";
pub const text_zinc_900: &'static str = "text-zinc-900";

pub const text_neutral_50: &'static str = "text-neutral-50";
pub const text_neutral_100: &'static str = "text-neutral-100";
pub const text_neutral_200: &'static str = "text-neutral-200";
pub const text_neutral_300: &'static str = "text-neutral-300";
pub const text_neutral_400: &'static str = "text-neutral-400";
pub const text_neutral_500: &'static str = "text-neutral-500";
pub const text_neutral_600: &'static str = "text-neutral-600";
pub const text_neutral_700: &'static str = "text-neutral-700";
pub const text_neutral_800: &'static str = "text-neutral-800";
pub const text_neutral_900: &'static str = "text-neutral-900";

pub const text_stone_50: &'static str = "text-stone-50";
pub const text_stone_100: &'static str = "text-stone-100";
pub const text_stone_200: &'static str = "text-stone-200";
pub const text_stone_300: &'static str = "text-stone-300";
pub const text_stone_400: &'static str = "text-stone-400";
pub const text_stone_500: &'static str = "text-stone-500";
pub const text_stone_600: &'static str = "text-stone-600";
pub const text_stone_700: &'static str = "text-stone-700";
pub const text_stone_800: &'static str = "text-stone-800";
pub const text_stone_900: &'static str = "text-stone-900";

pub const text_red_50: &'static str = "text-red-50";
pub const text_red_100: &'static str = "text-red-100";
pub const text_red_200: &'static str = "text-red-200";
pub const text_red_300: &'static str = "text-red-300";
pub const text_red_400: &'static str = "text-red-400";
pub const text_red_500: &'static str = "text-red-500";
pub const text_red_600: &'static str = "text-red-600";
pub const text_red_700: &'static str = "text-red-700";
pub const text_red_800: &'static str = "text-red-800";
pub const text_red_900: &'static str = "text-red-900";

pub const text_orange_50: &'static str = "text-orange-50";
pub const text_orange_100: &'static str = "text-orange-100";
pub const text_orange_200: &'static str = "text-orange-200";
pub const text_orange_300: &'static str = "text-orange-300";
pub const text_orange_400: &'static str = "text-orange-400";
pub const text_orange_500: &'static str = "text-orange-500";
pub const text_orange_600: &'static str = "text-orange-600";
pub const text_orange_700: &'static str = "text-orange-700";
pub const text_orange_800: &'static str = "text-orange-800";
pub const text_orange_900: &'static str = "text-orange-900";

pub const text_amber_50: &'static str = "text-amber-50";
pub const text_amber_100: &'static str = "text-amber-100";
pub const text_amber_200: &'static str = "text-amber-200";
pub const text_amber_300: &'static str = "text-amber-300";
pub const text_amber_400: &'static str = "text-amber-400";
pub const text_amber_500: &'static str = "text-amber-500";
pub const text_amber_600: &'static str = "text-amber-600";
pub const text_amber_700: &'static str = "text-amber-700";
pub const text_amber_800: &'static str = "text-amber-800";
pub const text_amber_900: &'static str = "text-amber-900";

pub const text_yellow_50: &'static str = "text-yellow-50";
pub const text_yellow_100: &'static str = "text-yellow-100";
pub const text_yellow_200: &'static str = "text-yellow-200";
pub const text_yellow_300: &'static str = "text-yellow-300";
pub const text_yellow_400: &'static str = "text-yellow-400";
pub const text_yellow_500: &'static str = "text-yellow-500";
pub const text_yellow_600: &'static str = "text-yellow-600";
pub const text_yellow_700: &'static str = "text-yellow-700";
pub const text_yellow_800: &'static str = "text-yellow-800";
pub const text_yellow_900: &'static str = "text-yellow-900";

pub const text_lime_50: &'static str = "text-lime-50";
pub const text_lime_100: &'static str = "text-lime-100";
pub const text_lime_200: &'static str = "text-lime-200";
pub const text_lime_300: &'static str = "text-lime-300";
pub const text_lime_400: &'static str = "text-lime-400";
pub const text_lime_500: &'static str = "text-lime-500";
pub const text_lime_600: &'static str = "text-lime-600";
pub const text_lime_700: &'static str = "text-lime-700";
pub const text_lime_800: &'static str = "text-lime-800";
pub const text_lime_900: &'static str = "text-lime-900";

pub const text_green_50: &'static str = "text-green-50";
pub const text_green_100: &'static str = "text-green-100";
pub const text_green_200: &'static str = "text-green-200";
pub const text_green_300: &'static str = "text-green-300";
pub const text_green_400: &'static str = "text-green-400";
pub const text_green_500: &'static str = "text-green-500";
pub const text_green_600: &'static str = "text-green-600";
pub const text_green_700: &'static str = "text-green-700";
pub const text_green_800: &'static str = "text-green-800";
pub const text_green_900: &'static str = "text-green-900";

pub const text_emerald_50: &'static str = "text-emerald-50";
pub const text_emerald_100: &'static str = "text-emerald-100";
pub const text_emerald_200: &'static str = "text-emerald-200";
pub const text_emerald_300: &'static str = "text-emerald-300";
pub const text_emerald_400: &'static str = "text-emerald-400";
pub const text_emerald_500: &'static str = "text-emerald-500";
pub const text_emerald_600: &'static str = "text-emerald-600";
pub const text_emerald_700: &'static str = "text-emerald-700";
pub const text_emerald_800: &'static str = "text-emerald-800";
pub const text_emerald_900: &'static str = "text-emerald-900";

pub const text_teal_50: &'static str = "text-teal-50";
pub const text_teal_100: &'static str = "text-teal-100";
pub const text_teal_200: &'static str = "text-teal-200";
pub const text_teal_300: &'static str = "text-teal-300";
pub const text_teal_400: &'static str = "text-teal-400";
pub const text_teal_500: &'static str = "text-teal-500";
pub const text_teal_600: &'static str = "text-teal-600";
pub const text_teal_700: &'static str = "text-teal-700";
pub const text_teal_800: &'static str = "text-teal-800";
pub const text_teal_900: &'static str = "text-teal-900";

pub const text_cyan_50: &'static str = "text-cyan-50";
pub const text_cyan_100: &'static str = "text-cyan-100";
pub const text_cyan_200: &'static str = "text-cyan-200";
pub const text_cyan_300: &'static str = "text-cyan-300";
pub const text_cyan_400: &'static str = "text-cyan-400";
pub const text_cyan_500: &'static str = "text-cyan-500";
pub const text_cyan_600: &'static str = "text-cyan-600";
pub const text_cyan_700: &'static str = "text-cyan-700";
pub const text_cyan_800: &'static str = "text-cyan-800";
pub const text_cyan_900: &'static str = "text-cyan-900";

pub const text_sky_50: &'static str = "text-sky-50";
pub const text_sky_100: &'static str = "text-sky-100";
pub const text_sky_200: &'static str = "text-sky-200";
pub const text_sky_300: &'static str = "text-sky-300";
pub const text_sky_400: &'static str = "text-sky-400";
pub const text_sky_500: &'static str = "text-sky-500";
pub const text_sky_600: &'static str = "text-sky-600";
pub const text_sky_700: &'static str = "text-sky-700";
pub const text_sky_800: &'static str = "text-sky-800";
pub const text_sky_900: &'static str = "text-sky-900";

pub const text_blue_50: &'static str = "text-blue-50";
pub const text_blue_100: &'static str = "text-blue-100";
pub const text_blue_200: &'static str = "text-blue-200";
pub const text_blue_300: &'static str = "text-blue-300";
pub const text_blue_400: &'static str = "text-blue-400";
pub const text_blue_500: &'static str = "text-blue-500";
pub const text_blue_600: &'static str = "text-blue-600";
pub const text_blue_700: &'static str = "text-blue-700";
pub const text_blue_800: &'static str = "text-blue-800";
pub const text_blue_900: &'static str = "text-blue-900";

pub const text_indigo_50: &'static str = "text-indigo-50";
pub const text_indigo_100: &'static str = "text-indigo-100";
pub const text_indigo_200: &'static str = "text-indigo-200";
pub const text_indigo_300: &'static str = "text-indigo-300";
pub const text_indigo_400: &'static str = "text-indigo-400";
pub const text_indigo_500: &'static str = "text-indigo-500";
pub const text_indigo_600: &'static str = "text-indigo-600";
pub const text_indigo_700: &'static str = "text-indigo-700";
pub const text_indigo_800: &'static str = "text-indigo-800";
pub const text_indigo_900: &'static str = "text-indigo-900";

pub const text_violet_50: &'static str = "text-violet-50";
pub const text_violet_100: &'static str = "text-violet-100";
pub const text_violet_200: &'static str = "text-violet-200";
pub const text_violet_300: &'static str = "text-violet-300";
pub const text_violet_400: &'static str = "text-violet-400";
pub const text_violet_500: &'static str = "text-violet-500";
pub const text_violet_600: &'static str = "text-violet-600";
pub const text_violet_700: &'static str = "text-violet-700";
pub const text_violet_800: &'static str = "text-violet-800";
pub const text_violet_900: &'static str = "text-violet-900";

pub const text_purple_50: &'static str = "text-purple-50";
pub const text_purple_100: &'static str = "text-purple-100";
pub const text_purple_200: &'static str = "text-purple-200";
pub const text_purple_300: &'static str = "text-purple-300";
pub const text_purple_400: &'static str = "text-purple-400";
pub const text_purple_500: &'static str = "text-purple-500";
pub const text_purple_600: &'static str = "text-purple-600";
pub const text_purple_700: &'static str = "text-purple-700";
pub const text_purple_800: &'static str = "text-purple-800";
pub const text_purple_900: &'static str = "text-purple-900";

pub const text_fuchsia_50: &'static str = "text-fuchsia-50";
pub const text_fuchsia_100: &'static str = "text-fuchsia-100";
pub const text_fuchsia_200: &'static str = "text-fuchsia-200";
pub const text_fuchsia_300: &'static str = "text-fuchsia-300";
pub const text_fuchsia_400: &'static str = "text-fuchsia-400";
pub const text_fuchsia_500: &'static str = "text-fuchsia-500";
pub const text_fuchsia_600: &'static str = "text-fuchsia-600";
pub const text_fuchsia_700: &'static str = "text-fuchsia-700";
pub const text_fuchsia_800: &'static str = "text-fuchsia-800";
pub const text_fuchsia_900: &'static str = "text-fuchsia-900";

pub const text_pink_50: &'static str = "text-pink-50";
pub const text_pink_100: &'static str = "text-pink-100";
pub const text_pink_200: &'static str = "text-pink-200";
pub const text_pink_300: &'static str = "text-pink-300";
pub const text_pink_400: &'static str = "text-pink-400";
pub const text_pink_500: &'static str = "text-pink-500";
pub const text_pink_600: &'static str = "text-pink-600";
pub const text_pink_700: &'static str = "text-pink-700";
pub const text_pink_800: &'static str = "text-pink-800";
pub const text_pink_900: &'static str = "text-pink-900";

pub const text_rose_50: &'static str = "text-rose-50";
pub const text_rose_100: &'static str = "text-rose-100";
pub const text_rose_200: &'static str = "text-rose-200";
pub const text_rose_300: &'static str = "text-rose-300";
pub const text_rose_400: &'static str = "text-rose-400";
pub const text_rose_500: &'static str = "text-rose-500";
pub const text_rose_600: &'static str = "text-rose-600";
pub const text_rose_700: &'static str = "text-rose-700";
pub const text_rose_800: &'static str = "text-rose-800";
pub const text_rose_900: &'static str = "text-rose-900";

pub fn with_opacity(text: &str, o: u8) -> String {
    format!("{}/{}", text, o)
}

pub fn with_opacity_scale(text: &str, o: f32) -> String {
    format!("{}/[{}]", text, o)
}

pub fn text_any(p: &str) -> String {
    format!("text-[{}]", p)
}

// https://tailwindcss.com/docs/text-decoration
pub const underline: &'static str = "underline";
pub const overline: &'static str = "overline";
pub const line_through: &'static str = "line-through";
pub const no_underline: &'static str = "no-underline";

// https://tailwindcss.com/docs/text-decoration-color
pub const decoration_inherit: &'static str = "decoration-inherit";
pub const decoration_current: &'static str = "decoration-current";
pub const decoration_transparent: &'static str = "decoration-transparent";
pub const decoration_black: &'static str = "decoration-black";
pub const decoration_white: &'static str = "decoration-white";

pub const decoration_slate_50: &'static str = "decoration-slate-50";
pub const decoration_slate_100: &'static str = "decoration-slate-100";
pub const decoration_slate_200: &'static str = "decoration-slate-200";
pub const decoration_slate_300: &'static str = "decoration-slate-300";
pub const decoration_slate_400: &'static str = "decoration-slate-400";
pub const decoration_slate_500: &'static str = "decoration-slate-500";
pub const decoration_slate_600: &'static str = "decoration-slate-600";
pub const decoration_slate_700: &'static str = "decoration-slate-700";
pub const decoration_slate_800: &'static str = "decoration-slate-800";
pub const decoration_slate_900: &'static str = "decoration-slate-900";

pub const decoration_gray_50: &'static str = "decoration-gray-50";
pub const decoration_gray_100: &'static str = "decoration-gray-100";
pub const decoration_gray_200: &'static str = "decoration-gray-200";
pub const decoration_gray_300: &'static str = "decoration-gray-300";
pub const decoration_gray_400: &'static str = "decoration-gray-400";
pub const decoration_gray_500: &'static str = "decoration-gray-500";
pub const decoration_gray_600: &'static str = "decoration-gray-600";
pub const decoration_gray_700: &'static str = "decoration-gray-700";
pub const decoration_gray_800: &'static str = "decoration-gray-800";
pub const decoration_gray_900: &'static str = "decoration-gray-900";

pub const decoration_zinc_50: &'static str = "decoration-zinc-50";
pub const decoration_zinc_100: &'static str = "decoration-zinc-100";
pub const decoration_zinc_200: &'static str = "decoration-zinc-200";
pub const decoration_zinc_300: &'static str = "decoration-zinc-300";
pub const decoration_zinc_400: &'static str = "decoration-zinc-400";
pub const decoration_zinc_500: &'static str = "decoration-zinc-500";
pub const decoration_zinc_600: &'static str = "decoration-zinc-600";
pub const decoration_zinc_700: &'static str = "decoration-zinc-700";
pub const decoration_zinc_800: &'static str = "decoration-zinc-800";
pub const decoration_zinc_900: &'static str = "decoration-zinc-900";

pub const decoration_neutral_50: &'static str = "decoration-neutral-50";
pub const decoration_neutral_100: &'static str = "decoration-neutral-100";
pub const decoration_neutral_200: &'static str = "decoration-neutral-200";
pub const decoration_neutral_300: &'static str = "decoration-neutral-300";
pub const decoration_neutral_400: &'static str = "decoration-neutral-400";
pub const decoration_neutral_500: &'static str = "decoration-neutral-500";
pub const decoration_neutral_600: &'static str = "decoration-neutral-600";
pub const decoration_neutral_700: &'static str = "decoration-neutral-700";
pub const decoration_neutral_800: &'static str = "decoration-neutral-800";
pub const decoration_neutral_900: &'static str = "decoration-neutral-900";

pub const decoration_stone_50: &'static str = "decoration-stone-50";
pub const decoration_stone_100: &'static str = "decoration-stone-100";
pub const decoration_stone_200: &'static str = "decoration-stone-200";
pub const decoration_stone_300: &'static str = "decoration-stone-300";
pub const decoration_stone_400: &'static str = "decoration-stone-400";
pub const decoration_stone_500: &'static str = "decoration-stone-500";
pub const decoration_stone_600: &'static str = "decoration-stone-600";
pub const decoration_stone_700: &'static str = "decoration-stone-700";
pub const decoration_stone_800: &'static str = "decoration-stone-800";
pub const decoration_stone_900: &'static str = "decoration-stone-900";

pub const decoration_red_50: &'static str = "decoration-red-50";
pub const decoration_red_100: &'static str = "decoration-red-100";
pub const decoration_red_200: &'static str = "decoration-red-200";
pub const decoration_red_300: &'static str = "decoration-red-300";
pub const decoration_red_400: &'static str = "decoration-red-400";
pub const decoration_red_500: &'static str = "decoration-red-500";
pub const decoration_red_600: &'static str = "decoration-red-600";
pub const decoration_red_700: &'static str = "decoration-red-700";
pub const decoration_red_800: &'static str = "decoration-red-800";
pub const decoration_red_900: &'static str = "decoration-red-900";

pub const decoration_orange_50: &'static str = "decoration-orange-50";
pub const decoration_orange_100: &'static str = "decoration-orange-100";
pub const decoration_orange_200: &'static str = "decoration-orange-200";
pub const decoration_orange_300: &'static str = "decoration-orange-300";
pub const decoration_orange_400: &'static str = "decoration-orange-400";
pub const decoration_orange_500: &'static str = "decoration-orange-500";
pub const decoration_orange_600: &'static str = "decoration-orange-600";
pub const decoration_orange_700: &'static str = "decoration-orange-700";
pub const decoration_orange_800: &'static str = "decoration-orange-800";
pub const decoration_orange_900: &'static str = "decoration-orange-900";

pub const decoration_amber_50: &'static str = "decoration-amber-50";
pub const decoration_amber_100: &'static str = "decoration-amber-100";
pub const decoration_amber_200: &'static str = "decoration-amber-200";
pub const decoration_amber_300: &'static str = "decoration-amber-300";
pub const decoration_amber_400: &'static str = "decoration-amber-400";
pub const decoration_amber_500: &'static str = "decoration-amber-500";
pub const decoration_amber_600: &'static str = "decoration-amber-600";
pub const decoration_amber_700: &'static str = "decoration-amber-700";
pub const decoration_amber_800: &'static str = "decoration-amber-800";
pub const decoration_amber_900: &'static str = "decoration-amber-900";

pub const decoration_yellow_50: &'static str = "decoration-yellow-50";
pub const decoration_yellow_100: &'static str = "decoration-yellow-100";
pub const decoration_yellow_200: &'static str = "decoration-yellow-200";
pub const decoration_yellow_300: &'static str = "decoration-yellow-300";
pub const decoration_yellow_400: &'static str = "decoration-yellow-400";
pub const decoration_yellow_500: &'static str = "decoration-yellow-500";
pub const decoration_yellow_600: &'static str = "decoration-yellow-600";
pub const decoration_yellow_700: &'static str = "decoration-yellow-700";
pub const decoration_yellow_800: &'static str = "decoration-yellow-800";
pub const decoration_yellow_900: &'static str = "decoration-yellow-900";

pub const decoration_lime_50: &'static str = "decoration-lime-50";
pub const decoration_lime_100: &'static str = "decoration-lime-100";
pub const decoration_lime_200: &'static str = "decoration-lime-200";
pub const decoration_lime_300: &'static str = "decoration-lime-300";
pub const decoration_lime_400: &'static str = "decoration-lime-400";
pub const decoration_lime_500: &'static str = "decoration-lime-500";
pub const decoration_lime_600: &'static str = "decoration-lime-600";
pub const decoration_lime_700: &'static str = "decoration-lime-700";
pub const decoration_lime_800: &'static str = "decoration-lime-800";
pub const decoration_lime_900: &'static str = "decoration-lime-900";

pub const decoration_green_50: &'static str = "decoration-green-50";
pub const decoration_green_100: &'static str = "decoration-green-100";
pub const decoration_green_200: &'static str = "decoration-green-200";
pub const decoration_green_300: &'static str = "decoration-green-300";
pub const decoration_green_400: &'static str = "decoration-green-400";
pub const decoration_green_500: &'static str = "decoration-green-500";
pub const decoration_green_600: &'static str = "decoration-green-600";
pub const decoration_green_700: &'static str = "decoration-green-700";
pub const decoration_green_800: &'static str = "decoration-green-800";
pub const decoration_green_900: &'static str = "decoration-green-900";

pub const decoration_emerald_50: &'static str = "decoration-emerald-50";
pub const decoration_emerald_100: &'static str = "decoration-emerald-100";
pub const decoration_emerald_200: &'static str = "decoration-emerald-200";
pub const decoration_emerald_300: &'static str = "decoration-emerald-300";
pub const decoration_emerald_400: &'static str = "decoration-emerald-400";
pub const decoration_emerald_500: &'static str = "decoration-emerald-500";
pub const decoration_emerald_600: &'static str = "decoration-emerald-600";
pub const decoration_emerald_700: &'static str = "decoration-emerald-700";
pub const decoration_emerald_800: &'static str = "decoration-emerald-800";
pub const decoration_emerald_900: &'static str = "decoration-emerald-900";

pub const decoration_teal_50: &'static str = "decoration-teal-50";
pub const decoration_teal_100: &'static str = "decoration-teal-100";
pub const decoration_teal_200: &'static str = "decoration-teal-200";
pub const decoration_teal_300: &'static str = "decoration-teal-300";
pub const decoration_teal_400: &'static str = "decoration-teal-400";
pub const decoration_teal_500: &'static str = "decoration-teal-500";
pub const decoration_teal_600: &'static str = "decoration-teal-600";
pub const decoration_teal_700: &'static str = "decoration-teal-700";
pub const decoration_teal_800: &'static str = "decoration-teal-800";
pub const decoration_teal_900: &'static str = "decoration-teal-900";

pub const decoration_cyan_50: &'static str = "decoration-cyan-50";
pub const decoration_cyan_100: &'static str = "decoration-cyan-100";
pub const decoration_cyan_200: &'static str = "decoration-cyan-200";
pub const decoration_cyan_300: &'static str = "decoration-cyan-300";
pub const decoration_cyan_400: &'static str = "decoration-cyan-400";
pub const decoration_cyan_500: &'static str = "decoration-cyan-500";
pub const decoration_cyan_600: &'static str = "decoration-cyan-600";
pub const decoration_cyan_700: &'static str = "decoration-cyan-700";
pub const decoration_cyan_800: &'static str = "decoration-cyan-800";
pub const decoration_cyan_900: &'static str = "decoration-cyan-900";

pub const decoration_sky_50: &'static str = "decoration-sky-50";
pub const decoration_sky_100: &'static str = "decoration-sky-100";
pub const decoration_sky_200: &'static str = "decoration-sky-200";
pub const decoration_sky_300: &'static str = "decoration-sky-300";
pub const decoration_sky_400: &'static str = "decoration-sky-400";
pub const decoration_sky_500: &'static str = "decoration-sky-500";
pub const decoration_sky_600: &'static str = "decoration-sky-600";
pub const decoration_sky_700: &'static str = "decoration-sky-700";
pub const decoration_sky_800: &'static str = "decoration-sky-800";
pub const decoration_sky_900: &'static str = "decoration-sky-900";

pub const decoration_blue_50: &'static str = "decoration-blue-50";
pub const decoration_blue_100: &'static str = "decoration-blue-100";
pub const decoration_blue_200: &'static str = "decoration-blue-200";
pub const decoration_blue_300: &'static str = "decoration-blue-300";
pub const decoration_blue_400: &'static str = "decoration-blue-400";
pub const decoration_blue_500: &'static str = "decoration-blue-500";
pub const decoration_blue_600: &'static str = "decoration-blue-600";
pub const decoration_blue_700: &'static str = "decoration-blue-700";
pub const decoration_blue_800: &'static str = "decoration-blue-800";
pub const decoration_blue_900: &'static str = "decoration-blue-900";

pub const decoration_indigo_50: &'static str = "decoration-indigo-50";
pub const decoration_indigo_100: &'static str = "decoration-indigo-100";
pub const decoration_indigo_200: &'static str = "decoration-indigo-200";
pub const decoration_indigo_300: &'static str = "decoration-indigo-300";
pub const decoration_indigo_400: &'static str = "decoration-indigo-400";
pub const decoration_indigo_500: &'static str = "decoration-indigo-500";
pub const decoration_indigo_600: &'static str = "decoration-indigo-600";
pub const decoration_indigo_700: &'static str = "decoration-indigo-700";
pub const decoration_indigo_800: &'static str = "decoration-indigo-800";
pub const decoration_indigo_900: &'static str = "decoration-indigo-900";

pub const decoration_violet_50: &'static str = "decoration-violet-50";
pub const decoration_violet_100: &'static str = "decoration-violet-100";
pub const decoration_violet_200: &'static str = "decoration-violet-200";
pub const decoration_violet_300: &'static str = "decoration-violet-300";
pub const decoration_violet_400: &'static str = "decoration-violet-400";
pub const decoration_violet_500: &'static str = "decoration-violet-500";
pub const decoration_violet_600: &'static str = "decoration-violet-600";
pub const decoration_violet_700: &'static str = "decoration-violet-700";
pub const decoration_violet_800: &'static str = "decoration-violet-800";
pub const decoration_violet_900: &'static str = "decoration-violet-900";

pub const decoration_purple_50: &'static str = "decoration-purple-50";
pub const decoration_purple_100: &'static str = "decoration-purple-100";
pub const decoration_purple_200: &'static str = "decoration-purple-200";
pub const decoration_purple_300: &'static str = "decoration-purple-300";
pub const decoration_purple_400: &'static str = "decoration-purple-400";
pub const decoration_purple_500: &'static str = "decoration-purple-500";
pub const decoration_purple_600: &'static str = "decoration-purple-600";
pub const decoration_purple_700: &'static str = "decoration-purple-700";
pub const decoration_purple_800: &'static str = "decoration-purple-800";
pub const decoration_purple_900: &'static str = "decoration-purple-900";

pub const decoration_fuchsia_50: &'static str = "decoration-fuchsia-50";
pub const decoration_fuchsia_100: &'static str = "decoration-fuchsia-100";
pub const decoration_fuchsia_200: &'static str = "decoration-fuchsia-200";
pub const decoration_fuchsia_300: &'static str = "decoration-fuchsia-300";
pub const decoration_fuchsia_400: &'static str = "decoration-fuchsia-400";
pub const decoration_fuchsia_500: &'static str = "decoration-fuchsia-500";
pub const decoration_fuchsia_600: &'static str = "decoration-fuchsia-600";
pub const decoration_fuchsia_700: &'static str = "decoration-fuchsia-700";
pub const decoration_fuchsia_800: &'static str = "decoration-fuchsia-800";
pub const decoration_fuchsia_900: &'static str = "decoration-fuchsia-900";

pub const decoration_pink_50: &'static str = "decoration-pink-50";
pub const decoration_pink_100: &'static str = "decoration-pink-100";
pub const decoration_pink_200: &'static str = "decoration-pink-200";
pub const decoration_pink_300: &'static str = "decoration-pink-300";
pub const decoration_pink_400: &'static str = "decoration-pink-400";
pub const decoration_pink_500: &'static str = "decoration-pink-500";
pub const decoration_pink_600: &'static str = "decoration-pink-600";
pub const decoration_pink_700: &'static str = "decoration-pink-700";
pub const decoration_pink_800: &'static str = "decoration-pink-800";
pub const decoration_pink_900: &'static str = "decoration-pink-900";

pub const decoration_rose_50: &'static str = "decoration-rose-50";
pub const decoration_rose_100: &'static str = "decoration-rose-100";
pub const decoration_rose_200: &'static str = "decoration-rose-200";
pub const decoration_rose_300: &'static str = "decoration-rose-300";
pub const decoration_rose_400: &'static str = "decoration-rose-400";
pub const decoration_rose_500: &'static str = "decoration-rose-500";
pub const decoration_rose_600: &'static str = "decoration-rose-600";
pub const decoration_rose_700: &'static str = "decoration-rose-700";
pub const decoration_rose_800: &'static str = "decoration-rose-800";
pub const decoration_rose_900: &'static str = "decoration-rose-900";

pub fn decoration_any(p: &str) -> String {
    format!("decoration-[{}]", p)
}

// https://tailwindcss.com/docs/text-decoration-style
pub const decoration_solid: &'static str = "decoration-solid";
pub const decoration_double: &'static str = "decoration-double";
pub const decoration_dotted: &'static str = "decoration-dotted";
pub const decoration_dashed: &'static str = "decoration-dashed";
pub const decoration_wavy: &'static str = "decoration-wavy";

// https://tailwindcss.com/docs/text-decoration-thickness
pub const decoration_auto: &'static str = "decoration-auto";
pub const decoration_from_font: &'static str = "decoration-from-font";
pub const decoration_0: &'static str = "decoration-0";
pub const decoration_1: &'static str = "decoration-1";
pub const decoration_2: &'static str = "decoration-2";
pub const decoration_4: &'static str = "decoration-4";
pub const decoration_8: &'static str = "decoration-8";

// https://tailwindcss.com/docs/text-underline-offset
pub const underline_offset_auto: &'static str = "underline-offset-auto";
pub const underline_offset_0: &'static str = "underline-offset-0";
pub const underline_offset_1: &'static str = "underline-offset-1";
pub const underline_offset_2: &'static str = "underline-offset-2";
pub const underline_offset_4: &'static str = "underline-offset-4";
pub const underline_offset_8: &'static str = "underline-offset-8";

pub fn underline_offset_any(p: &str) -> String {
    format!("underline-offset-[{}]", p)
}

// https://tailwindcss.com/docs/text-transform
pub const uppercase: &'static str = "uppercase";
pub const lowercase: &'static str = "lowercase";
pub const capitalize: &'static str = "capitalize";
pub const normal_case: &'static str = "normal-case";

// https://tailwindcss.com/docs/text-overflow
pub const truncate: &'static str = "truncate";
pub const text_ellipsis: &'static str = "text-ellipsis";
pub const text_clip: &'static str = "text-clip";

// https://tailwindcss.com/docs/text-indent
pub const indent_0: &'static str = "indent-0";
pub const indent_px: &'static str = "indent-px";
pub const indent_half: &'static str = "indent-0.5";
pub const indent_1: &'static str = "indent-1";
pub const indent_1_half: &'static str = "indent-1.5";
pub const indent_2: &'static str = "indent-2";
pub const indent_2_half: &'static str = "indent-2.5";
pub const indent_3: &'static str = "indent-3";
pub const indent_3_half: &'static str = "indent-3.5";
pub const indent_4: &'static str = "indent-4";
pub const indent_5: &'static str = "indent-5";
pub const indent_6: &'static str = "indent-6";
pub const indent_7: &'static str = "indent-7";
pub const indent_8: &'static str = "indent-8";
pub const indent_9: &'static str = "indent-9";
pub const indent_10: &'static str = "indent-10";
pub const indent_11: &'static str = "indent-11";
pub const indent_12: &'static str = "indent-12";
pub const indent_16: &'static str = "indent-16";
pub const indent_20: &'static str = "indent-20";
pub const indent_24: &'static str = "indent-24";
pub const indent_28: &'static str = "indent-28";
pub const indent_32: &'static str = "indent-32";
pub const indent_36: &'static str = "indent-36";
pub const indent_40: &'static str = "indent-40";
pub const indent_44: &'static str = "indent-44";
pub const indent_48: &'static str = "indent-48";
pub const indent_52: &'static str = "indent-52";
pub const indent_56: &'static str = "indent-56";
pub const indent_60: &'static str = "indent-60";
pub const indent_64: &'static str = "indent-64";
pub const indent_72: &'static str = "indent-72";
pub const indent_80: &'static str = "indent-80";
pub const indent_96: &'static str = "indent-96";

pub fn indent_any(p: &str) -> String {
    format!("indent-[{}]", p)
}

// https://tailwindcss.com/docs/vertical-align
pub const align_baseline: &'static str = "align-baseline";
pub const align_top: &'static str = "align-top";
pub const align_middle: &'static str = "align-middle";
pub const align_bottom: &'static str = "align-bottom";
pub const align_text_top: &'static str = "align-text-top";
pub const align_text_bottom: &'static str = "align-text-bottom";
pub const align_sub: &'static str = "align-sub";
pub const align_super: &'static str = "align-super";

pub fn align_any(p: &str) -> String {
    format!("align-[{}]", p)
}

// https://tailwindcss.com/docs/whitespace
pub const whitespace_normal: &'static str = "whitespace-normal";
pub const whitespace_nowrap: &'static str = "whitespace-nowrap";
pub const whitespace_pre: &'static str = "whitespace-pre";
pub const whitespace_pre_line: &'static str = "whitespace-pre-line";
pub const whitespace_pre_wrap: &'static str = "whitespace-pre-wrap";

// https://tailwindcss.com/docs/word-break
pub const break_normal: &'static str = "break-normal";
pub const break_words: &'static str = "break-words";
pub const break_all: &'static str = "break-all";
pub const break_keep: &'static str = "break-keep";

// https://tailwindcss.com/docs/content
pub const content_none: &'static str = "content-none";

pub fn content_any(p: &str) -> String {
    format!("content-[{}]", p)
}