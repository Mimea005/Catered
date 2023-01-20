use serde::Serialize;


#[derive(Serialize, Clone)]
pub struct Dish<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub price: (isize, usize),
    pub publisher: &'a str
}