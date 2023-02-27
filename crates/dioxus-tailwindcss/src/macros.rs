pub use dioxus_class::ext::*;

#[macro_export]
macro_rules! any {
    ( $( $prefix:ident )* ) => {
        paste!{
            pub fn [< $( $prefix _ )* any >](p: &str) -> String {
                format!(concat!($( stringify!($prefix), "-", )* "[{}]"), p)
            }
        }
    }
}

#[macro_export]
macro_rules! minus {
    ( $( $part:ident )+ $last:literal ) => {
        paste!{
            pub const [< minus_ $( $part _ )* $last >]: &'static str = concat!("-", $( stringify!($part), "-", )* stringify!($last));
        }
    }
}

#[macro_export]
macro_rules! colors {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* inherit);
        constant!($( $prefix )* current);
        constant!($( $prefix )* transparent);
        constant!($( $prefix )* black);
        constant!($( $prefix )* white);

        constant!($( $prefix )* slate 50);
        constant!($( $prefix )* slate 100);
        constant!($( $prefix )* slate 200);
        constant!($( $prefix )* slate 300);
        constant!($( $prefix )* slate 400);
        constant!($( $prefix )* slate 500);
        constant!($( $prefix )* slate 600);
        constant!($( $prefix )* slate 700);
        constant!($( $prefix )* slate 800);
        constant!($( $prefix )* slate 900);

        constant!($( $prefix )* gray 50);
        constant!($( $prefix )* gray 100);
        constant!($( $prefix )* gray 200);
        constant!($( $prefix )* gray 300);
        constant!($( $prefix )* gray 400);
        constant!($( $prefix )* gray 500);
        constant!($( $prefix )* gray 600);
        constant!($( $prefix )* gray 700);
        constant!($( $prefix )* gray 800);
        constant!($( $prefix )* gray 900);

        constant!($( $prefix )* zinc 50);
        constant!($( $prefix )* zinc 100);
        constant!($( $prefix )* zinc 200);
        constant!($( $prefix )* zinc 300);
        constant!($( $prefix )* zinc 400);
        constant!($( $prefix )* zinc 500);
        constant!($( $prefix )* zinc 600);
        constant!($( $prefix )* zinc 700);
        constant!($( $prefix )* zinc 800);
        constant!($( $prefix )* zinc 900);

        constant!($( $prefix )* neutral 50);
        constant!($( $prefix )* neutral 100);
        constant!($( $prefix )* neutral 200);
        constant!($( $prefix )* neutral 300);
        constant!($( $prefix )* neutral 400);
        constant!($( $prefix )* neutral 500);
        constant!($( $prefix )* neutral 600);
        constant!($( $prefix )* neutral 700);
        constant!($( $prefix )* neutral 800);
        constant!($( $prefix )* neutral 900);

        constant!($( $prefix )* stone 50);
        constant!($( $prefix )* stone 100);
        constant!($( $prefix )* stone 200);
        constant!($( $prefix )* stone 300);
        constant!($( $prefix )* stone 400);
        constant!($( $prefix )* stone 500);
        constant!($( $prefix )* stone 600);
        constant!($( $prefix )* stone 700);
        constant!($( $prefix )* stone 800);
        constant!($( $prefix )* stone 900);

        constant!($( $prefix )* red 50);
        constant!($( $prefix )* red 100);
        constant!($( $prefix )* red 200);
        constant!($( $prefix )* red 300);
        constant!($( $prefix )* red 400);
        constant!($( $prefix )* red 500);
        constant!($( $prefix )* red 600);
        constant!($( $prefix )* red 700);
        constant!($( $prefix )* red 800);
        constant!($( $prefix )* red 900);

        constant!($( $prefix )* orange 50);
        constant!($( $prefix )* orange 100);
        constant!($( $prefix )* orange 200);
        constant!($( $prefix )* orange 300);
        constant!($( $prefix )* orange 400);
        constant!($( $prefix )* orange 500);
        constant!($( $prefix )* orange 600);
        constant!($( $prefix )* orange 700);
        constant!($( $prefix )* orange 800);
        constant!($( $prefix )* orange 900);

        constant!($( $prefix )* amber 50);
        constant!($( $prefix )* amber 100);
        constant!($( $prefix )* amber 200);
        constant!($( $prefix )* amber 300);
        constant!($( $prefix )* amber 400);
        constant!($( $prefix )* amber 500);
        constant!($( $prefix )* amber 600);
        constant!($( $prefix )* amber 700);
        constant!($( $prefix )* amber 800);
        constant!($( $prefix )* amber 900);

        constant!($( $prefix )* yellow 50);
        constant!($( $prefix )* yellow 100);
        constant!($( $prefix )* yellow 200);
        constant!($( $prefix )* yellow 300);
        constant!($( $prefix )* yellow 400);
        constant!($( $prefix )* yellow 500);
        constant!($( $prefix )* yellow 600);
        constant!($( $prefix )* yellow 700);
        constant!($( $prefix )* yellow 800);
        constant!($( $prefix )* yellow 900);

        constant!($( $prefix )* lime 50);
        constant!($( $prefix )* lime 100);
        constant!($( $prefix )* lime 200);
        constant!($( $prefix )* lime 300);
        constant!($( $prefix )* lime 400);
        constant!($( $prefix )* lime 500);
        constant!($( $prefix )* lime 600);
        constant!($( $prefix )* lime 700);
        constant!($( $prefix )* lime 800);
        constant!($( $prefix )* lime 900);

        constant!($( $prefix )* green 50);
        constant!($( $prefix )* green 100);
        constant!($( $prefix )* green 200);
        constant!($( $prefix )* green 300);
        constant!($( $prefix )* green 400);
        constant!($( $prefix )* green 500);
        constant!($( $prefix )* green 600);
        constant!($( $prefix )* green 700);
        constant!($( $prefix )* green 800);
        constant!($( $prefix )* green 900);

        constant!($( $prefix )* emerald 50);
        constant!($( $prefix )* emerald 100);
        constant!($( $prefix )* emerald 200);
        constant!($( $prefix )* emerald 300);
        constant!($( $prefix )* emerald 400);
        constant!($( $prefix )* emerald 500);
        constant!($( $prefix )* emerald 600);
        constant!($( $prefix )* emerald 700);
        constant!($( $prefix )* emerald 800);
        constant!($( $prefix )* emerald 900);

        constant!($( $prefix )* teal 50);
        constant!($( $prefix )* teal 100);
        constant!($( $prefix )* teal 200);
        constant!($( $prefix )* teal 300);
        constant!($( $prefix )* teal 400);
        constant!($( $prefix )* teal 500);
        constant!($( $prefix )* teal 600);
        constant!($( $prefix )* teal 700);
        constant!($( $prefix )* teal 800);
        constant!($( $prefix )* teal 900);

        constant!($( $prefix )* cyan 50);
        constant!($( $prefix )* cyan 100);
        constant!($( $prefix )* cyan 200);
        constant!($( $prefix )* cyan 300);
        constant!($( $prefix )* cyan 400);
        constant!($( $prefix )* cyan 500);
        constant!($( $prefix )* cyan 600);
        constant!($( $prefix )* cyan 700);
        constant!($( $prefix )* cyan 800);
        constant!($( $prefix )* cyan 900);

        constant!($( $prefix )* sky 50);
        constant!($( $prefix )* sky 100);
        constant!($( $prefix )* sky 200);
        constant!($( $prefix )* sky 300);
        constant!($( $prefix )* sky 400);
        constant!($( $prefix )* sky 500);
        constant!($( $prefix )* sky 600);
        constant!($( $prefix )* sky 700);
        constant!($( $prefix )* sky 800);
        constant!($( $prefix )* sky 900);

        constant!($( $prefix )* blue 50);
        constant!($( $prefix )* blue 100);
        constant!($( $prefix )* blue 200);
        constant!($( $prefix )* blue 300);
        constant!($( $prefix )* blue 400);
        constant!($( $prefix )* blue 500);
        constant!($( $prefix )* blue 600);
        constant!($( $prefix )* blue 700);
        constant!($( $prefix )* blue 800);
        constant!($( $prefix )* blue 900);

        constant!($( $prefix )* indigo 50);
        constant!($( $prefix )* indigo 100);
        constant!($( $prefix )* indigo 200);
        constant!($( $prefix )* indigo 300);
        constant!($( $prefix )* indigo 400);
        constant!($( $prefix )* indigo 500);
        constant!($( $prefix )* indigo 600);
        constant!($( $prefix )* indigo 700);
        constant!($( $prefix )* indigo 800);
        constant!($( $prefix )* indigo 900);

        constant!($( $prefix )* violet 50);
        constant!($( $prefix )* violet 100);
        constant!($( $prefix )* violet 200);
        constant!($( $prefix )* violet 300);
        constant!($( $prefix )* violet 400);
        constant!($( $prefix )* violet 500);
        constant!($( $prefix )* violet 600);
        constant!($( $prefix )* violet 700);
        constant!($( $prefix )* violet 800);
        constant!($( $prefix )* violet 900);

        constant!($( $prefix )* purple 50);
        constant!($( $prefix )* purple 100);
        constant!($( $prefix )* purple 200);
        constant!($( $prefix )* purple 300);
        constant!($( $prefix )* purple 400);
        constant!($( $prefix )* purple 500);
        constant!($( $prefix )* purple 600);
        constant!($( $prefix )* purple 700);
        constant!($( $prefix )* purple 800);
        constant!($( $prefix )* purple 900);

        constant!($( $prefix )* fuchsia 50);
        constant!($( $prefix )* fuchsia 100);
        constant!($( $prefix )* fuchsia 200);
        constant!($( $prefix )* fuchsia 300);
        constant!($( $prefix )* fuchsia 400);
        constant!($( $prefix )* fuchsia 500);
        constant!($( $prefix )* fuchsia 600);
        constant!($( $prefix )* fuchsia 700);
        constant!($( $prefix )* fuchsia 800);
        constant!($( $prefix )* fuchsia 900);

        constant!($( $prefix )* pink 50);
        constant!($( $prefix )* pink 100);
        constant!($( $prefix )* pink 200);
        constant!($( $prefix )* pink 300);
        constant!($( $prefix )* pink 400);
        constant!($( $prefix )* pink 500);
        constant!($( $prefix )* pink 600);
        constant!($( $prefix )* pink 700);
        constant!($( $prefix )* pink 800);
        constant!($( $prefix )* pink 900);

        constant!($( $prefix )* rose 50);
        constant!($( $prefix )* rose 100);
        constant!($( $prefix )* rose 200);
        constant!($( $prefix )* rose 300);
        constant!($( $prefix )* rose 400);
        constant!($( $prefix )* rose 500);
        constant!($( $prefix )* rose 600);
        constant!($( $prefix )* rose 700);
        constant!($( $prefix )* rose 800);
        constant!($( $prefix )* rose 900);

        crate::any!($( $prefix )*);
    }
}

