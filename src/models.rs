use chrono::NaiveDate;
use chrono::NaiveTime;

use sqlx::FromRow;

#[derive(FromRow)]
pub struct ListRow {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub start: i32,
    pub password: Option<String>,
    pub ip: Option<String>,
    pub proxy: Option<String>,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub checked: Option<NaiveDate>,
    pub size: Option<f64>,
    pub removed: i8,
    pub parent: Option<String>,
}

#[derive(FromRow)]
pub struct TextRow {
    pub id: String,
    pub text: Option<String>,
}
