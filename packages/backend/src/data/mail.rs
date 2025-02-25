use std::collections::HashMap;

use mailgun_rs::{EmailAddress, Mailgun, MailgunRegion, Message};

use crate::app_config::AppConfig;

pub struct Mailer {
    api_key: String,
    domain: String,
    skip: bool,
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
            skip: config.dev_skip_emails,
        }
    }

    fn mailgun(&self) -> Mailgun {
        Mailgun {
            api_key: self.api_key.clone(),
            domain: self.domain.clone(),
        }
    }

    pub async fn send_template(&self, instruction: SendInstruction) -> Result<(), reqwest::Error> {
        if self.skip {
            return Ok(());
        }

        let recipient = EmailAddress::address(&instruction.to);

        let message = Message {
            to: vec![recipient],
            subject: instruction.subject.clone(),
            template: instruction.template_key.clone(),
            template_vars: instruction.vars.clone(),
            ..Default::default()
        };

        let client = self.mailgun();
        let from = EmailAddress::name_address("WiTathon", "no-reply@hack.bathcs.com");

        client.async_send(MailgunRegion::EU, &from, message).await?;
        Ok(())
    }

    pub async fn fake_send_email() {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}
