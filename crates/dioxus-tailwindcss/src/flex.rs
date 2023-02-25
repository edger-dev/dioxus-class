use crate::ext::*;

// https://tailwindcss.com/docs/flex-basis
constant!(basis auto);
size_0_to_96!(basis);
fraction_2_to_6!(basis);
fraction_12!(basis);
constant!(basis full);

// https://tailwindcss.com/docs/flex-direction
constant!(flex row);
constant!(flex row reverse);
constant!(flex col);
constant!(flex col reverse);

// https://tailwindcss.com/docs/flex-wrap
constant!(flex wrap);
constant!(flex wrap reverse);
constant!(flex nowrap);

// https://tailwindcss.com/docs/flex
constant!(flex 1);
constant!(flex auto);
constant!(flex initial);
constant!(flex none);

any!(flex);

// https://tailwindcss.com/docs/flex-grow
constant!(grow);
constant!(grow 0);

any!(grow);

// https://tailwindcss.com/docs/flex-shrink
constant!(shrink);
constant!(shrink 0);

any!(shrink);

// https://tailwindcss.com/docs/order
_1_to_12!(order);
constant!(order first);
constant!(order last);
constant!(order none);

any!(order);

// https://tailwindcss.com/docs/grid-template-columns
_1_to_12!(grid cols);
constant!(grid cols none);

any!(grid cols);

// https://tailwindcss.com/docs/grid-column
constant!(col auto);

_1_to_12!(col span);
constant!(col span full);

_1_to_12!(col start);
constant!(col start auto);

_1_to_12!(col end);
constant!(col end auto);

any!(col);
any!(col start);
any!(col end);

// https://tailwindcss.com/docs/grid-template-rows
_1_to_6!(grid rows);
constant!(grid rows none);

any!(grid rows);

// https://tailwindcss.com/docs/grid-row
constant!(row auto);
_1_to_6!(row span);
constant!(row span full);

_1_to_6!(row start);
constant!(row start 7);
constant!(row start auto);

_1_to_6!(row end);
constant!(row end 7);
constant!(row end auto);

any!(row);

any!(row start);

any!(row end);

// https://tailwindcss.com/docs/grid-auto-flow
constant!(grid flow row);
constant!(grid flow col);
constant!(grid flow dense);
constant!(grid flow row dense);
constant!(grid flow col dense);

// https://tailwindcss.com/docs/grid-auto-columns
constant!(auto cols auto);
constant!(auto cols min);
constant!(auto cols max);
constant!(auto cols fr);

any!(auto cols);

// https://tailwindcss.com/docs/grid-auto-rows
constant!(auto rows auto);
constant!(auto rows min);
constant!(auto rows max);
constant!(auto rows fr);

any!(auto rows);

// https://tailwindcss.com/docs/gap
size_0_to_96!(gap);
size_0_to_96!(gap x);
size_0_to_96!(gap y);

// https://tailwindcss.com/docs/justify-content
constant!(justify start);
constant!(justify end);
constant!(justify center);
constant!(justify between);
constant!(justify around);
constant!(justify evenly);

// https://tailwindcss.com/docs/justify-items
constant!(justify items start);
constant!(justify items end);
constant!(justify items center);
constant!(justify items stretch);

// https://tailwindcss.com/docs/justify-self
constant!(justify self auto);
constant!(justify self start);
constant!(justify self end);
constant!(justify self center);
constant!(justify self stretch);

// https://tailwindcss.com/docs/align-content
constant!(content start);
constant!(content end);
constant!(content center);
constant!(content between);
constant!(content around);
constant!(content evenly);
constant!(content baseline);

// https://tailwindcss.com/docs/align-items
constant!(items start);
constant!(items end);
constant!(items center);
constant!(items baseline);
constant!(items stretch);

// https://tailwindcss.com/docs/align-self
constant!(self auto);
constant!(self start);
constant!(self end);
constant!(self center);
constant!(self baseline);
constant!(self stretch);

// https://tailwindcss.com/docs/place-content
constant!(place content start);
constant!(place content end);
constant!(place content center);
constant!(place content between);
constant!(place content around);
constant!(place content evenly);
constant!(place content baseline);
constant!(place content stretch);

// https://tailwindcss.com/docs/place-items
constant!(place items start);
constant!(place items end);
constant!(place items center);
constant!(place items baseline);
constant!(place items stretch);

// https://tailwindcss.com/docs/place-self
constant!(place self auto);
constant!(place self start);
constant!(place self end);
constant!(place self center);
constant!(place self stretch);

