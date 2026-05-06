use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::error;

pub enum EmailType {
    TtramiteAltaCompletado,
    TtramiteAltaRechazado,
    TtramiteBajaCompletado,
    TtramiteBajaRechazado,
    TramiteLibretaCompletado,
    TramiteLibretaRechazado,
    MailWithBody(String, String),
}

pub fn send_email(
    from_n: String,
    from: String,
    to_n: String,
    to: String,
    tipo: EmailType,
    env_config: &crate::config::EnvConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(Mailbox::new(Some(from_n), from.parse()?))
        .to(Mailbox::new(Some(to_n), to.parse()?))
        .subject(match tipo {
            EmailType::TtramiteAltaCompletado => "Trámite de alta completado",
            EmailType::TtramiteAltaRechazado => "Trámite de alta rechazado",
            EmailType::TtramiteBajaCompletado => "Trámite de baja completado",
            EmailType::TtramiteBajaRechazado => "Trámite de baja rechazado",
            EmailType::TramiteLibretaRechazado => "Trámite de libreta sanitaria rechazado",
            EmailType::TramiteLibretaCompletado => "Trámite de libreta sanitaria completado",
            EmailType::MailWithBody(sub, _) => &sub,
        })
        .header(ContentType::TEXT_HTML)
        .body(String::from(match tipo {
            EmailType::TtramiteAltaCompletado => body,
            EmailType::TtramiteAltaRechazado => body,
            EmailType::TtramiteBajaCompletado => body,
            EmailType::TtramiteBajaRechazado => body,
            EmailType::TramiteLibretaCompletado => body,
            EmailType::TramiteLibretaRechazado => body,
            EmailType::MailWithBody(_, body) => body,
        }))?;

    let creds = Credentials::new(
        env_config.mail_user.clone(),
        env_config.mail_password.clone(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&env_config.mail_url)?
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Could not send email: {:?}", e);
            Err(Box::new(e))
        }
    }
}
