use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    id: Option<u16>,
    session_token: String,
    expires_at: i64,
    username: String,
}

impl Session {
    // pub fn get_id(&self) -> i32 {
    //     self.id.unwrap_or(0) as i32
    // }

    pub fn get_session_token(&self) -> &str {
        &self.session_token
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_expires_at(&self) -> i64 {
        self.expires_at
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewSession {
    pub session_token: String,
    // EpochTime
    pub expires_at: i64,
    pub username: String,
}

impl NewSession {
    fn prepare(self) -> Session {
        Session {
            id: None,
            session_token: self.session_token,
            expires_at: self.expires_at,
            username: self.username,
        }
    }
}

#[derive(Debug)]
pub struct State {
    sessions: RwLock<Vec<Session>>,
}

impl State {
    pub fn new() -> State {
        State {
            sessions: RwLock::new(Vec::new()),
        }
    }
}

pub struct Database;

impl Database {
    pub fn list_sessions(context: &State) -> Vec<Session> {
        let sessions = context.sessions.read().unwrap();
        sessions.iter().cloned().collect()
    }

    pub fn add_session(context: &State, new_user: NewSession) {
        let mut sessions = context.sessions.write().unwrap();
        let mut user = new_user.prepare();
        user.id = Some((sessions.len() + 1) as u16);
        sessions.push(user);
    }

    pub fn destroy_session(context: &State, session_id: String) {
        let sessions_read = context.sessions.read().unwrap().clone().to_vec();
        let mut sessions_write = context.sessions.write().unwrap();
        let rwlock = RwLock::new(Vec::new());
        let sessions_new = &mut rwlock.write().unwrap();
        for session in sessions_read {
            if session.session_token != session_id {
                sessions_new.push(session)
            }
        }
        *sessions_write = sessions_new.to_vec();
    }
}
