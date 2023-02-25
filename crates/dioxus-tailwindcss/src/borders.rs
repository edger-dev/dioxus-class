use crate::ext::*;

// https://tailwindcss.com/docs/border-radius
constant!(rounded);
constant!(rounded none);
constant!(rounded sm);
constant!(rounded md);
constant!(rounded lg);
constant!(rounded xl);
constant!(rounded 2xl);
constant!(rounded 3xl);
constant!(rounded full);

constant!(rounded t);
constant!(rounded t none);
constant!(rounded t sm);
constant!(rounded t md);
constant!(rounded t lg);
constant!(rounded t xl);
constant!(rounded t 2xl);
constant!(rounded t 3xl);
constant!(rounded t full);

constant!(rounded r);
constant!(rounded r none);
constant!(rounded r sm);
constant!(rounded r md);
constant!(rounded r lg);
constant!(rounded r xl);
constant!(rounded r 2xl);
constant!(rounded r 3xl);
constant!(rounded r full);

constant!(rounded b);
constant!(rounded b none);
constant!(rounded b sm);
constant!(rounded b md);
constant!(rounded b lg);
constant!(rounded b xl);
constant!(rounded b 2xl);
constant!(rounded b 3xl);
constant!(rounded b full);

constant!(rounded l);
constant!(rounded l none);
constant!(rounded l sm);
constant!(rounded l md);
constant!(rounded l lg);
constant!(rounded l xl);
constant!(rounded l 2xl);
constant!(rounded l 3xl);
constant!(rounded l full);

constant!(rounded tl);
constant!(rounded tl none);
constant!(rounded tl sm);
constant!(rounded tl md);
constant!(rounded tl lg);
constant!(rounded tl xl);
constant!(rounded tl 2xl);
constant!(rounded tl 3xl);
constant!(rounded tl full);

constant!(rounded tr);
constant!(rounded tr none);
constant!(rounded tr sm);
constant!(rounded tr md);
constant!(rounded tr lg);
constant!(rounded tr xl);
constant!(rounded tr 2xl);
constant!(rounded tr 3xl);
constant!(rounded tr full);

constant!(rounded br);
constant!(rounded br none);
constant!(rounded br sm);
constant!(rounded br md);
constant!(rounded br lg);
constant!(rounded br xl);
constant!(rounded br 2xl);
constant!(rounded br 3xl);
constant!(rounded br full);

constant!(rounded bl);
constant!(rounded bl none);
constant!(rounded bl sm);
constant!(rounded bl md);
constant!(rounded bl lg);
constant!(rounded bl xl);
constant!(rounded bl 2xl);
constant!(rounded bl 3xl);
constant!(rounded bl full);

any!(rounded);

// https://tailwindcss.com/docs/border-width
constant!(border);
constant!(border 0);
constant!(border 2);
constant!(border 4);
constant!(border 8);

constant!(border x);
constant!(border x 0);
constant!(border x 2);
constant!(border x 4);
constant!(border x 8);

constant!(border y);
constant!(border y 0);
constant!(border y 2);
constant!(border y 4);
constant!(border y 8);

constant!(border t);
constant!(border t 0);
constant!(border t 2);
constant!(border t 4);
constant!(border t 8);

constant!(border r);
constant!(border r 0);
constant!(border r 2);
constant!(border r 4);
constant!(border r 8);

constant!(border b);
constant!(border b 0);
constant!(border b 2);
constant!(border b 4);
constant!(border b 8);

constant!(border l);
constant!(border l 0);
constant!(border l 2);
constant!(border l 4);
constant!(border l 8);

// https://tailwindcss.com/docs/border-color
colors!(border);
colors!(border x);
colors!(border y);
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
