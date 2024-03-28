// https://tailwindcss.com/docs/responsive-design

/// sm:
pub fn sm(v: &str) -> String {
    "sm:".to_owned() + v
}

/// md:
pub fn md(v: &str) -> String {
    "md:".to_owned() + v
}

/// lg:
pub fn lg(v: &str) -> String {
    "lg:".to_owned() + v
}

/// xl:
pub fn xl(v: &str) -> String {
    "xl:".to_owned() + v
}

/// 2xl:
pub fn _2xl(v: &str) -> String {
    "2xl:".to_owned() + v
}

/// 2xl:
pub fn xxl(v: &str) -> String {
    "2xl:".to_owned() + v
}

/// max-sm:
pub fn max_sm(v: &str) -> String {
    "max-sm:".to_owned() + v
}

/// max-md:
pub fn max_md(v: &str) -> String {
    "max-md:".to_owned() + v
}

/// max-lg:
pub fn max_lg(v: &str) -> String {
    "max-lg:".to_owned() + v
}

/// max-xl:
pub fn max_xl(v: &str) -> String {
    "max-xl:".to_owned() + v
}

/// max-2xl:
pub fn max_2xl(v: &str) -> String {
    "max-2xl:".to_owned() + v
}

/// max-2xl:
pub fn max_xxl(v: &str) -> String {
    "max-2xl:".to_owned() + v
}

/// min-[{p}]:
pub fn min_any(p: &str, v: &str) -> String {
    format!("min-[{}]:{}", p, v)
}

/// max-[{p}]:
pub fn max_any(p: &str, v: &str) -> String {
    format!("max-[{}]:{}", p, v)
}