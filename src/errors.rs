use std::fmt;

#[derive(Debug)]
pub struct ClientError {
    details: String,
    pub auth_ticket: String,
}

impl ClientError {
    pub fn new(msg: &str, ticket: String) -> ClientError {
        ClientError {
            details: msg.to_string(),
            auth_ticket: ticket,
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
