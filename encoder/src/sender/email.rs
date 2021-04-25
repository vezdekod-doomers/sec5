use crate::sender::Sender;
use std::error::Error;
use lettre::{SmtpTransport, Transport, Message};
use lettre::transport::smtp::authentication::Credentials;

struct EmailSenderInternal {
    transport: SmtpTransport,
    from: String,
}

pub struct EmailSender {
    internal: Option<EmailSenderInternal>,
}

impl EmailSender {
    pub fn new() -> EmailSender {
        EmailSender { internal: None }
    }
}

impl Sender for EmailSender {
    fn init(&mut self) {
        println!("Enter SMTP server url");
        let smtp = crate::helper::read_line();
        if smtp.is_empty() {
            println!("E-mail sender is NOT initialized");
            return;
        }
        println!("Enter SMTP username");
        let smtp_username = crate::helper::read_line();
        println!("Enter SMTP password");
        let smtp_password = crate::helper::read_line();
        let creds = Credentials::new(smtp_username.clone(), smtp_password);

        let mailer = SmtpTransport::relay(&smtp)
            .unwrap()
            .credentials(creds)
            .build();

        println!("Enter 'From' header value ({})", smtp_username);
        let mut from = crate::helper::read_line();
        if from.is_empty() {
            from = smtp_username
        }
        self.internal = Some(EmailSenderInternal {
            from,
            transport: mailer
        })
    }

    fn send(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        let internal = self.internal.as_ref().unwrap();
        println!("Enter recipient E-mail");
        let to = crate::helper::read_line();
        let email = Message::builder()
            .from(internal.from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject("The Encrypted Message")
            .body(msg.to_owned())
            .unwrap();

        internal.transport.send(&email)?;
        Ok(())
    }

    fn name(&self) -> &'static str {
        "email"
    }
}