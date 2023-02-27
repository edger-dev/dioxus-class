use crate::ext::*;

// https://tailwindcss.com/docs/transition-property
constant!(transition none);
constant!(transition all);
constant!(transition);
constant!(transition colors);
constant!(transition opacity);
constant!(transition shadow);
constant!(transition transform);

any!(transition);

// https://tailwindcss.com/docs/transition-duration
constant!(duration 75);
constant!(duration 100);
constant!(duration 150);
constant!(duration 200);
constant!(duration 300);
constant!(duration 500);
constant!(duration 700);
constant!(duration 1000);

any!(duration);

// https://tailwindcss.com/docs/transition-timing-function
constant!(ease linear);
constant!(ease in);
constant!(ease out);
constant!(ease in out);

any!(ease);

// https://tailwindcss.com/docs/transition-delay
constant!(delay 75);
constant!(delay 100);
constant!(delay 150);
constant!(delay 200);
constant!(delay 300);
constant!(delay 500);
constant!(delay 700);
constant!(delay 1000);

any!(delay);

// https://tailwindcss.com/docs/animation
constant!(animate none);
constant!(animate spin);
constant!(animate ping);
constant!(animate pulse);
constant!(animate bounce);

pub fn motion_safe(v: &str) -> String {
    "motion-safe:".to_owned() + v
}

pub fn motion_reduce(v: &str) -> String {
    "motion-reduce:".to_owned() + v
}

any!(animate);