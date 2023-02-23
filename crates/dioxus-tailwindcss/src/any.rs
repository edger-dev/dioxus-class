// https://tailwindcss.com/docs/responsive-design

pub fn px(v: u16) -> String {
    format!("{}px", v)
}

pub fn rem(v: f32) -> String {
    format!("{}rem", v)
}

pub fn percent(v: f32) -> String {
    format!("{}%", v)
}