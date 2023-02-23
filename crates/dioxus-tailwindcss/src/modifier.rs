// https://tailwindcss.com/docs/hover-focus-and-other-states#pseudo-class-reference

pub fn hover(v: &str) -> String {
    "hover:".to_owned() + v
}

pub fn focus(v: &str) -> String {
    "focus:".to_owned() + v
}

pub fn focus_within(v: &str) -> String {
    "focus-within:".to_owned() + v
}

pub fn focus_visible(v: &str) -> String {
    "focus-visible:".to_owned() + v
}

pub fn active(v: &str) -> String {
    "active:".to_owned() + v
}

pub fn visited(v: &str) -> String {
    "visited:".to_owned() + v
}

pub fn target(v: &str) -> String {
    "target:".to_owned() + v
}

pub fn first(v: &str) -> String {
    "first:".to_owned() + v
}

pub fn last(v: &str) -> String {
    "last:".to_owned() + v
}

pub fn only(v: &str) -> String {
    "only:".to_owned() + v
}

pub fn odd(v: &str) -> String {
    "odd:".to_owned() + v
}

pub fn even(v: &str) -> String {
    "even:".to_owned() + v
}

pub fn first_of_type(v: &str) -> String {
    "first-of-type:".to_owned() + v
}

pub fn last_of_type(v: &str) -> String {
    "last-of-type:".to_owned() + v
}

pub fn only_of_type(v: &str) -> String {
    "only-of-type:".to_owned() + v
}

pub fn empty(v: &str) -> String {
    "empty:".to_owned() + v
}

pub fn disabled(v: &str) -> String {
    "disabled:".to_owned() + v
}

pub fn enabled(v: &str) -> String {
    "enabled:".to_owned() + v
}

pub fn checked(v: &str) -> String {
    "checked:".to_owned() + v
}

pub fn indeterminate(v: &str) -> String {
    "indeterminate:".to_owned() + v
}

pub fn default(v: &str) -> String {
    "default:".to_owned() + v
}

pub fn required(v: &str) -> String {
    "required:".to_owned() + v
}

pub fn valid(v: &str) -> String {
    "valid:".to_owned() + v
}

pub fn invalid(v: &str) -> String {
    "invalid:".to_owned() + v
}

pub fn in_range(v: &str) -> String {
    "in-range:".to_owned() + v
}

pub fn out_of_range(v: &str) -> String {
    "out-of-range:".to_owned() + v
}

pub fn placeholder_shown(v: &str) -> String {
    "placeholder-shown:".to_owned() + v
}

pub fn autofill(v: &str) -> String {
    "autofill:".to_owned() + v
}

pub fn read_only(v: &str) -> String {
    "read-only:".to_owned() + v
}