#[macro_export]
macro_rules! blend {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* normal);
        constant!($( $prefix )* multiply);
        constant!($( $prefix )* screen);
        constant!($( $prefix )* overlay);
        constant!($( $prefix )* darken);
        constant!($( $prefix )* lighten);
        constant!($( $prefix )* color dodge);
        constant!($( $prefix )* color burn);
        constant!($( $prefix )* hard light);
        constant!($( $prefix )* soft light);
        constant!($( $prefix )* difference);
        constant!($( $prefix )* exclusion);
        constant!($( $prefix )* hue);
        constant!($( $prefix )* saturation);
        constant!($( $prefix )* color);
        constant!($( $prefix )* luminosity);
    }
}

#[macro_export]
macro_rules! _1_to_6 {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* 1);
        constant!($( $prefix )* 2);
        constant!($( $prefix )* 3);
        constant!($( $prefix )* 4);
        constant!($( $prefix )* 5);
        constant!($( $prefix )* 6);
    }
}

#[macro_export]
macro_rules! _1_to_12 {
    ( $( $prefix:ident )* ) => {
        crate::_1_to_6!($( $prefix )* );
        constant!($( $prefix )* 7);
        constant!($( $prefix )* 8);
        constant!($( $prefix )* 9);
        constant!($( $prefix )* 10);
        constant!($( $prefix )* 11);
        constant!($( $prefix )* 12);
    }
}

