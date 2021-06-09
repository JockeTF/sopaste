use std::borrow::Cow;

use chrono::NaiveDate;
use chrono::NaiveTime;

use sqlx::query_as;
use sqlx::FromRow;
use sqlx::Result;
use sqlx::Type;

use crate::storage::Pool;

#[derive(Type)]
#[sqlx(transparent)]
pub struct Text(Option<Vec<u8>>);

impl Text {
    pub fn decode(&self) -> Cow<'_, str> {
        match &self.0 {
            Some(v) => String::from_utf8_lossy(v),
            None => Cow::from(""),
        }
    }
}

#[derive(FromRow)]
pub struct ListRow {
    pub id: Text,
    pub name: Text,
    pub description: Text,
    pub language: Text,
    pub start: i32,
    pub password: Text,
    pub ip: Text,
    pub proxy: Text,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub checked: Option<NaiveDate>,
    pub size: Option<f64>,
    pub removed: i8,
    pub parent: Text,
}

impl ListRow {
    pub async fn find(pool: &Pool, id: &str) -> Result<Self> {
        let sql = "
            SELECT
                list.*
            FROM
                list
            WHERE
                removed IS FALSE
                AND list.id = ?
        ";

        query_as(sql).bind(id).fetch_one(&**pool).await
    }
}

#[derive(FromRow)]
pub struct TextRow {
    pub id: Text,
    pub text: Text,
}

impl TextRow {
    pub async fn find(pool: &Pool, id: &str) -> Result<Self> {
        let sql = "
            SELECT
                text.*
            FROM
                list
                INNER JOIN text ON list.id = text.id
            WHERE
                removed IS FALSE
                AND list.id = ?
        ";

        query_as(sql).bind(id).fetch_one(&**pool).await
    }
}
