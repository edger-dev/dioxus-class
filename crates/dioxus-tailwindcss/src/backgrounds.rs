use crate::ext::*;

// https://tailwindcss.com/docs/background-attachment
constant!(bg fixed);
constant!(bg local);
constant!(bg scroll);

// https://tailwindcss.com/docs/background-clip
constant!(bg clip border);
constant!(bg clip padding);
constant!(bg clip content);
constant!(bg clip text);

// https://tailwindcss.com/docs/background-color
colors!(bg);

// https://tailwindcss.com/docs/background-origin
constant!(bg origin border);
constant!(bg origin padding);
constant!(bg origin content);

// https://tailwindcss.com/docs/background-position
constant!(bg bottom);
constant!(bg center);
constant!(bg left);
constant!(bg left bottom);
constant!(bg left top);
constant!(bg right);
constant!(bg right bottom);
constant!(bg right top);
constant!(bg top);

// https://tailwindcss.com/docs/background-repeat
constant!(bg repeat);
constant!(bg no repeat);
constant!(bg repeat x);
constant!(bg repeat y);
constant!(bg repeat round);
constant!(bg repeat space);

// https://tailwindcss.com/docs/background-size
constant!(bg auto);
constant!(bg cover);
constant!(bg contain);

// https://tailwindcss.com/docs/background-image
constant!(bg none);
constant!(bg gradient to t);
constant!(bg gradient to tr);
constant!(bg gradient to r);
constant!(bg gradient to br);
constant!(bg gradient to b);
constant!(bg gradient to bl);
constant!(bg gradient to l);
constant!(bg gradient to tl);

// https://tailwindcss.com/docs/gradient-color-stops
colors!(from);
colors!(to);
