use serde::Serialize;

#[derive(Serialize)]
pub struct Ress<T> {
    pub message: &'static str,
    pub description: &'static str,
    pub data: Option<T>,
}
