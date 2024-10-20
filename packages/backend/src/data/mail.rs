use std::collections::HashMap;

use mailgun_rs::{EmailAddress, Mailgun, MailgunRegion, Message, SendResponse, SendResult};

use crate::app_config::AppConfig;

pub struct Mailer {
    pub api_key: String,
    pub domain: String,
}

pub struct SendInstruction {
    pub template_key: String,
    pub to: String,
    pub vars: HashMap<String, String>,
    pub subject: String,
}

impl Mailer {
    pub fn client(config: &AppConfig) -> Self {
        Mailer {
            api_key: config.mailgun_api_key.clone(),
            domain: config.mailgun_domain.clone(),
        }
    }

    fn mailgun(&self, message: Message) -> Mailgun {
        Mailgun {
            api_key: self.api_key.clone(),
            domain: self.domain.clone(),
            message,
        }
    }

    pub async fn send_template(&self, instruction: SendInstruction) -> SendResult<SendResponse> {
        let recipient = EmailAddress::address(&instruction.to);

        let message = Message {
            to: vec![recipient],
            subject: instruction.subject.clone(),
            template: instruction.template_key.clone(),
            template_vars: instruction.vars.clone(),
            ..Default::default()
        };

        let client = self.mailgun(message);
        let from = EmailAddress::name_address("BCSS Game Jam", "no-reply@hack.bathcs.com");

        client.async_send(MailgunRegion::EU, &from).await
    }

    pub async fn fake_send_email() {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
