use crate::ext::*;

// https://tailwindcss.com/docs/border-collapse
constant!(border collapse);
constant!(border separate);

// https://tailwindcss.com/docs/border-spacing
size_0_to_96!(border spacing);
size_0_to_96!(border spacing x);
size_0_to_96!(border spacing y);

// https://tailwindcss.com/docs/table-layout
constant!(table auto);
constant!(table fixed);