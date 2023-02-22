#[macro_export]
macro_rules! class {
    ( $( $t:tt )* ) => {
        Class::from(vec![ $( $t )* ])
    }
}