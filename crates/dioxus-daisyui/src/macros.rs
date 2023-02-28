#[macro_export]
macro_rules! semantic_colors {
    ( $( $prefix:ident )* ) => {
        constant!($( $prefix )* primary);
        constant!($( $prefix )* primary focus);
        constant!($( $prefix )* primary content);

        constant!($( $prefix )* secondary);
        constant!($( $prefix )* secondary focus);
        constant!($( $prefix )* secondary content);

        constant!($( $prefix )* accent);
        constant!($( $prefix )* accent focus);
        constant!($( $prefix )* accent content);

        constant!($( $prefix )* neutral);
        constant!($( $prefix )* neutral focus);
        constant!($( $prefix )* neutral content);

        constant!($( $prefix )* base 100);
        constant!($( $prefix )* base 200);
        constant!($( $prefix )* base 300);
        constant!($( $prefix )* base content);

        constant!($( $prefix )* info);
        constant!($( $prefix )* info content);

        constant!($( $prefix )* success);
        constant!($( $prefix )* success content);

        constant!($( $prefix )* warning);
        constant!($( $prefix )* warning content);

        constant!($( $prefix )* error);
        constant!($( $prefix )* error content);
    }
}

