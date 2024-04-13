use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct CSession {
    pub user_id: Uuid,
    pub user_agent: String,
    pub client_ip: String,
    pub is_blocked: bool,
    pub refresh_token: String,
    pub expires_at: NaiveDateTime,
}
