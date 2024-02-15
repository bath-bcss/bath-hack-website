use std::collections::HashMap;

use mailgun_rs::{EmailAddress, Mailgun, MailgunRegion, Message, SendResponse, SendResult};

use crate::app_config::AppConfig;

pub struct Mailer<'a> {
    pub api_key: &'a String,
    pub domain: &'a String,
}

pub struct SendInstruction<'a> {
    pub template_key: &'a String,
    pub to: &'a String,
    pub vars: &'a HashMap<String, String>,
    pub subject: &'a String,
}

impl<'a> Mailer<'a> {
    pub fn client(config: &'a AppConfig) -> Self {
        Mailer::<'a> {
            api_key: &config.mailgun_api_key,
            domain: &config.mailgun_domain,
        }
    }

    fn mailgun(&self, message: Message) -> Mailgun {
        Mailgun {
            api_key: self.api_key.clone(),
            domain: self.domain.clone(),
            message,
        }
    }

    pub fn send_template<'b>(&self, instruction: &'b SendInstruction) -> SendResult<SendResponse> {
        let recipient = EmailAddress::address(instruction.to);

        let message = Message {
            to: vec![recipient],
            subject: instruction.subject.clone(),
            template: instruction.template_key.clone(),
            template_vars: instruction.vars.clone(),
            ..Default::default()
        };

        let client = self.mailgun(message);
        let from = EmailAddress::name_address("BCSS Bath Hack", "no-reply@hack.bathcs.com");

        client.send(MailgunRegion::EU, &from)
    }
}
