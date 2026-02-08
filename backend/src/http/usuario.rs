use crate::models::usuario::UsuarioService;

pub struct UsuarioController<'a> {
    usuario_serv: &'a UsuarioService<'a>
}

impl UsuarioController<'_> {
    pub fn new<'a> (usuario_serv: &'a UsuarioService<'a>) -> UsuarioController<'a> {
        UsuarioController {usuario_serv}
    }
}