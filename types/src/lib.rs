use serde::Serialize;


#[derive(Serialize, Clone)]
pub struct Dish<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub price: (isize, usize),
    pub publisher: &'a str
}

pub struct User<'a> {
    pub name: &'a str,
    pub role: Role
}

/// The role of a user describes the relation between the
/// service and the user e.g
/// Customer: is a customer, can only by and contact
/// Regular: same as a customer with extra benefits
/// Admin: administrator, can add, edit, remove and communicate with costumers
pub enum Role {
    Customer,
    Regular(isize),
    Admin
}