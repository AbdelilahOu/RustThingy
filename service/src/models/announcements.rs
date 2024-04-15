use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
pub struct Announcement {
    pub title: String,
    pub description: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
    pub category: String,
    pub targets: Option<Vec<Uuid>>,
    pub attachements: Option<NaiveDateTime>,
    pub important: Option<bool>,
    pub audience: Option<String>,
    pub alert: Option<NaiveDateTime>,
}
