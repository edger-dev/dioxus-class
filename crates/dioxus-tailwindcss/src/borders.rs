use crate::ext::*;

// https://tailwindcss.com/docs/border-radius
border_radius!(rounded);
border_radius!(rounded s);
border_radius!(rounded e);
border_radius!(rounded t);
border_radius!(rounded r);
border_radius!(rounded b);
border_radius!(rounded l);
border_radius!(rounded ss);
border_radius!(rounded se);
border_radius!(rounded ee);
border_radius!(rounded es);
border_radius!(rounded tl);
border_radius!(rounded tr);
border_radius!(rounded br);
border_radius!(rounded bl);

any!(rounded);

// https://tailwindcss.com/docs/border-width
border_width!(border);
border_width!(border x);
border_width!(border y);
border_width!(border s);
border_width!(border e);
border_width!(border t);
border_width!(border r);
border_width!(border b);
border_width!(border l);

// https://tailwindcss.com/docs/border-color
colors!(border);
colors!(border x);
colors!(border y);
colors!(border s);
colors!(border e);
colors!(border t);
colors!(border r);
colors!(border b);
colors!(border l);

// https://tailwindcss.com/docs/border-style
constant!(border solid);
constant!(border dashed);
constant!(border dotted);
constant!(border double);
constant!(border hidden);
constant!(border none);

// https://tailwindcss.com/docs/divide-width
constant!(divide x);
constant!(divide x 0);
constant!(divide x 2);
constant!(divide x 4);
constant!(divide x 8);
constant!(divide x reverse);

constant!(divide y);
constant!(divide y 0);
constant!(divide y 2);
constant!(divide y 4);
constant!(divide y 8);
constant!(divide y reverse);

// https://tailwindcss.com/docs/divide-color
colors!(divide);

// https://tailwindcss.com/docs/divide-style
constant!(divide solid);
constant!(divide dashed);
constant!(divide dotted);
constant!(divide double);
constant!(divide none);

// https://tailwindcss.com/docs/outline-width
constant!(outline 0);
constant!(outline 1);
constant!(outline 2);
constant!(outline 4);
constant!(outline 8);

// https://tailwindcss.com/docs/outline-color
colors!(outline);

// https://tailwindcss.com/docs/outline-style
constant!(outline none);
constant!(outline);
constant!(outline dashed);
constant!(outline dotted);
constant!(outline double);

// https://tailwindcss.com/docs/outline-offset
constant!(outline offset 0);
constant!(outline offset 1);
constant!(outline offset 2);
constant!(outline offset 4);
constant!(outline offset 8);

any!(outline offset);

// https://tailwindcss.com/docs/ring-width
constant!(ring);
constant!(ring 0);
constant!(ring 1);
constant!(ring 2);
constant!(ring 4);
constant!(ring 8);
constant!(ring inset);

// https://tailwindcss.com/docs/ring-color
colors!(ring);

// https://tailwindcss.com/docs/ring-offset-width
constant!(ring offset 0);
constant!(ring offset 1);
constant!(ring offset 2);
constant!(ring offset 4);
constant!(ring offset 8);

// https://tailwindcss.com/docs/ring-offset-color
colors!(ring offset);