#[macro_export]
macro_rules! size_0_to_96 {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* 0);
        constant!($( $prefix )* px);
        paste!{
            pub const [< $( $prefix _ )* _half >]: &'static str = concat!($( stringify!($prefix), "-", )* "0.5");
        }
        paste!{
            pub const [< $( $prefix _ )* 1_half >]: &'static str = concat!($( stringify!($prefix), "-", )* "1.5");
        }
        paste!{
            pub const [< $( $prefix _ )* 2_half >]: &'static str = concat!($( stringify!($prefix), "-", )* "2.5");
        }
        paste!{
            pub const [< $( $prefix _ )* 3_half >]: &'static str = concat!($( stringify!($prefix), "-", )* "3.5");
        }
        crate::_1_to_12!($( $prefix )* );
        constant!($( $prefix )* 16);
        constant!($( $prefix )* 20);
        constant!($( $prefix )* 24);
        constant!($( $prefix )* 28);
        constant!($( $prefix )* 32);
        constant!($( $prefix )* 36);
        constant!($( $prefix )* 40);
        constant!($( $prefix )* 44);
        constant!($( $prefix )* 48);
        constant!($( $prefix )* 52);
        constant!($( $prefix )* 56);
        constant!($( $prefix )* 60);
        constant!($( $prefix )* 64);
        constant!($( $prefix )* 72);
        constant!($( $prefix )* 80);
        constant!($( $prefix )* 96);

        crate::any!($( $prefix )*);
    }
}

