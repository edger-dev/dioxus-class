use crate::class::Class;

#[derive(Clone, Debug)]
pub struct Classes {
    pub name: String,
    pub classes: Vec<(String, Class)>,
}