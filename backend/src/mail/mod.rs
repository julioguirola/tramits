use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::error;

use crate::config::EnvConfig;

pub enum EmailType {
    TramiteCompletado,
    TramiteRechazado,
    MailWithBody(String, String),
}

const COLOR_BACKGROUND: &str = "oklch(1 0 0)";
const COLOR_FOREGROUND: &str = "oklch(0.141 0.005 285.823)";
const COLOR_PRIMARY: &str = "oklch(0.648 0.2 131.684)";
const COLOR_PRIMARY_FOREGROUND: &str = "oklch(0.986 0.031 120.757)";
const COLOR_MUTED: &str = "oklch(0.967 0.001 286.375)";
const COLOR_MUTED_FOREGROUND: &str = "oklch(0.552 0.016 285.938)";
const COLOR_BORDER: &str = "oklch(0.92 0.004 286.32)";
const COLOR_DESTRUCTIVE: &str = "oklch(0.577 0.245 27.325)";

fn email_layout(
    title: &str,
    subtitle: &str,
    status: &str,
    status_color: &str,
    body: &str,
) -> String {
    format!(
        r#"<!doctype html>
<html lang="es">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <title>{title}</title>
  </head>
  <body style="margin:0;background:{background};font-family:'Plus Jakarta Sans','Trebuchet MS',sans-serif;color:{foreground};">
    <span style="display:none;opacity:0;visibility:hidden;height:0;width:0;">{subtitle}</span>
    <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="background:{background};padding:32px 16px;">
      <tr>
        <td align="center">
          <table role="presentation" cellpadding="0" cellspacing="0" width="100%" style="max-width:600px;border:1px solid {border};border-radius:14px;overflow:hidden;background:{muted};">
            <tr>
              <td style="padding:24px 28px;background:{background};">
                <div style="font-family:'Fraunces','Times New Roman',serif;font-size:20px;font-weight:700;color:{foreground};">Tramits · MINCIN</div>
                <div style="margin-top:8px;font-size:24px;font-weight:700;color:{foreground};">{title}</div>
                <div style="margin-top:6px;font-size:14px;color:{muted_foreground};">{subtitle}</div>
              </td>
            </tr>
            <tr>
              <td style="padding:0 28px 20px 28px;background:{background};">
                <div style="display:inline-block;padding:6px 12px;border-radius:999px;background:{status_color};color:{status_text_color};font-size:12px;font-weight:600;letter-spacing:0.3px;">{status}</div>
              </td>
            </tr>
            <tr>
              <td style="padding:0 28px 24px 28px;background:{background};font-size:15px;line-height:1.6;color:{foreground};">
                {body}
              </td>
            </tr>
            <tr>
              <td style="padding:20px 28px;border-top:1px solid {border};background:{muted};font-size:12px;color:{muted_foreground};">
                Este correo es informativo. Si tienes dudas, contacta a tu oficina de registro.
              </td>
            </tr>
          </table>
        </td>
      </tr>
    </table>
  </body>
</html>"#,
        title = title,
        subtitle = subtitle,
        status = status,
        status_color = status_color,
        status_text_color = COLOR_PRIMARY_FOREGROUND,
        body = body,
        background = COLOR_BACKGROUND,
        foreground = COLOR_FOREGROUND,
        muted = COLOR_MUTED,
        muted_foreground = COLOR_MUTED_FOREGROUND,
        border = COLOR_BORDER,
    )
}

fn tramite_completado(titulo: &str, detalle: &str) -> String {
    let cuerpo = format!(
        "<p>Tu solicitud fue <strong>completada</strong>.</p><p>{}</p>",
        detalle
    );
    email_layout(
        titulo,
        "Resultado final de tu solicitud",
        "COMPLETADO",
        COLOR_PRIMARY,
        &cuerpo,
    )
}

fn tramite_rechazado(titulo: &str, detalle: &str) -> String {
    let cuerpo = format!(
        "<p>Tu solicitud fue <strong>rechazada</strong>.</p><p>{}</p>",
        detalle
    );
    email_layout(
        titulo,
        "Resultado final de tu solicitud",
        "RECHAZADO",
        COLOR_DESTRUCTIVE,
        &cuerpo,
    )
}

pub fn send_email(
    from_n: String,
    from: String,
    to_n: String,
    to: String,
    tipo: EmailType,
    env_config: &EnvConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let body = match &tipo {
        EmailType::TramiteCompletado => tramite_completado(
            "Trámite completado",
            "Tu solicitud fue aprobada y registrada en el sistema.",
        ),
        EmailType::TramiteRechazado => tramite_rechazado(
            "Trámite rechazado",
            "Revisa los datos enviados y vuelve a intentarlo si es necesario.",
        ),
        EmailType::MailWithBody(_, raw_body) => email_layout(
            "Mensaje de Tramits",
            "Notificación del sistema",
            "INFORMACIÓN",
            COLOR_PRIMARY,
            raw_body,
        ),
    };

    let subject = match &tipo {
        EmailType::TramiteCompletado => "Trámite completado",
        EmailType::TramiteRechazado => "Trámite rechazado",
        EmailType::MailWithBody(sub, _) => sub.as_str(),
    };

    let email = Message::builder()
        .from(Mailbox::new(Some(from_n), from.parse()?))
        .to(Mailbox::new(Some(to_n), to.parse()?))
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)?;

    let creds = Credentials::new(
        env_config.mail_user.clone(),
        env_config.mail_password.clone(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::from_url(&env_config.mail_url)?
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
