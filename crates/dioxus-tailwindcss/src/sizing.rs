use crate::ext::*;

// https://tailwindcss.com/docs/width
size_0_to_96!(w);
fraction_2_to_6!(w);
fraction_12!(w);
constant!(w auto);
constant!(w full);
constant!(w screen);
constant!(w svw);
constant!(w lvw);
constant!(w dvw);
constant!(w min);
constant!(w max);
constant!(w fit);

// https://tailwindcss.com/docs/min-width
size_0_to_96!(min w);

constant!(min w full);
constant!(min w min);
constant!(min w max);
constant!(min w fit);

// https://tailwindcss.com/docs/max-width
size_0_to_96!(max w);
constant!(max w none);
xs_to_7xl!(max w);
constant!(max w full);
constant!(max w min);
constant!(max w max);
constant!(max w fix);
constant!(max w prose);
sm_to_2xl!(max w screen);

// https://tailwindcss.com/docs/height
size_0_to_96!(h);
fraction_2_to_6!(h);
constant!(h auto);
constant!(h full);
constant!(h screen);
constant!(h svh);
constant!(h lvh);
constant!(h dvh);
constant!(h min);
constant!(h max);
constant!(h fit);

// https://tailwindcss.com/docs/min-height
size_0_to_96!(min h);
constant!(min h full);
constant!(min h screen);
constant!(min h svh);
constant!(min h lvh);
constant!(min h dvh);
constant!(min h min);
constant!(min h max);
constant!(min h fit);


// https://tailwindcss.com/docs/max-height
size_0_to_96!(max h);
constant!(max h none);
constant!(max h full);
constant!(max h screen);
constant!(max h svh);
constant!(max h lvh);
constant!(max h dvh);
constant!(max h min);
constant!(max h max);
constant!(max h fit);

// https://tailwindcss.com/docs/size
size_0_to_96!(size);
fraction_2_to_6!(size);
fraction_12!(size);
constant!(size full);
constant!(size min);
constant!(size max);
constant!(size fix);
