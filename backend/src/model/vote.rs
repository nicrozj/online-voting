use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Vote {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl Vote {}
