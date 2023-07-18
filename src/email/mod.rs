use crate::config_parser::Email;
use crate::CONFIGS;

use lettre::message::{header::ContentType, MultiPart};
use lettre::message::{Attachment, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::fs;

pub struct EmailHandler<'a> {
    email_configs: &'a Email,
    subject: String,
    message: String,
    attachment: Option<SinglePart>,
}

impl EmailHandler<'_> {
    pub fn new(subject: &str, message: &str) -> Self {
        Self {
            email_configs: &CONFIGS.email,
            subject: String::from(subject),
            message: String::from(message),
            attachment: None,
        }
    }

    pub fn add_attachment(&mut self, filename: &str, content_type: &str) {
        let contents = fs::read(filename).expect("Something went wrong reading the file");
        let content_type = ContentType::parse(content_type).unwrap();
        self.attachment =
            Some(Attachment::new(String::from(filename)).body(contents, content_type));
    }

    fn build_multipart(&self) -> MultiPart {
        let mutli_part = MultiPart::mixed().singlepart(
            lettre::message::SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body(self.message.clone()),
        );

        match &self.attachment {
            Some(attachment) => mutli_part.singlepart(attachment.clone()),
            None => mutli_part,
        }
    }

    fn build_email(&self) -> Message {
        Message::builder()
            .from(self.email_configs.email.parse().unwrap())
            .to(self.email_configs.recipient.parse().unwrap())
            .subject(self.subject.clone())
            .multipart(self.build_multipart())
            .unwrap()
    }

    pub fn send_email(&self) {
        let credentials = Credentials::new(
            self.email_configs.email.to_string(),
            self.email_configs.password.to_string(),
        );

        let sender = SmtpTransport::relay(&self.email_configs.smtp_server)
            .unwrap()
            .credentials(credentials)
            .port(self.email_configs.smtp_port)
            .build();

        match sender.send(&self.build_email()) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
