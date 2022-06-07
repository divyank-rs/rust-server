use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pizza {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price_inr: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Orders {
    pub id: i32,
    pub pizza_id: i32,
    pub mobile_no: Option<String>,
    pub remarks: Option<String>,
    pub price_inr: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
