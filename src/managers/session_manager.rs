use std::sync::Mutex;
use std::collections::HashMap;
use models::session::Session;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

lazy_static! {
    static ref SESSIONS: Mutex<HashMap<String, Session>> = {
        let hashmap = HashMap::new();
        Mutex::new(hashmap)
    };
}

pub struct SessionManager;

impl SessionManager {
    pub fn open_session(username: &String) -> String {
        let session = Session::new(username);
        let token = session.token.clone();

        let mut hashmap = SESSIONS.lock().unwrap();
        hashmap.insert(session.token.clone(), session);

        return token;
    }

    pub fn get_session_user(token: &String) -> Option<String> {
        let hashmap = SESSIONS.lock().unwrap();

        let res = hashmap.get(token);
        match res {
            Some(session) => Some(session.username.clone()),
            None => None,
        }
    }

    pub fn get_session_opened_date(token: &String) -> Option<DateTime<UTC>> {
        let hashmap = SESSIONS.lock().unwrap();

        let res = hashmap.get(token);
        match res {
            Some(session) => Some(session.opened_on.clone()),
            None => None,
        }
    }

    pub fn close_session(token: &String) {
        let mut hashmap = SESSIONS.lock().unwrap();

        hashmap.remove(token);
    }
}