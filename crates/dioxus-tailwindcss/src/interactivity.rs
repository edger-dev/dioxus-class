use crate::ext::*;

// https://tailwindcss.com/docs/accent-color
colors!(accent);
constant!(accent auto);

// https://tailwindcss.com/docs/appearance
constant!(appearance none);
constant!(appearance auto);

// https://tailwindcss.com/docs/cursor
constant!(cursor auto);
constant!(cursor default);
constant!(cursor pointer);
constant!(cursor wait);
constant!(cursor text);
constant!(cursor move);
constant!(cursor help);
constant!(cursor not allowed);
constant!(cursor none);
constant!(cursor context menu);
constant!(cursor progress);
constant!(cursor cell);
constant!(cursor crosshair);
constant!(cursor vertical text);
constant!(cursor alias);
constant!(cursor copy);
constant!(cursor no drop);
constant!(cursor grab);
constant!(cursor grabbing);
constant!(cursor all scoll);
constant!(cursor col resize);
constant!(cursor row resize);
constant!(cursor n resize);
constant!(cursor e resize);
constant!(cursor s resize);
constant!(cursor w resize);
constant!(cursor ne resize);
constant!(cursor nw resize);
constant!(cursor se resize);
constant!(cursor sw resize);
constant!(cursor ew resize);
constant!(cursor ns resize);
constant!(cursor nesw resize);
constant!(cursor nwse resize);
constant!(cursor zoom in);
constant!(cursor zoom out);

any!(cursor);

// https://tailwindcss.com/docs/caret-color
colors!(caret);

// https://tailwindcss.com/docs/pointer-events
constant!(pointer events none);
constant!(pointer events auto);

// https://tailwindcss.com/docs/resize
constant!(resize none);
constant!(resize x);
constant!(resize y);
constant!(resize);

// https://tailwindcss.com/docs/scroll-behavior
constant!(scroll auto);
constant!(scroll smooth);

// https://tailwindcss.com/docs/scroll-margin
size_0_to_96!(scroll m);
size_0_to_96!(scroll mx);
size_0_to_96!(scroll my);
size_0_to_96!(scroll ms);
size_0_to_96!(scroll me);
size_0_to_96!(scroll mt);
size_0_to_96!(scroll mr);
size_0_to_96!(scroll mb);
size_0_to_96!(scroll ml);

// https://tailwindcss.com/docs/scroll-padding
size_0_to_96!(scroll p);
size_0_to_96!(scroll px);
size_0_to_96!(scroll py);
size_0_to_96!(scroll ps);
size_0_to_96!(scroll pe);
size_0_to_96!(scroll pt);
size_0_to_96!(scroll pr);
size_0_to_96!(scroll pb);
size_0_to_96!(scroll pl);

// https://tailwindcss.com/docs/scroll-snap-align
constant!(snap start);
constant!(snap end);
constant!(snap center);
constant!(snap align none);

// https://tailwindcss.com/docs/scroll-snap-stop
constant!(snap normal);
constant!(snap always);

// https://tailwindcss.com/docs/scroll-snap-type
constant!(snap none);
constant!(snap x);
constant!(snap y);
constant!(snap both);
constant!(snap mandatory);
constant!(snap proximity);

// https://tailwindcss.com/docs/touch-action
constant!(touch auto);
constant!(touch none);
constant!(touch pan x);
constant!(touch pan left);
constant!(touch pan right);
constant!(touch pan y);
constant!(touch pan up);
constant!(touch pan down);
constant!(touch pinch zoom);
constant!(touch manipulation);

// https://tailwindcss.com/docs/user-select
constant!(select none);
constant!(select text);
constant!(select all);
constant!(select auto);

// https://tailwindcss.com/docs/will-change
constant!(will change auto);
constant!(will change scroll);
constant!(will change contents);
constant!(will change transform);

any!(will change);