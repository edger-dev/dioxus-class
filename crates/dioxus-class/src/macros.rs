#[macro_export]
macro_rules! constant_internal {
    ( $( $part:ident )+ _ $next:ident $( $extra:ident )+  ) => {
        constant_internal!($( $part )* $next _ $( $extra )*);
    };
    ( $( $part:ident )+ _ $last:ident ) => {
        paste!{
            #[doc = concat!($( stringify!($part), "-", )* stringify!($last))]
            pub const [< $( $part _ )* $last >]: &'static str = concat!($( stringify!($part), "-", )* stringify!($last));
        }
    };
    ( $( $part:ident )+ _ $last:literal ) => {
        paste!{
            #[doc = concat!($( stringify!($part), "-", )* $last)]
            pub const [< $( $part _ )* $last >]: &'static str = concat!($( stringify!($part), "-", )* $last);
        }
    };
}

#[macro_export]
macro_rules! constant {
    ( $last:ident ) => {
        #[doc = concat!(stringify!($last))]
        pub const $last: &'static str = concat!(stringify!($last));
    };
    ( $first:ident $( $extra:ident )+  ) => {
        constant_internal!($first _ $( $extra )*);
    };
    ( $( $part:ident )+ $last:literal ) => {
        constant_internal!($( $part )* _ $last);
    };
}
