use crate::ext::*;

// https://tailwindcss.com/docs/scale
scales!(scale);
scales!(scale x);
scales!(scale y);

// https://tailwindcss.com/docs/rotate
constant!(rotate 0);
constant!(rotate 1);
constant!(rotate 2);
constant!(rotate 3);
constant!(rotate 6);
constant!(rotate 12);
constant!(rotate 45);
constant!(rotate 90);
constant!(rotate 180);

any!(rotate);

// https://tailwindcss.com/docs/translate
size_0_to_96!(translate x);
fraction_2_to_4!(translate x);
constant!(translate x full);

size_0_to_96!(translate y);
fraction_2_to_4!(translate y);
constant!(translate y full);

// https://tailwindcss.com/docs/skew
constant!(skew x 0);
constant!(skew x 1);
constant!(skew x 2);
constant!(skew x 3);
constant!(skew x 6);
constant!(skew x 12);

any!(skew x);

constant!(skew y 0);
constant!(skew y 1);
constant!(skew y 2);
constant!(skew y 3);
constant!(skew y 6);
constant!(skew y 12);

any!(skew y);

// https://tailwindcss.com/docs/transform-origin
constant!(origin center);
constant!(origin left);
constant!(origin right);
constant!(origin top);
constant!(origin top left);
constant!(origin top right);
constant!(origin bottom);
constant!(origin bottom left);
constant!(origin bottom right);