use std::sync::Mutex;
use std::collections::HashMap;
use models::session::Session;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use timer::Timer;
use chrono::duration::Duration as ChronoDuration;
use std::time::Duration as StdDuration;
use std::env;
use std::str::FromStr;
use std::ops::Add;

lazy_static! {
    static ref SESSIONS: Mutex<HashMap<String, Session>> = {
        let hashmap = HashMap::new();
        SessionManager::clean_sessions();
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

    pub fn clean_sessions() {
        fn do_clean() {
            let mut hashmap = SESSIONS.lock().unwrap();
            let mut to_remove = Vec::new();

            for (token, session) in hashmap.iter() {
                let session_timeout = env::var("SESSION_TIMEOUT").unwrap();
                let seconds = i64::from_str(&session_timeout).unwrap();

                let expire_date = session.opened_on.add(ChronoDuration::seconds(seconds));
                let current_date = UTC::now();

                if current_date.ge(&expire_date) {
                    to_remove.push(token.clone());
                }
            }

            for token in to_remove {
                hashmap.remove(&token);
                println!("Session {} cleaned", token);
            }
        }

        let session_cleanup = env::var("SESSION_CLEANUP").unwrap();
        let cleanup_seconds = u64::from_str(&session_cleanup).unwrap();
        let duration = StdDuration::new(cleanup_seconds, 0);

        let timer = Timer::new(duration, do_clean);
        timer.start_delayed(duration);
    }
}