pub use paste::paste;

#[macro_export]
macro_rules! constant_internal {
    ( $( $part:ident )+ _ $next:ident $( $extra:ident )+  ) => {
        constant_internal!($( $part )* $next _ $( $extra )*);
    };
    ( $( $part:ident )+ _ $last:ident ) => {
        paste!{
            pub const [< $( $part _ )* $last >]: &'static str = concat!($( stringify!($part), "-", )* stringify!($last));
        }
    };
    ( $( $part:ident )+ _ $last:literal ) => {
        paste!{
            pub const [< $( $part _ )* $last >]: &'static str = concat!($( stringify!($part), "-", )* stringify!($last));
        }
    };
}

#[macro_export]
macro_rules! constant {
    ( $last:ident ) => {
        pub const $last: &'static str = concat!(stringify!($last));
    };
    ( $first:ident $( $extra:ident )+  ) => {
        constant_internal!($first _ $( $extra )*);
    };
    ( $( $part:ident )+ $last:literal ) => {
        constant_internal!($( $part )* _ $last);
    };
}

#[macro_export]
macro_rules! class {
    ( $( $t:tt )* ) => {
        Class::from(vec![ $( $t )* ])
    }
}

#[macro_export]
macro_rules! style_type {
    ( $type:ident, $( $key:ident: $val:expr, )* ) => {
        #[derive(Clone, Debug)]
        pub struct $type {
        $(
            pub $key: Class,
        )*
        }
        impl Default for $type {
            fn default() -> Self {
                Self {
                $(
                    $key: $val,
                )*
                }
            }
        }
        impl Into<Classes> for $type {
            fn into(self) -> Classes {
                Classes {
                    name: format!("{}::$type", module_path!()),
                    classes: vec![
                    $(
                        ($key.to_string(), self.$key.clone()),
                    )*
                    ]
                }
            }
        }
    }
}

#[macro_export]
macro_rules! style {
    ( $( $key:ident: $val:expr, )* ) => {
        style_type!{Style, $( $key: $val, )*}
    }
}