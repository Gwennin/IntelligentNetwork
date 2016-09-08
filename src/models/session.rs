use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use uuid::Uuid;

#[derive(Debug)]
pub struct Session {
    pub username: String,
    pub opened_on: DateTime<UTC>,
    pub token: String,
}

impl Session {
    pub fn new(username: &String) -> Session {

        let token = Uuid::new_v4();
        Session {
            username: username.to_string(),
            opened_on: UTC::now(),
            token: token.simple().to_string()
        }
    }
}