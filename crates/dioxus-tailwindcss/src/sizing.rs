use crate::ext::*;

// https://tailwindcss.com/docs/width
size_0_to_96!(w);
fraction_2_to_6!(w);
fraction_12!(w);
constant!(w full);
constant!(w screen);
constant!(w min);
constant!(w max);
constant!(w fit);

// https://tailwindcss.com/docs/min-width
constant!(min w full);
constant!(min w min);
constant!(min w max);
constant!(min w fit);

any!(min w);

constant!(max w 0);
constant!(max w none);
xs_to_7xl!(max w);
constant!(max w full);
constant!(max w min);
constant!(max w max);
constant!(max w fix);
constant!(max w prose);

sm_to_2xl!(max w screen);

any!(max w);

// https://tailwindcss.com/docs/height
size_0_to_96!(h);
fraction_2_to_6!(h);
constant!(h auto);
constant!(h full);
constant!(h screen);
constant!(h min);
constant!(h max);
constant!(h fit);

// https://tailwindcss.com/docs/min-height
constant!(min h 0);
constant!(min h full);
constant!(min h screen);
constant!(min h min);
constant!(min h max);
constant!(min h fit);

any!(min h);

size_0_to_96!(max h);
constant!(max h none);
constant!(max h full);
constant!(max h screen);
constant!(max h min);
constant!(max h max);
constant!(max h fit);
