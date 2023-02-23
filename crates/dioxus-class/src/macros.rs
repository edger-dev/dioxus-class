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