#[macro_export]
macro_rules! fraction_2_to_4 {
    ( $( $prefix:ident )* ) => {
        paste!{
            pub const [< $( $prefix _ )* 1__2 >]: &'static str = concat!($( stringify!($prefix), "-", )* "1/2");
            pub const [< $( $prefix _ )* 1__3 >]: &'static str = concat!($( stringify!($prefix), "-", )* "1/3");
            pub const [< $( $prefix _ )* 2__3 >]: &'static str = concat!($( stringify!($prefix), "-", )* "2/3");
            pub const [< $( $prefix _ )* 1__4 >]: &'static str = concat!($( stringify!($prefix), "-", )* "1/4");
            pub const [< $( $prefix _ )* 2__4 >]: &'static str = concat!($( stringify!($prefix), "-", )* "2/4");
            pub const [< $( $prefix _ )* 3__4 >]: &'static str = concat!($( stringify!($prefix), "-", )* "3/4");
        }
    }
}

#[macro_export]
macro_rules! fraction_2_to_6 {
    ( $( $prefix:ident )* ) => {
        crate::fraction_2_to_4!($( $prefix )*);
        paste!{
            pub const [< $( $prefix _ )* 1__5 >]: &'static str = concat!($( stringify!($prefix), "-", )* "1/5");
            pub const [< $( $prefix _ )* 2__5 >]: &'static str = concat!($( stringify!($prefix), "-", )* "2/5");
            pub const [< $( $prefix _ )* 3__5 >]: &'static str = concat!($( stringify!($prefix), "-", )* "3/5");
            pub const [< $( $prefix _ )* 4__5 >]: &'static str = concat!($( stringify!($prefix), "-", )* "4/5");
            pub const [< $( $prefix _ )* 1__6 >]: &'static str = concat!($( stringify!($prefix), "-", )* "1/6");
            pub const [< $( $prefix _ )* 2__6 >]: &'static str = concat!($( stringify!($prefix), "-", )* "2/6");
            pub const [< $( $prefix _ )* 3__6 >]: &'static str = concat!($( stringify!($prefix), "-", )* "3/6");
            pub const [< $( $prefix _ )* 4__6 >]: &'static str = concat!($( stringify!($prefix), "-", )* "4/6");
            pub const [< $( $prefix _ )* 5__6 >]: &'static str = concat!($( stringify!($prefix), "-", )* "5/6");
        }
    }
}

#[macro_export]
macro_rules! fraction_12 {
    ( $( $prefix:ident )* ) => {
        paste!{
            pub const [< $( $prefix _ )* 1__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "1/12");
            pub const [< $( $prefix _ )* 2__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "2/12");
            pub const [< $( $prefix _ )* 3__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "3/12");
            pub const [< $( $prefix _ )* 4__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "4/12");
            pub const [< $( $prefix _ )* 5__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "5/12");
            pub const [< $( $prefix _ )* 6__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "6/12");
            pub const [< $( $prefix _ )* 7__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "7/12");
            pub const [< $( $prefix _ )* 8__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "8/12");
            pub const [< $( $prefix _ )* 9__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "9/12");
            pub const [< $( $prefix _ )* 10__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "10/12");
            pub const [< $( $prefix _ )* 11__12 >]: &'static str = concat!($( stringify!($prefix), "-", )* "11/12");
        }
    }
}

#[macro_export]
macro_rules! sm_to_2xl {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* sm);
        constant!($( $prefix )* md);
        constant!($( $prefix )* lg);
        constant!($( $prefix )* xl);
        constant!($( $prefix )* 2xl);
    }
}

#[macro_export]
macro_rules! xs_to_7xl {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* xs);
        crate::sm_to_2xl!($( $prefix )*);
        constant!($( $prefix )* 3xl);
        constant!($( $prefix )* 4xl);
        constant!($( $prefix )* 5xl);
        constant!($( $prefix )* 6xl);
        constant!($( $prefix )* 7xl);
    }
}

#[macro_export]
macro_rules! _3xs_to_7xl {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* 3xs);
        constant!($( $prefix )* 2xs);
        crate::xs_to_7xl!($( $prefix )*);
    }
}

#[macro_export]
macro_rules! opacities {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* 0);
        constant!($( $prefix )* 5);
        constant!($( $prefix )* 10);
        constant!($( $prefix )* 20);
        constant!($( $prefix )* 25);
        constant!($( $prefix )* 30);
        constant!($( $prefix )* 40);
        constant!($( $prefix )* 50);
        constant!($( $prefix )* 60);
        constant!($( $prefix )* 70);
        constant!($( $prefix )* 75);
        constant!($( $prefix )* 80);
        constant!($( $prefix )* 90);
        constant!($( $prefix )* 95);
        constant!($( $prefix )* 100);

        crate::any!($( $prefix )*);
    }
}

#[macro_export]
macro_rules! scales {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* 0);
        constant!($( $prefix )* 50);
        constant!($( $prefix )* 75);
        constant!($( $prefix )* 90);
        constant!($( $prefix )* 95);
        constant!($( $prefix )* 100);
        constant!($( $prefix )* 105);
        constant!($( $prefix )* 110);
        constant!($( $prefix )* 125);
        constant!($( $prefix )* 150);

        crate::any!($( $prefix )*);
    }
}

