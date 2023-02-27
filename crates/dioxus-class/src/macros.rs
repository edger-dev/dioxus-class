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
    ( $( $t:expr )* ) => {
        {
            let mut __class_values__: Vec<String> = Vec::new();
            /*
            let mut index: usize = 0;
            println!("{}:{}", file!(), line!());
             */
            $(
                __class_values__.push(String::from($t));
            /*
                index += 1;
                println!("{}:{} [{}] {} -> {}", file!(), line!(), index, stringify!($t), String::from($t));
             */
            )*
            Class::from(__class_values__)
        }
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
                    name: format!("{}::{}", module_path!(), stringify!($type)),
                    classes: vec![
                    $(
                        (stringify!($key).to_owned(), self.$key.clone()),
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