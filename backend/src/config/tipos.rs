use serde::Serialize;

#[derive(Serialize)]
pub enum Respuesta {
    Success,
    Error,
    Warn,
    Info,
}

impl Respuesta {
    pub fn as_str(&self) -> &'static str {
        match self {
            Respuesta::Success => "Éxito",
            Respuesta::Error => "Error",
            Respuesta::Warn => "Atención",
            Respuesta::Info => "Información",
        }
    }
}

#[derive(Serialize)]
pub struct Ress<T> {
    pub message: &'static str,
    pub description: &'static str,
    pub data: Option<T>,
}
