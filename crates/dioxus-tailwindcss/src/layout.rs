use crate::ext::*;

// https://tailwindcss.com/docs/aspect-ratio
constant!(aspect auto);
constant!(aspect square);
constant!(aspect video);


// https://tailwindcss.com/docs/aspect-ratio#arbitrary-values
/// aspect-[{}/{}]
pub fn aspect(w: u16, h: u16) -> String {
    format!("aspect-[{}/{}]", w, h)
}

// https://tailwindcss.com/docs/container
constant!(container);

// https://tailwindcss.com/docs/columns
_1_to_12!(columns);
constant!(columns auto);
_3xs_to_7xl!(columns);

any!(columns);

// https://tailwindcss.com/docs/break-after
constant!(break after auto);
constant!(break after avoid);
constant!(break after all);
constant!(break after avoid page);
constant!(break after page);
constant!(break after left);
constant!(break after right);
constant!(break after column);

// https://tailwindcss.com/docs/break-before
constant!(break before auto);
constant!(break before avoid);
constant!(break before all);
constant!(break before avoid page);
constant!(break before page);
constant!(break before left);
constant!(break before right);
constant!(break before column);

// https://tailwindcss.com/docs/break-inside
constant!(break inside auto);
constant!(break inside avoid);
constant!(break inside avoid page);
constant!(break inside avoid column);

// https://tailwindcss.com/docs/box-decoration-break
constant!(box decoration clone);
constant!(box decoration slice);

// https://tailwindcss.com/docs/box-sizing
constant!(box border);
constant!(box content);

// https://tailwindcss.com/docs/display
constant!(block);
constant!(inline block);
constant!(inline);
constant!(flex);
constant!(inline flex);

constant!(table);
constant!(inline table);
constant!(table caption);
constant!(table cell);
constant!(table column);
constant!(table column group);
constant!(table footer group);
constant!(table header group);
constant!(table row group);
constant!(table row);
constant!(flow root);
constant!(grid);
constant!(inline grid);
constant!(contents);
constant!(list item);
constant!(hidden);

// https://tailwindcss.com/docs/float
constant!(float start);
constant!(float end);
constant!(float right);
constant!(float left);
constant!(float none);

// https://tailwindcss.com/docs/clear
constant!(clear start);
constant!(clear end);
constant!(clear left);
constant!(clear right);
constant!(clear both);
constant!(clear none);

// https://tailwindcss.com/docs/isolation
constant!(isolate);
constant!(isolation auto);

// https://tailwindcss.com/docs/object-fit
constant!(object contain);
constant!(object cover);
constant!(object fill);
constant!(object none);
constant!(object scale down);

// https://tailwindcss.com/docs/object-position
constant!(object bottom);
constant!(object center);
constant!(object left);
constant!(object left bottom);
constant!(object left top);
constant!(object right);
constant!(object right bottom);
constant!(object right top);
constant!(object top);

// https://tailwindcss.com/docs/overflow
constant!(overflow auto);
constant!(overflow hidden);
constant!(overflow clip);
constant!(overflow visible);
constant!(overflow scroll);
constant!(overflow x auto);
constant!(overflow y auto);
constant!(overflow x hidden);
constant!(overflow y hidden);
constant!(overflow x clip);
constant!(overflow y clip);
constant!(overflow x visible);
constant!(overflow y visible);
constant!(overflow x scroll);
constant!(overflow y scroll);

// https://tailwindcss.com/docs/overscroll-behavior
constant!(overscroll auto);
constant!(overscroll contain);
constant!(overscroll none);
constant!(overscroll y auto);
constant!(overscroll y contain);
constant!(overscroll y none);
constant!(overscroll x auto);
constant!(overscroll x contain);
constant!(overscroll x none);

// https://tailwindcss.com/docs/position
#[doc = "static"]
pub const static_: &'static str = "static";
constant!(fixed);
constant!(absolute);
constant!(relative);
constant!(sticky);

// https://tailwindcss.com/docs/top-right-bottom-left
constant!(top auto);
size_0_to_96!(top);
fraction_2_to_4!(top);
constant!(top full);

constant!(bottom auto);
size_0_to_96!(bottom);
fraction_2_to_4!(bottom);
constant!(bottom full);

constant!(left auto);
size_0_to_96!(left);
fraction_2_to_4!(left);
constant!(left full);

constant!(right auto);
size_0_to_96!(right);
fraction_2_to_4!(right);
constant!(right full);

constant!(inset auto);
size_0_to_96!(inset);
fraction_2_to_4!(inset);
constant!(inset full);

constant!(inset x auto);
size_0_to_96!(inset x);
fraction_2_to_4!(inset x);
constant!(inset x full);

constant!(inset y auto);
size_0_to_96!(inset y);
fraction_2_to_4!(inset y);
constant!(inset y full);

// https://tailwindcss.com/docs/visibility
constant!(visible);
constant!(invisible);
constant!(collapse);

// https://tailwindcss.com/docs/z-index
constant!(z 0);
constant!(z 10);
constant!(z 20);
constant!(z 30);
constant!(z 40);
constant!(z 50);
constant!(z auto);

any!(z);

minus!(z 10);
minus!(z 20);
minus!(z 30);
minus!(z 40);
minus!(z 50);