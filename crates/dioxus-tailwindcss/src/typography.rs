use crate::ext::*;

// https://tailwindcss.com/docs/font-family
constant!(font sans);
constant!(font serif);
constant!(font mono);

any!(font);

// https://tailwindcss.com/docs/font-size
constant!(text xs);
constant!(text sm);
constant!(text base);
constant!(text lg);
constant!(text xl);
constant!(text 2xl);
constant!(text 3xl);
constant!(text 4xl);
constant!(text 5xl);
constant!(text 6xl);
constant!(text 7xl);
constant!(text 8xl);
constant!(text 9xl);

// https://tailwindcss.com/docs/font-smoothing
constant!(antialiased);
constant!(subpixel antialiased);

// https://tailwindcss.com/docs/font-style
constant!(italic);
constant!(not italic);

// https://tailwindcss.com/docs/font-weight
constant!(font thin);
constant!(font extralight);
constant!(font light);
constant!(font normal);
constant!(font medium);
constant!(font semibold);
constant!(font bold);
constant!(font extrabold);
constant!(font black);

// https://tailwindcss.com/docs/font-variant-numeric
constant!(normal nums);
constant!(ordinal);
constant!(slashed zero);
constant!(lining nums);
constant!(oldstyle nums);
constant!(proportional nums);
constant!(tabular nums);
constant!(diagonal fractions);
constant!(stacked fractions);

// https://tailwindcss.com/docs/letter-spacing
constant!(tracking tighter);
constant!(tracking tight);
constant!(tracking normal);
constant!(tracking wide);
constant!(tracking wider);
constant!(tracking widest);

any!(tracking);

// https://tailwindcss.com/docs/line-height
constant!(leading 3);
constant!(leading 4);
constant!(leading 5);
constant!(leading 6);
constant!(leading 7);
constant!(leading 8);
constant!(leading 9);
constant!(leading 10);
constant!(leading none);
constant!(leading tight);
constant!(leading snug);
constant!(leading normal);
constant!(leading relaxed);
constant!(leading loose);

any!(leading);

// https://tailwindcss.com/docs/list-style-type
constant!(list none);
constant!(list disc);
constant!(list decimal);

any!(list);

// https://tailwindcss.com/docs/list-style-position
constant!(list inside);
constant!(list outside);

// https://tailwindcss.com/docs/text-align
constant!(text left);
constant!(text center);
constant!(text right);
constant!(text justify);
constant!(text start);
constant!(text end);

// https://tailwindcss.com/docs/text-color
colors!(text);

pub fn with_opacity(text: &str, o: u8) -> String {
    format!("{}/{}", text, o)
}

pub fn with_opacity_scale(text: &str, o: f32) -> String {
    format!("{}/[{}]", text, o)
}

// https://tailwindcss.com/docs/text-decoration
constant!(underline);
constant!(overline);
constant!(line through);
constant!(no underline);

// https://tailwindcss.com/docs/text-decoration-color
colors!(decoration);

// https://tailwindcss.com/docs/text-decoration-style
constant!(decoration solid);
constant!(decoration double);
constant!(decoration dotted);
constant!(decoration dashed);
constant!(decoration wavy);

// https://tailwindcss.com/docs/text-decoration-thickness
constant!(decoration auto);
constant!(decoration from font);
constant!(decoration 0);
constant!(decoration 1);
constant!(decoration 2);
constant!(decoration 4);
constant!(decoration 8);

// https://tailwindcss.com/docs/text-underline-offset
constant!(underline offset auto);
constant!(underline offset 0);
constant!(underline offset 1);
constant!(underline offset 2);
constant!(underline offset 4);
constant!(underline offset 8);

any!(underline offset);

// https://tailwindcss.com/docs/text-transform
constant!(uppercase);
constant!(lowercase);
constant!(capitalize);
constant!(normal case);

// https://tailwindcss.com/docs/text-overflow
constant!(truncate);
constant!(text ellipsis);
constant!(text clip);

// https://tailwindcss.com/docs/text-indent
size_0_to_96!(indent);

// https://tailwindcss.com/docs/vertical-align
constant!(align baseline);
constant!(align top);
constant!(align middle);
constant!(align bottom);
constant!(align text top);
constant!(align text bottom);
constant!(align sub);
constant!(align super);

any!(align);

// https://tailwindcss.com/docs/whitespace
constant!(whitespace normal);
constant!(whitespace nowrap);
constant!(whitespace pre);
constant!(whitespace pre line);
constant!(whitespace pre wrap);

// https://tailwindcss.com/docs/word-break
constant!(break normal);
constant!(break words);
constant!(break all);
constant!(break keep);

// https://tailwindcss.com/docs/content
constant!(content none);

any!(content);