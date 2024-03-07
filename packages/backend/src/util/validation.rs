pub fn humanise_validation_error(errors: &bhw_types::validator::ValidationErrors) -> String {
    let hm = errors.field_errors();
    let mut message = String::default();
    for (key, field_errors) in hm {
        message.push_str(format!("{} is invalid: ", key).as_str());

        for field_error in field_errors {
            if let Some(field_message) = &field_error.message {
                message.push_str(&field_message)
            }
        }

        message.push_str(";");
    }

    message
}
